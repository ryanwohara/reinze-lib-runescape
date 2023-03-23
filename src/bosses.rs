use crate::common::c1;
use crate::common::c2;
use crate::common::commas_from_string;
use crate::common::get_rsn;
use crate::common::get_stats;
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

    let stats = match get_stats(&rsn) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    let mut boss_kills: Vec<String> = Vec::new();
    let mut index = 0 - 1 as isize;
    let offset = 38;

    for line in stats {
        index += 1;

        if index - offset >= 0 {
            if line[0] == "-1" {
                continue;
            }

            let name: &str = bosses[(index - offset) as usize];
            let rank = &line[0];
            let kills = &line[1];

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
