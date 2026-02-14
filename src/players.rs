extern crate reqwest;
extern crate select;

use anyhow::Result;
use common::commas;
use common::source::Source;
use format_num::NumberFormat;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
struct TotalRsPlayers {
    accounts: f64,
}

pub fn lookup(s: &Source) -> Result<Vec<String>, ()> {
    let total_players = match get_rs3_players() {
        Ok(resp) => resp,
        Err(_) => return Err(()),
    };
    let osrs_players = match get_osrs_players() {
        Ok(resp) => resp,
        Err(_) => return Err(()),
    };

    // Jagex is ashamed of their RS3 player count. The RS3 site only shows the total number of players,
    // so we have to subtract the OSRS player count from the total to get the RS3 player count.
    let rs3_players = total_players - osrs_players;

    let total_registered = match get_total_players() {
        Ok(resp) => resp,
        Err(_) => return Err(()),
    };

    let num = NumberFormat::new();

    // There are currently 81,203 OSRS players (68.88%) and 36,687 RS3 players (31.12%) online. (Total: 117,890) (Total Registered Accounts: 296,907,582)
    let string = format!(
        "There are currently {} OSRS players ({}%) and {} RS3 players ({}%) online. (Total: {}) (Total Registered Accounts: {})",
        s.c2(commas(osrs_players, "d")),
        s.c2(commas(osrs_players / total_players * 100.0, ".2f")),
        s.c2(commas(rs3_players, ",d")),
        s.c2(commas(rs3_players / total_players * 100.0, ".2f")),
        s.c2(commas(total_players, ",d")),
        s.c2(commas(total_registered, ",d"))
    );

    let output: Vec<String> = vec![string];

    Ok(output)
}

fn get_rs3_players() -> Result<f64, ()> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Fetch this weird jQuery callback thing. Looks like this:
    // jQuery36006339226594951519_1645569829067(127551);
    let resp = match rt.block_on(reqwest::get("https://www.runescape.com/player_count.js?varname=iPlayerCount&callback=jQuery36006339226594951519_1645569829067&_=1645569829068")) {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error making HTTP request: {}", e);
        return Err(())
        },
    };

    let mut string = match rt.block_on(resp.text()) {
        Ok(string) => string,
        Err(e) => {
            println!("Error getting text: {}", e);
            return Err(());
        }
    };

    // Remove the last two characters
    string.pop();
    string.pop();

    // Remove the first two characters
    let string = match string.split("(").nth(1) {
        Some(string) => string,
        None => {
            println!("Error splitting string");
            return Err(());
        }
    };

    // Strip commas and convert to a float
    Ok(get_int(string))
}

fn get_osrs_players() -> Result<f64, ()> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Fetch the entire OSRS website to parse out the player count
    let resp = match rt.block_on(reqwest::get("https://oldschool.runescape.com")) {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error making HTTP request: {}", e);
            return Err(());
        }
    };

    let string = match rt.block_on(resp.text()) {
        Ok(string) => string,
        Err(e) => {
            println!("Error getting text: {}", e);
            return Err(());
        }
    };

    let re = match Regex::new(
        r"<p class='player-count'>There are currently ([\d,]+) people playing!</p>",
    ) {
        Ok(re) => re,
        Err(e) => {
            println!("Error creating regex: {}", e);
            return Err(());
        }
    };
    let matched = re.captures(&string);
    let string = matched.unwrap().get(1).unwrap().as_str();

    // Strip commas and convert to a float
    Ok(get_int(string))
}

fn get_total_players() -> Result<f64, ()> {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Fetch some JSON from the Runescape website
    let resp = match rt.block_on(reqwest::get(
        "https://secure.runescape.com/m=account-creation-reports/rsusertotal.ws",
    )) {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error making HTTP request: {}", e);
            return Err(());
        }
    };

    let totaljson: TotalRsPlayers = match rt.block_on(resp.json::<TotalRsPlayers>()) {
        Ok(totaljson) => totaljson,
        Err(e) => {
            println!("Error getting json: {}", e);
            return Err(());
        }
    };

    Ok(totaljson.accounts)
}

fn get_int(string: &str) -> f64 {
    // Strip commas and convert to a float
    string.replace(",", "").parse::<f64>().unwrap_or(0.0)
}
