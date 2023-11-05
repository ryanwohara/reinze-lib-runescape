use crate::database;
use format_num::NumberFormat;
use mysql::{prelude::*, *};

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f
            .to_uppercase()
            .chain(c.flat_map(|c| c.to_lowercase()))
            .collect(),
    }
}

// Gray
// c1
pub fn c1(s: &str) -> String {
    format!("\x0314{}", s)
}

// Red
// c2
pub fn c2(s: &str) -> String {
    format!("\x0304{}", s)
}

// Red
// c3
pub fn c3(s: &str) -> String {
    format!("\x0305{}", s)
}

// Green
// c4
pub fn c4(s: &str) -> String {
    format!("\x0303{}", s)
}

// Yellow
// c5
pub fn c5(s: &str) -> String {
    format!("\x0307{}", s)
}

// A function for wrapping a string in brackets that are colored gray
// l
pub fn l(s: &str) -> String {
    format!("{}{}{}", c1("["), c2(s), c1("]"))
}

// A function for wrapping a string in parenthesis that are colored gray
// p
pub fn p(s: &str) -> String {
    format!("{}{}{}", c1("("), c2(s), c1(")"))
}

// Converts a level to experience
pub fn level_to_xp(level: u32) -> u32 {
    let mut xp = 0;

    for i in 1..level {
        let x = i as f32;

        xp += (x + 300.0 * 2.0_f32.powf(x / 7.0)).floor() as u32 / 4;
    }

    xp
}

// Converts experience to a level
pub fn xp_to_level(xp: u32) -> u32 {
    for level in 1..=127 {
        if xp < level_to_xp(level) {
            return level - 1;
        }
    }

    0
}

// Adds commas to a number
pub fn commas(n: f64) -> String {
    let num = NumberFormat::new();

    num.format(",d", n)
}

// Adds commas to a string
pub fn commas_from_string(n: &str) -> String {
    let n = match n.parse::<f64>() {
        Ok(n) => n,
        Err(_) => 0.0,
    };

    commas(n)
}

// Catches shorthand skill names and returns the full name
pub fn skill(s: &str) -> String {
    match s.to_lowercase().as_str() {
        "overall" | "stats" | "total" => "Overall".to_string(),
        "attack" | "att" => "Attack".to_string(),
        "defence" | "def" => "Defence".to_string(),
        "strength" | "str" => "Strength".to_string(),
        "hitpoints" | "hp" => "Hitpoints".to_string(),
        "ranged" | "range" => "Ranged".to_string(),
        "prayer" | "pray" => "Prayer".to_string(),
        "magic" | "mage" => "Magic".to_string(),
        "cooking" | "cook" => "Cooking".to_string(),
        "woodcutting" | "wc" => "Woodcutting".to_string(),
        "fletching" | "fletch" => "Fletching".to_string(),
        "fishing" | "fish" => "Fishing".to_string(),
        "firemaking" | "fm" => "Firemaking".to_string(),
        "crafting" | "craft" => "Crafting".to_string(),
        "smithing" | "smith" => "Smithing".to_string(),
        "mining" | "mine" => "Mining".to_string(),
        "herblore" | "herb" => "Herblore".to_string(),
        "agility" | "agil" => "Agility".to_string(),
        "thieving" | "thief" => "Thieving".to_string(),
        "slayer" | "slay" => "Slayer".to_string(),
        "farming" | "farm" => "Farming".to_string(),
        "runecraft" | "rc" => "Runecraft".to_string(),
        "hunter" | "hunt" => "Hunter".to_string(),
        "construction" | "con" => "Construction".to_string(),
        _ => String::new(),
    }
}

// Returns a vector of all skills
pub fn skills() -> Vec<String> {
    vec![
        "Overall".to_string(),
        "Attack".to_string(),
        "Defence".to_string(),
        "Strength".to_string(),
        "Hitpoints".to_string(),
        "Ranged".to_string(),
        "Prayer".to_string(),
        "Magic".to_string(),
        "Cooking".to_string(),
        "Woodcutting".to_string(),
        "Fletching".to_string(),
        "Fishing".to_string(),
        "Firemaking".to_string(),
        "Crafting".to_string(),
        "Smithing".to_string(),
        "Mining".to_string(),
        "Herblore".to_string(),
        "Agility".to_string(),
        "Thieving".to_string(),
        "Slayer".to_string(),
        "Farming".to_string(),
        "Runecraft".to_string(),
        "Hunter".to_string(),
        "Construction".to_string(),
    ]
}

pub fn get_rsn(author: &str, rsn_n: &str) -> core::result::Result<Vec<mysql::Row>, mysql::Error> {
    let mut conn = match database::connect() {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error connecting to database: {}", e);
            return Err(e);
        }
    };

    let mut host = author.split("!").collect::<Vec<&str>>()[1];
    if host.starts_with("~") {
        host = host.split("~").collect::<Vec<&str>>()[1];
    }

    match conn.exec_first(
        "SELECT rsn FROM rsn WHERE host = :host AND rsn_ident = :rsn_n",
        params! { host, rsn_n },
    ) {
        Ok(Some(rsn)) => Ok(vec![rsn]),
        Ok(None) => Ok(vec![]),
        Err(e) => {
            println!("Error getting rsn: {}", e);
            Err(e)
        }
    }
}

pub fn get_stats(rsn: &str) -> core::result::Result<Vec<Vec<String>>, ()> {
    let mut stats = Vec::new();

    let body = match query_stats(rsn) {
        Ok(body) => body,
        Err(_) => return Err(()),
    };

    for line in body.lines() {
        let split = line
            .split(",")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        stats.push(split);
    }

    Ok(stats)
}

fn query_stats(rsn: &str) -> core::result::Result<String, ()> {
    let url = format!(
        "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player={}",
        rsn
    );

    let resp = match reqwest::blocking::get(&url) {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error getting stats: {}", e);
            return Err(());
        }
    };

    let body = match resp.text() {
        Ok(body) => body.to_owned(),
        Err(e) => {
            println!("Error getting stats: {}", e);
            return Err(());
        }
    };

    Ok(body)
}
