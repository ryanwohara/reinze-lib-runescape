extern crate reqwest;
extern crate select;

use anyhow::{Context, Result};
use common::commas;
use common::source::Source;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use tokio::runtime::Runtime;

#[derive(Deserialize, Serialize, Debug)]
struct TotalRsPlayers {
    accounts: f64,
}

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let rt = Runtime::new().expect("failed to create tokio runtime");

    let total_players = get_rs3_players(&rt)?;
    let osrs_players = get_osrs_players(&rt)?;

    // Jagex is ashamed of their RS3 player count. The RS3 site only shows the total number of players,
    // so we have to subtract the OSRS player count from the total to get the RS3 player count.
    let rs3_players = total_players - osrs_players;

    let total_registered = get_total_players(&rt)?;

    // There are currently 81,203 OSRS players (68.88%) and 36,687 RS3 players (31.12%) online. (Total: 117,890) (Total Registered Accounts: 296,907,582)
    let string = format!(
        "There are currently {}\x03 OSRS players A({}\x03%) and {}\x03 RS3 players ({}\x03%) online. (Total: {}\x03) (Total Registered Accounts: {}\x03)",
        s.c2(commas(osrs_players, "d")),
        s.c2(commas(osrs_players / total_players * 100.0, ".2f")),
        s.c2(commas(rs3_players, "d")),
        s.c2(commas(rs3_players / total_players * 100.0, ".2f")),
        s.c2(commas(total_players, "d")),
        s.c2(commas(total_registered, "d"))
    );

    let output: Vec<String> = vec![string];

    Ok(output)
}

fn get_rs3_players(rt: &Runtime) -> Result<f64> {
    // Fetch this weird jQuery callback thing. Looks like this:
    // jQuery36006339226594951519_1645569829067(127551);
    let resp = rt.block_on(reqwest::get("https://www.runescape.com/player_count.js?varname=iPlayerCount&callback=jQuery36006339226594951519_1645569829067&_=1645569829068"))
        .context("HTTP request failed")?;

    let mut string = rt
        .block_on(resp.text())
        .context("failed to read response text")?;

    // Remove the last two characters
    string.pop();
    string.pop();

    // Remove the first two characters
    let string = string
        .split("(")
        .nth(1)
        .context("unexpected RS3 player count format")?;

    // Strip commas and convert to a float
    Ok(get_int(string))
}

fn get_osrs_players(rt: &Runtime) -> Result<f64> {
    // Fetch the entire OSRS website to parse out the player count
    let resp = rt
        .block_on(reqwest::get("https://oldschool.runescape.com"))
        .context("HTTP request failed")?;

    let string = rt
        .block_on(resp.text())
        .context("failed to read response text")?;

    let re =
        Regex::new(r"<p class='player-count'>There are currently ([\d,]+) people playing!</p>")
            .unwrap();

    let string = re
        .captures(&string)
        .context("player count not found in OSRS page")?
        .get(1)
        .context("capture group 1 missing")?
        .as_str();

    // Strip commas and convert to a float
    Ok(get_int(string))
}

fn get_total_players(rt: &Runtime) -> Result<f64> {
    // Fetch some JSON from the Runescape website
    let resp = rt
        .block_on(reqwest::get(
            "https://secure.runescape.com/m=account-creation-reports/rsusertotal.ws",
        ))
        .context("HTTP request failed")?;

    let totaljson: TotalRsPlayers = rt
        .block_on(resp.json::<TotalRsPlayers>())
        .context("failed to parse JSON response")?;

    Ok(totaljson.accounts)
}

fn get_int(string: &str) -> f64 {
    // Strip commas and convert to a float
    string.replace(",", "").parse::<f64>().unwrap_or(0.0)
}
