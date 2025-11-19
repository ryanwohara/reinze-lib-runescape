use crate::items::Mapping;
use common::{database, *};
use itertools::Itertools;
use meval::eval_str;
use mysql::{prelude::*, *};
use regex::Regex;
use std::fs::read_to_string;

// Catches shorthand skill names and returns the full name
pub fn skill(s: &str) -> String {
    match s.to_lowercase().as_str() {
        "overall" | "stats" | "total" | "combat" | "cmb" => "Overall",
        "attack" | "att" => "Attack",
        "defence" | "def" => "Defence",
        "strength" | "str" => "Strength",
        "hitpoints" | "hp" => "Hitpoints",
        "ranged" | "range" => "Ranged",
        "prayer" | "pray" => "Prayer",
        "magic" | "mage" => "Magic",
        "cooking" | "cook" => "Cooking",
        "woodcutting" | "wc" => "Woodcutting",
        "fletching" | "fletch" => "Fletching",
        "fishing" | "fish" => "Fishing",
        "firemaking" | "fm" => "Firemaking",
        "crafting" | "craft" => "Crafting",
        "smithing" | "smith" => "Smithing",
        "mining" | "mine" => "Mining",
        "herblore" | "herb" => "Herblore",
        "agility" | "agil" => "Agility",
        "thieving" | "thief" => "Thieving",
        "slayer" | "slay" => "Slayer",
        "farming" | "farm" => "Farming",
        "runecraft" | "rc" => "Runecraft",
        "hunter" | "hunt" => "Hunter",
        "construction" | "con" => "Construction",
        "sail" | "sailing" => "Sailing",
        _ => "",
    }
    .to_string()
}

// Returns a vector of all skills
pub fn skills() -> Vec<String> {
    vec![
        "Overall",
        "Attack",
        "Defence",
        "Strength",
        "Hitpoints",
        "Ranged",
        "Prayer",
        "Magic",
        "Cooking",
        "Woodcutting",
        "Fletching",
        "Fishing",
        "Firemaking",
        "Crafting",
        "Smithing",
        "Mining",
        "Herblore",
        "Agility",
        "Thieving",
        "Slayer",
        "Farming",
        "Runecraft",
        "Hunter",
        "Construction",
        "Sailing",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}

pub fn skill_id<T>(skill: T) -> u32
where
    T: ToString,
{
    skills()
        .iter()
        .position(|s| s.to_string() == skill.to_string())
        .unwrap_or(0) as u32
}

pub fn skill_by_id(skill: u32) -> String {
    let mut s = skills();

    s.retain(|x| skill == skill_id(x));

    s.pop().unwrap_or("Overall".to_string())
}

// Converts a level to experience
pub fn level_to_xp(level: u32) -> u32 {
    let mut xp = 0.0;

    for i in 1..level {
        let x: f32 = i as f32;

        xp += (x + 300.0 * 2.0_f32.powf(x / 7.0)).floor() / 4.0;
    }

    xp.floor() as u32
        + match level {
            96..=99 => 1,
            105..110 => 2,
            110..115 => 5,
            115..120 => 3,
            120..126 => 7,
            126 => 4,
            _ => 0,
        }
}

// Converts experience to a level
pub fn xp_to_level(xp: u32) -> u32 {
    for level in 2..=127 {
        if xp < level_to_xp(level) {
            return level - 1;
        }
    }

    126
}

#[derive(Debug, Clone)]
pub struct Combat {
    pub level: f64,
    pub style: String,
}

impl Combat {
    pub fn new(level: f64, style: String) -> Combat {
        Combat { level, style }
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
        Combat::new(level, "Melee".to_string())
    } else if ranged > melee && ranged > magic {
        Combat::new(level, "Ranged".to_string())
    } else {
        // if magic > melee && magic > ranged
        Combat::new(level, "Magic".to_string())
    }
}

pub fn get_rsn(author: &str, rsn_n: &str) -> core::result::Result<Vec<Row>, Error> {
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

pub fn get_item_db() -> Result<Vec<Mapping>, ()> {
    let mapping_filename = "lib/item_db.json";

    let mapping_file_contents = match read_to_string(mapping_filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening item_db.json: {}", e);
            return Err(());
        }
    };

    match serde_json::from_str::<Vec<Mapping>>(&mapping_file_contents) {
        Ok(json) => Ok(json),
        Err(e) => {
            println!("Error parsing item_db.json into JSON: {}", e);
            Err(())
        }
    }
}

pub fn parse_item_db(overall_query: &str) -> Result<Vec<Mapping>, ()> {
    let mut found_items: Vec<Mapping> = vec![];

    let item_db = match get_item_db() {
        Ok(item_db) => item_db,
        Err(_) => return Err(()),
    };

    for query in overall_query
        .split(",")
        .into_iter()
        .map(|index| index.trim())
    {
        if query.len() == 0 {
            continue;
        }

        let (query, count) = parse_query(query);
        let regex_string = format!(r"(?i){}", replace_item_abbreviations(&query));
        let re = match Regex::new(&regex_string) {
            Ok(re) => re,
            Err(e) => {
                println!("Error creating regex: {}", e);
                return Err(());
            }
        };

        for item in item_db.iter() {
            let matched = re.captures(&item.name);
            if matched.is_some() {
                let mut item_with_total = item.to_owned();
                item_with_total.total = Some(count);

                found_items.push(item_with_total);
            }
        }
    }

    Ok(found_items
        .iter()
        .unique_by(|i| i.id)
        .map(|i| i.to_owned())
        .collect())
}

pub fn eval_query<T>(q: T) -> std::result::Result<f64, ()>
where
    T: ToString,
{
    let query = q.to_string();

    let re_kmb = Regex::new(r"(?P<num>[\d.]+)(?P<kmb>[kmb])").unwrap();
    let processed = re_kmb.replace_all(&query, replace_all).to_string();

    eval_str(&processed).map_err(|e| {
        println!("Error: {}", e);
        ()
    })
}

pub fn replace_all(caps: &regex::Captures) -> String {
    let (num, kmb) = (
        caps.name("num").unwrap().as_str(),
        caps.name("kmb").unwrap().as_str(),
    );
    let mut num = num.parse::<f64>().unwrap_or_default();

    if let Some(factor) = match kmb {
        "k" => Some(1_000.0),
        "m" => Some(1_000_000.0),
        "b" => Some(1_000_000_000.0),
        _ => None,
    } {
        num *= factor;
    }
    num.to_string()
}

pub fn parse_query(query: &str) -> (String, u64) {
    let err = (query.to_string(), 0);

    let re = match Regex::new(r"^(\d+[kmb]?)\s+(.+)$") {
        Ok(x) => x,
        Err(_) => return err,
    };

    let matched = match re.captures(query) {
        Some(captures) => captures,
        None => return err,
    };

    let count = match matched.get(1) {
        Some(c) => match eval_query(c.as_str()) {
            Ok(r) => r as u64,
            Err(_) => 0,
        },
        None => return err,
    };

    let name = match matched.get(2) {
        Some(n) => n.as_str().to_string(),
        None => return err,
    };

    (name, count)
}

pub fn replace_item_abbreviations(q: &str) -> String {
    let mut query = q.to_string();

    let patterns = [
        (r"^sgs$", "Saradomin godsword"),
        (r"^ags$", "Armadyl godsword"),
        (r"^bgs$", "Bandos godsword"),
        (r"^zgs$", "Zamorak godsword"),
        (r"^a(nc?)gs", "Ancient godsword"),
        (r"^dfs$", "Dragonfire shield"),
        (r"^dd$", "Dragon dagger"),
        (r"^ddp$", "Dragon dagger\\(p\\)"),
        (r"^dds$", "Dragon dagger\\(p\\+\\+\\)"),
        (r"^bcp$", "Bandos chestplate"),
        (r"\btass(y|ies?)?\b", "tassets"),
        (r"^acp$", "Armadyl chestplate"),
        (r"^acb$", "Armadyl crossbow"),
        (r"^acs$", "Armadyl chainskirt"),
        (r"^ac$", "Armadyl c"),
        (r"^sot?d$", "Staff of the dead"),
        (r"^z ?s(pear)?$", "Zamorakian spear"),
        (r"^t?bp$", "blowpipe"),
        (r"^ss$", "Saradomin sword"),
        (r"^b ?ring$", "Berserker ring"),
        (r"^tbow", "Twisted bow"),
        (r"^d ?bow$", "Dark bow"),
        (r"^ely$", "Elysian"),
        (r"^bstaff$", "Battlestaff"),
        (r"^scp$", "Super combat potion"),
        (r"^d.?leather$", "Dragon leather"),
        (r"^d.?pick(axe)?$", "Dragon pickaxe"),
        (r"^d.?axe$", "Dragon axe"),
        (r"^dfh$", "Dragon full helm"),
        (r"^ore$", " ore"),
        (r"^dwh?$", "dragon warhammer"),
        (
            r"bolt(y|age)$",
            "(amylase|runite ore|air orb|^battlestaff|torstol$|Super combat potion(4))",
        ),
        (r"\b3a\b", "3rd age"),
        (r"\babby\b", "abyssal"),
        (r"\baddy\b", "adamant"),
        (r"eternal ammy", "amulet of eternal glory"),
        (r"\bammy\b", "amulet"),
        (r"\banc(es|y)?\b", "ancestral"),
        (r"\bang\b", "anguish"),
        (r"^aotd$", "amulet of the damned"),
        (r"\barc\b", "arcane"),
        (r"^aws$", "ancient wyvern shield"),
        (r"^b2b$", "bones to bananas"),
        (r"^b2p$", "bones to peaches"),
        (r"\bbally?\b", "ballista"),
        (r"\bbaxe\b", "battleaxe"),
        (r"\bbrassy\b", "brassard"),
        (r"^brews$", " brew\\(4\\)"),
        (r"^broads$", "broad-tipped"),
        (r"\bzerk(er)\b", "berserker"),
        (r"^bs$", "bow strings"),
        (r"^bss$", "blessed spirit shield"),
        (r"\bb ?staff\b", "battlestaff"),
        (r"\bcp\b", "chestplate"),
        (r"^cav$", "cavalier"),
        (r"^cball$", "cannonball"),
        (r"\bcbow\b", "crossbow"),
        (r"\bchain\b", "chainbody"),
        (r"^d2h$", "dragon 2h sword"),
        (r"^d ?bolts?$", "^dragon bolts$"),
        (r"^d ?darts?$", "^dragon darts$"),
        (r"^d ?bones?$", "dragon bones"),
        (r"^d ?bows?$", "dark bow"),
        (r"^d ?cb$", "dragon crossbow"),
        (r"^d ?chain$", "dragon chainbody"),
        (r"^d ?claws?$", "dragon claws"),
        (r"\bdex\b", "dexterous"),
        (r"^dfh$", "dragon full helm"),
        (r"^dfs$", "^dragonfire shield$"),
        (r"^dfw$", "dragonfire ward"),
        (r"^dh$", "dharok's.*[a-zA-Z]$"),
        (r"\bdh\b", "dharok's"),
        (r"^dhcb?$", "dragon hunter crossbow"),
        (r"^dhl$", "dragon hunter lance"),
        (r"\bdhide\b", "d'hide"),
        (r"^d ?legs?$", "dragon platelegs"),
        (r"^d ?med", "dragon med helm"),
        (r"^d ?plate$", "dragon platebody"),
        (r"\bscim\b", "scimitar"),
        (r"^ess$", "(rune|pure) essence"),
        (r"^bowfa$", "enhanced crystal weapon"),
        (r"^salad robes?$", "xerician"),
        (r"^salad( ?blade)?$", "enhanced crystal weapon"),
        (r"\bfg\b", "faceguard"),
        (r"^g ?spear$", "guthan's warspear"),
        (r"\bg\b", "(granite|guthan's)"),
        (r"\bguth\b", "guthan's"),
        (r"\bharmo?\b", "harmonised orb"),
        (r"^hb$", "heavy ballista"),
        (r"^lb$", "light ballista"),
        (r"\binq(is)?\b", "inquisitor"),
        (r"^jordans?$", "primordial boots"),
        (r"\bjusti\b", "justiciar"),
        (r"\bk\b", "karil's"),
        (r"\bkaram\b", "karambwan"),
        (r"^k ?([cx]b|bow)$", "karil's crossbow"),
        (r"^(vw|korasi)$", "voidwaker"),
        (r"^kskirt$", "karil's leatherskirt"),
        (r"^ktop$", "karil's leathertop"),
        (r"^lb$", "lightbearer"),
        (r"^lb[ab]a?$", "leaf-bladed battleaxe"),
        (r"^lbs$", "leaf-bladed sword"),
        (r"\blegs\b", "platelegs"),
        (r"^lobby$", "lobster"),
        (r"\blumby\b", "lumbridge"),
        (r"\bardy\b", "ardougne"),
        (r"^muds$", "mud rune"),
        (r"^nats$", "nature rune"),
        (r"^fires$", "fire rune"),
        (r"^airs$", "air rune"),
        (r"^waters$", "water rune"),
        (r"^ob(sidian|by) ?maul$", "tzhaar-ket-om"),
        (r"\bocc\b", "occult"),
        (r"^pegs$", "pegasian boots"),
        (r"\bphat\b", "partyhat"),
        (r"\bpick\b", "pickaxe"),
        (r"^p ?neck$", "phoenix necklace"),
        (r"\bpots\b", "potion"),
        (r"^p ?pot$", "prayer potion"),
        (r"^prims?$", "primordial boots"),
        (r"^prossy$", "proselyte"),
        (
            r"^mega( ?rare)?$",
            "twisted bow|tumeken's shadow|scythe of vitur",
        ),
        (r"^r2h$", "rune 2h sword"),
        (r"^rangers$", "ranger boots"),
        (r"^rcb$", "rune crossbow"),
        (r"^rfh$", "rune full helm"),
        (r"^rm(h|ed)$", "rune med helm"),
        (r"^rol$", "ring of life"),
        (r"^ros$", "ring of suffering"),
        (r"^rotg$", "ring of the gods"),
        (r"^row$", "ring of wealth"),
        (r"^rpb$", "rune platebody"),
        (r"^sang$", "sanguinesti staff"),
        (r"^sc[bp]$", "super combat"),
        (r"^sol$", "staff of light"),
        (r"^sotd$", "staff of the dead"),
        (r"^swordy$", "swordfish"),
        (r"^tsot?d$", "staff of the dead|magic fang"),
        (r"^n ?hally$", "noxious halberd"),
        (r"^v ?bow$", "venator bow"),
        (r"\bwh\b", "warhammer"),
        (r"\bxbow\b", "crossbow"),
        (r"^z(bow|cb)$", "zaryte crossbow"),
        (r"^zenny$", "zenyte"),
        (r"^zspear$", "zamorakian spear"),
        (r"^zhasta$", "zamorakian hasta"),
        (r"\bhally\b", "halberd"),
        (r"\bobby\b", "obsidian"),
        (r"\bd\b", "dragon"),
        (r"\bnezzy\b", "Helm of Neitiznot"),
    ];

    for (pattern, replacement) in patterns.iter() {
        let re = Regex::new(pattern).unwrap();
        if re.is_match(&query) {
            query = re.replace_all(&query, *replacement).to_string();
        }
    }

    query
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
                "Overall",
                "Attack",
                "Defence",
                "Strength",
                "Hitpoints",
                "Ranged",
                "Prayer",
                "Magic",
                "Cooking",
                "Woodcutting",
                "Fletching",
                "Fishing",
                "Firemaking",
                "Crafting",
                "Smithing",
                "Mining",
                "Herblore",
                "Agility",
                "Thieving",
                "Slayer",
                "Farming",
                "Runecraft",
                "Hunter",
                "Construction",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_level_to_xp() {
        assert_eq!(level_to_xp(1), 0);
        assert_eq!(level_to_xp(2), 83);
        assert_eq!(level_to_xp(3), 174);
        assert_eq!(level_to_xp(4), 276);
        assert_eq!(level_to_xp(5), 388);
        assert_eq!(level_to_xp(6), 512);
        assert_eq!(level_to_xp(7), 650);
        assert_eq!(level_to_xp(8), 801);
        assert_eq!(level_to_xp(9), 969);
        assert_eq!(level_to_xp(10), 1154);
        assert_eq!(level_to_xp(11), 1358);
        assert_eq!(level_to_xp(12), 1584);
        assert_eq!(level_to_xp(13), 1833);
        assert_eq!(level_to_xp(14), 2107);
        assert_eq!(level_to_xp(15), 2411);
        assert_eq!(level_to_xp(16), 2746);
        assert_eq!(level_to_xp(17), 3115);
        assert_eq!(level_to_xp(18), 3523);
        assert_eq!(level_to_xp(19), 3973);
        assert_eq!(level_to_xp(20), 4470);
        assert_eq!(level_to_xp(21), 5018);
        assert_eq!(level_to_xp(22), 5624);
        assert_eq!(level_to_xp(23), 6291);
        assert_eq!(level_to_xp(24), 7028);
        assert_eq!(level_to_xp(25), 7842);
        assert_eq!(level_to_xp(26), 8740);
        assert_eq!(level_to_xp(27), 9730);
        assert_eq!(level_to_xp(28), 10824);
        assert_eq!(level_to_xp(29), 12031);
        assert_eq!(level_to_xp(30), 13363);
        assert_eq!(level_to_xp(45), 61512);
        assert_eq!(level_to_xp(55), 166636);
        assert_eq!(level_to_xp(75), 1210421);
        assert_eq!(level_to_xp(92), 6517253);
        assert_eq!(level_to_xp(95), 8771558);
        assert_eq!(level_to_xp(96), 9684577);
        assert_eq!(level_to_xp(97), 10692629);
        assert_eq!(level_to_xp(98), 11805606);
        assert_eq!(level_to_xp(99), 13034431);
        assert_eq!(level_to_xp(100), 14391160);
        assert_eq!(level_to_xp(105), 23611006);
        assert_eq!(level_to_xp(110), 38737661);
        assert_eq!(level_to_xp(115), 63555443);
        assert_eq!(level_to_xp(120), 104273167);
        assert_eq!(level_to_xp(126), 188884740);
        assert_eq!(level_to_xp(127), 208545568);
    }

    #[test]
    fn test_xp_to_level() {
        assert_eq!(xp_to_level(0), 1);
        assert_eq!(xp_to_level(83), 2);
        assert_eq!(xp_to_level(174), 3);
        assert_eq!(xp_to_level(276), 4);
        assert_eq!(xp_to_level(388), 5);
        assert_eq!(xp_to_level(512), 6);
        assert_eq!(xp_to_level(650), 7);
        assert_eq!(xp_to_level(801), 8);
        assert_eq!(xp_to_level(969), 9);
        assert_eq!(xp_to_level(1154), 10);
        assert_eq!(xp_to_level(1358), 11);
        assert_eq!(xp_to_level(1584), 12);
        assert_eq!(xp_to_level(1833), 13);
        assert_eq!(xp_to_level(2107), 14);
        assert_eq!(xp_to_level(2411), 15);
        assert_eq!(xp_to_level(2746), 16);
        assert_eq!(xp_to_level(3115), 17);
        assert_eq!(xp_to_level(3523), 18);
        assert_eq!(xp_to_level(3973), 19);
        assert_eq!(xp_to_level(4470), 20);
        assert_eq!(xp_to_level(5018), 21);
        assert_eq!(xp_to_level(5624), 22);
        assert_eq!(xp_to_level(6291), 23);
        assert_eq!(xp_to_level(7028), 24);
        assert_eq!(xp_to_level(7842), 25);
        assert_eq!(xp_to_level(8740), 26);
        assert_eq!(xp_to_level(9730), 27);
        assert_eq!(xp_to_level(10824), 28);
        assert_eq!(xp_to_level(12031), 29);
        assert_eq!(xp_to_level(13363), 30);
        assert_eq!(xp_to_level(61512), 45);
        assert_eq!(xp_to_level(166636), 55);
        assert_eq!(xp_to_level(1210421), 75);
        assert_eq!(xp_to_level(6517253), 92);
        assert_eq!(xp_to_level(8771558), 95);
        assert_eq!(xp_to_level(9684577), 96);
        assert_eq!(xp_to_level(10692629), 97);
        assert_eq!(xp_to_level(11805606), 98);
        assert_eq!(xp_to_level(12352331), 98);
        assert_eq!(xp_to_level(13034431), 99);
        assert_eq!(xp_to_level(14391160), 100);
        assert_eq!(xp_to_level(23611006), 105);
        assert_eq!(xp_to_level(38737661), 110);
        assert_eq!(xp_to_level(63555443), 115);
        assert_eq!(xp_to_level(104273167), 120);
        assert_eq!(xp_to_level(188884740), 126);
        assert_eq!(xp_to_level(200000000), 126);
    }

    #[test]
    fn test_get_item_db() {
        let item_db = match get_item_db() {
            Ok(item_db) => item_db,
            _ => vec![],
        };

        assert_eq!(item_db.len(), 0);
    }
}
