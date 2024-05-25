use common::{database, *};
use mysql::{prelude::*, *};
use regex::Regex;

// Catches shorthand skill names and returns the full name
pub fn skill(s: &str) -> String {
    match s.to_lowercase().as_str() {
        "overall" | "stats" | "total" | "combat" | "cmb" => "Overall".to_string(),
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

#[derive(Debug, Clone)]
pub struct Combat {
    pub level: f64,
    pub style: String,
}

impl Combat {
    pub fn new(level: f64, style: String) -> Combat {
        Combat {
            level: level,
            style: style,
        }
    }
}

pub fn get_cmb(
    att: &u32,
    str: &u32,
    def: &u32,
    hp: &u32,
    range: &u32,
    pray: &u32,
    mage: &u32,
) -> Combat {
    let base = ((def + hp) + (pray / 2)) as f64 * 0.25;

    let melee = 0.325 * (att + str) as f64;
    let ranged = 0.325 * ((range.to_owned() / 2) as f64 + range.to_owned() as f64);
    let magic = 0.325 * ((mage.to_owned() / 2) as f64 + mage.to_owned() as f64);

    let max_contribution = f64::max(melee, f64::max(ranged, magic));
    let level = f64::round((base + max_contribution) * 1000.0) / 1000.0;

    if melee > ranged && melee > magic {
        return Combat::new(level, "Melee".to_string());
    } else if ranged > melee && ranged > magic {
        return Combat::new(level, "Ranged".to_string());
    } else {
        // if magic > melee && magic > ranged
        return Combat::new(level, "Magic".to_string());
    }
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

fn query_stats(rsn: &str, endpoint: &str) -> core::result::Result<String, ()> {
    let url = format!("{}{}", endpoint, rsn);

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

pub fn get_stats(rsn: &str, endpoint: &str) -> core::result::Result<Vec<Vec<String>>, ()> {
    let mut stats = Vec::new();

    let body = match query_stats(rsn, endpoint) {
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

pub fn process_account_type_flags(
    query: &str,
    split: Vec<String>,
) -> (Vec<String>, String, String) {
    let re_ser = Regex::new(r"(?:^|\b|\s)-([iuhdlt1]|sk|fs)(?:\s|\b|$)").unwrap();
    let nil = (
        split.to_owned(),
        "".to_owned(),
        "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=".to_owned(),
    );

    let (mut output, flag) = re_ser
        .captures(query)
        .map(|capture| {
            let flag = capture.get(1).map_or("", |flag| flag.as_str());
            (
                match flag {
                    "i" => (
                        split,
                        l("Iron"),
                        "https://secure.runescape.com/m=hiscore_oldschool_ironman/index_lite.ws?player=".to_owned(),
                    ),
                    "u" => (
                        split,
                        l("Ultimate"),
                        "https://secure.runescape.com/m=hiscore_oldschool_ultimate/index_lite.ws?player=".to_owned(),
                    ),
                    "h" => (
                        split,
                        l("Hardcore"),
                        "https://secure.runescape.com/m=hiscore_oldschool_hardcore_ironman/index_lite.ws?player=".to_owned(),
                    ),
                    "d" => (
                        split,
                        l("Deadman"),
                        "https://secure.runescape.com/m=hiscore_oldschool_deadman/index_lite.ws?player=".to_owned(),
                    ),
                    "l" => (
                        split,
                        l("Leagues"),
                        "https://secure.runescape.com/m=hiscore_oldschool_seasonal/index_lite.ws?player=".to_owned(),
                    ),
                    "t" => (
                        split,
                        l("Tournament"),
                        "https://secure.runescape.com/m=hiscore_oldschool_tournament/index_lite.ws?player=".to_owned(),
                    ),
                    "1" => (
                        split,
                        l("1 Def"),
                        "https://secure.runescape.com/m=hiscore_oldschool_skiller_defence/index_lite.ws?player=".to_owned(),
                    ),
                    "sk" => (
                        split,
                        l("Skiller"),
                        "https://secure.runescape.com/m=hiscore_oldschool_skiller/index_lite.ws?player=".to_owned(),
                    ),
                    "fs" => (
                        split,
                        l("Fresh Start"),
                        "https://secure.runescape.com/m=hiscore_oldschool_fresh_start/index_lite.ws?player=".to_owned(),
                    ),
                    _ => nil.to_owned(),
                },
                flag,
            )
        })
        .unwrap_or_else(|| (nil, ""));

    if !flag.is_empty() {
        output.0.retain(|arg| arg != &format!("-{}", flag));
    }

    output
}

#[cfg(test)]
mod tests {
    // import names from outer (for mod tests) scope
    use super::*;
    #[test]
    fn test_skill() {
        assert_eq!(skill("overall"), "Overall");
        assert_eq!(skill("stats"), "Overall");
        assert_eq!(skill("total"), "Overall");
        assert_eq!(skill("attack"), "Attack");
        assert_eq!(skill("att"), "Attack");
        assert_eq!(skill("defence"), "Defence");
        assert_eq!(skill("def"), "Defence");
        assert_eq!(skill("strength"), "Strength");
        assert_eq!(skill("str"), "Strength");
        assert_eq!(skill("hitpoints"), "Hitpoints");
        assert_eq!(skill("hp"), "Hitpoints");
        assert_eq!(skill("ranged"), "Ranged");
        assert_eq!(skill("range"), "Ranged");
        assert_eq!(skill("prayer"), "Prayer");
        assert_eq!(skill("pray"), "Prayer");
        assert_eq!(skill("magic"), "Magic");
        assert_eq!(skill("mage"), "Magic");
        assert_eq!(skill("cooking"), "Cooking");
        assert_eq!(skill("cook"), "Cooking");
        assert_eq!(skill("woodcutting"), "Woodcutting");
        assert_eq!(skill("wc"), "Woodcutting");
        assert_eq!(skill("fletching"), "Fletching");
        assert_eq!(skill("fletch"), "Fletching");
        assert_eq!(skill("fishing"), "Fishing");
        assert_eq!(skill("fish"), "Fishing");
        assert_eq!(skill("firemaking"), "Firemaking");
        assert_eq!(skill("fm"), "Firemaking");
        assert_eq!(skill("crafting"), "Crafting");
        assert_eq!(skill("craft"), "Crafting");
        assert_eq!(skill("smithing"), "Smithing");
        assert_eq!(skill("smith"), "Smithing");
        assert_eq!(skill("mining"), "Mining");
        assert_eq!(skill("mine"), "Mining");
        assert_eq!(skill("herblore"), "Herblore");
        assert_eq!(skill("herb"), "Herblore");
        assert_eq!(skill("agility"), "Agility");
        assert_eq!(skill("agil"), "Agility");
        assert_eq!(skill("thieving"), "Thieving");
        assert_eq!(skill("thief"), "Thieving");
        assert_eq!(skill("slayer"), "Slayer");
        assert_eq!(skill("slay"), "Slayer");
        assert_eq!(skill("farming"), "Farming");
        assert_eq!(skill("farm"), "Farming");
        assert_eq!(skill("runecraft"), "Runecraft");
        assert_eq!(skill("rc"), "Runecraft");
        assert_eq!(skill("hunter"), "Hunter");
        assert_eq!(skill("hunt"), "Hunter");
        assert_eq!(skill("construction"), "Construction");
        assert_eq!(skill("con"), "Construction");
        assert_eq!(skill("invalid"), "");
    }

    #[test]
    fn test_skills() {
        assert_eq!(skills().len(), 24,);
        assert_eq!(
            skills(),
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
        );
    }

    #[test]
    fn test_level_to_xp() {
        assert_eq!(level_to_xp(1), 0);
        assert_eq!(level_to_xp(2), 83);
        assert_eq!(level_to_xp(3), 174);
        assert_eq!(level_to_xp(4), 275);
        assert_eq!(level_to_xp(5), 387);
        assert_eq!(level_to_xp(6), 511);
        assert_eq!(level_to_xp(7), 648);
        assert_eq!(level_to_xp(8), 799);
        assert_eq!(level_to_xp(9), 966);
        assert_eq!(level_to_xp(10), 1151);
        assert_eq!(level_to_xp(11), 1355);
        assert_eq!(level_to_xp(12), 1580);
        assert_eq!(level_to_xp(13), 1829);
        assert_eq!(level_to_xp(14), 2103);
        assert_eq!(level_to_xp(15), 2406);
        assert_eq!(level_to_xp(16), 2740);
        assert_eq!(level_to_xp(17), 3109);
        assert_eq!(level_to_xp(18), 3517);
        assert_eq!(level_to_xp(19), 3967);
        assert_eq!(level_to_xp(20), 4463);
        assert_eq!(level_to_xp(21), 5011);
        assert_eq!(level_to_xp(22), 5616);
        assert_eq!(level_to_xp(23), 6283);
        assert_eq!(level_to_xp(24), 7020);
        assert_eq!(level_to_xp(25), 7833);
        assert_eq!(level_to_xp(26), 8730);
        assert_eq!(level_to_xp(27), 9720);
        assert_eq!(level_to_xp(28), 10813);
        assert_eq!(level_to_xp(29), 12020);
        assert_eq!(level_to_xp(30), 13352);
        assert_eq!(level_to_xp(45), 61495);
        assert_eq!(level_to_xp(55), 166614);
        assert_eq!(level_to_xp(75), 1210391);
        assert_eq!(level_to_xp(92), 6517217);
        assert_eq!(level_to_xp(95), 8771521);
        assert_eq!(level_to_xp(96), 9684539);
        assert_eq!(level_to_xp(97), 10692591);
        assert_eq!(level_to_xp(98), 11805568);
        assert_eq!(level_to_xp(99), 13034392);
    }

    #[test]
    fn test_xp_to_level() {
        assert_eq!(xp_to_level(0), 1);
        assert_eq!(xp_to_level(83), 2);
        assert_eq!(xp_to_level(174), 3);
        assert_eq!(xp_to_level(275), 4);
        assert_eq!(xp_to_level(387), 5);
        assert_eq!(xp_to_level(511), 6);
        assert_eq!(xp_to_level(648), 7);
        assert_eq!(xp_to_level(799), 8);
        assert_eq!(xp_to_level(966), 9);
        assert_eq!(xp_to_level(1151), 10);
        assert_eq!(xp_to_level(1355), 11);
        assert_eq!(xp_to_level(1580), 12);
        assert_eq!(xp_to_level(1829), 13);
        assert_eq!(xp_to_level(2103), 14);
        assert_eq!(xp_to_level(2406), 15);
        assert_eq!(xp_to_level(2740), 16);
        assert_eq!(xp_to_level(3109), 17);
        assert_eq!(xp_to_level(3517), 18);
        assert_eq!(xp_to_level(3967), 19);
        assert_eq!(xp_to_level(4463), 20);
        assert_eq!(xp_to_level(5011), 21);
        assert_eq!(xp_to_level(5616), 22);
        assert_eq!(xp_to_level(6283), 23);
        assert_eq!(xp_to_level(7020), 24);
        assert_eq!(xp_to_level(7833), 25);
        assert_eq!(xp_to_level(8730), 26);
        assert_eq!(xp_to_level(9720), 27);
        assert_eq!(xp_to_level(10813), 28);
        assert_eq!(xp_to_level(12020), 29);
        assert_eq!(xp_to_level(13352), 30);
        assert_eq!(xp_to_level(61495), 45);
        assert_eq!(xp_to_level(166614), 55);
        assert_eq!(xp_to_level(1210391), 75);
        assert_eq!(xp_to_level(6517217), 92);
        assert_eq!(xp_to_level(8771521), 95);
        assert_eq!(xp_to_level(9684539), 96);
        assert_eq!(xp_to_level(10692591), 97);
        assert_eq!(xp_to_level(11805568), 98);
        assert_eq!(xp_to_level(13034392), 99);
    }
}
