mod bh;
mod bosses;
mod clues;
mod common;
mod database;
mod ge;
mod items;
mod level;
mod lms;
mod params;
mod patch;
mod players;
mod prices;
mod stats;
mod wiki;
mod xp;

use regex::Regex;

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
        "bh" | "bounty" | "bhunter" | "bountyhunter" => bh::lookup(query, author, rsn_n),
        "boss" | "bosses" => bosses::lookup(query, author, rsn_n),
        "clue" | "clues" => clues::lookup(query, author, rsn_n),
        "experience" | "xperience" | "exp" | "xp" => xp::xp(query),
        "ge" => ge::ge(query),
        "level" | "lvl" => level::level(query),
        "lms" | "lmstanding" | "lmanstanding" | "lastmstanding" | "lastmanstanding" => {
            lms::lookup(query, author, rsn_n)
        }
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
        "wiki" => wiki::wiki(query),
        "help" => Ok(r"bh[N]
boss[N]
clues[N]
ge
level
xp
params
players
price
wiki"
            //stats"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        "" => Ok(r"b(ounty)?h(unter)?\d*
boss\d*
clues?\d*
ge
l(ast)?m(an)?s(tanding)?
level
lvl
params
patch
players
price
e?xp(erience)?
wiki"
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
