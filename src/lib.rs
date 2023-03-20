mod bosses;
mod common;
mod database;
mod ge;
mod items;
mod level;
mod params;
mod patch;
mod players;
mod prices;
mod stats;
mod xp;

use regex::{Captures, Regex};

#[no_mangle]
pub extern "C" fn exported(
    mut command: &str,
    query: &str,
    author: &str,
) -> Result<Vec<String>, ()> {
    let re = Regex::new(r"^([a-zA-Z]+)(\d+)$").unwrap();
    let re_match = match re.captures(command) {
        Some(captures) => vec![captures],
        None => vec![],
    };

    let mut rsn_n = "0";

    if re_match.len() > 0 {
        command = re_match[0].get(1).unwrap().as_str();
        rsn_n = re_match[0].get(2).unwrap().as_str();
    }

    match command {
        "boss" => bosses::bosses(query, author, rsn_n),
        "experience" | "exp" | "xp" => xp::xp(query),
        "ge" => ge::ge(query),
        "level" | "lvl" => level::level(query),
        "param" | "params" => params::params(query),
        "patch" => patch::patch(query),
        "players" => players::players(),
        "price" => prices::prices(query),
        // "overall" | "stats" | "total" | "attack" | "att" | "defence" | "def" | "strength"
        // | "str" | "hitpoints" | "hp" | "ranged" | "range" | "prayer" | "pray" | "magic"
        // | "mage" | "cooking" | "cook" | "woodcutting" | "wc" | "fletching" | "fletch"
        // | "fishing" | "fish" | "firemaking" | "fm" | "crafting" | "craft" | "smithing"
        // | "smith" | "mining" | "mine" | "herblore" | "herb" | "agility" | "agil" | "thieving"
        // | "thief" | "slayer" | "slay" | "farming" | "farm" | "runecraft" | "rc" | "hunter"
        // | "hunt" | "construction" | "con" => stats::stats(command, query, author),
        "help" => Ok(r"boss[N]
ge
level
xp
params
players
price"
            //stats"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        "" => Ok(r"boss\d*
ge
level
lvl
params
patch
players
price
xp
exp
experience"
            // stats
            // overall
            // total
            // attack
            // att
            // defence
            // def
            // strength
            // str
            // hitpoints
            // hp
            // ranged
            // range
            // prayer
            // pray
            // magic
            // mage
            // cooking
            // cook
            // woodcutting
            // wc
            // fletching
            // fletch
            // fishing
            // fish
            // firemaking
            // fm
            // crafting
            // craft
            // smithing
            // smith
            // mining
            // mine
            // herblore
            // herb
            // agility
            // agil
            // thieving
            // thief
            // slayer
            // slay
            // farming
            // farm
            // runecraft
            // rc
            // hunter
            // hunt
            // construction
            // con"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        _ => Ok(vec![]),
    }
}
