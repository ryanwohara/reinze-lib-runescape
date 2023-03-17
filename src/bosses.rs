use crate::common::c1;
use crate::common::c2;
use crate::common::commas_from_string;
use crate::common::get_rsn;
use crate::common::l;
use crate::common::p;
use mysql::{from_row, Row};

pub fn bosses(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let row: Vec<Row>;
    let mut rsn: String = query.to_string();
    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();

    if rsn.len() == 0 {
        row = match get_rsn(author, rsn_n) {
            Ok(db_rsn) => db_rsn,
            Err(_) => vec![],
        };

        rsn = match row.first() {
            Some(db_rsn) => from_row(db_rsn.to_owned()),
            None => nick, // Default to the user's IRC nickname
        };
    }

    let bosses: [&str; 51] = [
        "Abyssal Sire",
        "Alchemical Hydra",
        "Barrows Chests",
        "Bryophyta",
        "Callisto",
        "Cerberus",
        "CoX",
        "CoX: Challenge",
        "Chaos Elemental",
        "Chaos Fanatic",
        "Commander Zilyana",
        "Corporeal Beast",
        "Crazy Archaeologist",
        "Dagannoth Prime",
        "Dagannoth Rex",
        "Dagannoth Supreme",
        "Deranged Archaeologist",
        "General Graardor",
        "Giant Mole",
        "Grotesque Guardians",
        "Hespori",
        "Kalphite Queen",
        "King Black Dragon",
        "Kraken",
        "Kree'Arra",
        "K'ril Tsutsaroth",
        "Mimic",
        "Nex",
        "Nightmare",
        "Phantom Muspah",
        "Phosani's Nightmare",
        "Obor",
        "Sarachnis",
        "Scorpia",
        "Skotizo",
        "Tempoross",
        "Guantlet",
        "Corrupted Gauntlet",
        "ToB",
        "ToB: Hard",
        "Thermonuclear Smoke Devil",
        "ToA",
        "ToA: Expert",
        "TzKal-Zuk",
        "TzTok-Jad",
        "Venenatis",
        "Vet'ion",
        "Vorkath",
        "Wintertodt",
        "Zalcano",
        "Zulrah",
    ];

    let url = format!(
        "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player={}",
        rsn
    );

    let resp = match reqwest::blocking::get(url) {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error making HTTP request: {}", e);
            return Err(());
        }
    };

    let string = match resp.text() {
        Ok(string) => string,
        Err(e) => {
            println!("Error getting text: {}", e);
            return Err(());
        }
    };

    let mut boss_kills: Vec<String> = Vec::new();
    let mut index = 0 - 1 as isize;
    let offset = 38;

    for line in string.lines() {
        index += 1;

        if index - offset >= 0 {
            let split: Vec<&str> = line.split(',').collect();

            if split[0] == "-1" {
                continue;
            }

            let name: &str = bosses[(index - offset) as usize];
            let rank = split[0];
            let kills = split[1];

            if bosses.contains(&name) {
                boss_kills.push(format!(
                    "{}: {} {}",
                    c1(name),
                    c2(&commas_from_string(kills)),
                    p(&commas_from_string(rank))
                ));
            }
        }
    }

    let output = format!("{} {}", l("Boss"), boss_kills.join(&c1(" | ")));

    Ok(vec![output])
}
