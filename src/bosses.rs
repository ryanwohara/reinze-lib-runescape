use crate::common::c1;
use crate::common::c2;
use crate::common::p;

#[allow(unused_comparisons)]
pub fn bosses(rsn: &str) -> Result<Vec<String>, ()> {
    let bosses: [&str; 50] = [
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

    let rt = tokio::runtime::Runtime::new().unwrap();

    let resp = match rt.block_on(reqwest::get(&url)) {
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
                boss_kills.push(format!("{}: {} {}", c1(name), c2(kills), p(rank)));
            }
        }
    }

    Ok(boss_kills)
}