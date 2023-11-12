use common::{
    c1, c2, commas_from_string, convert_split_to_string, get_rsn, get_stats, l, p,
    process_account_type_flags, unranked,
};
use mysql::from_row;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();
    let rsn = if query.is_empty() {
        get_rsn(author, rsn_n)
            .ok()
            .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
            .unwrap_or_else(|| nick)
    } else {
        query.to_string()
    };

    let bosses: [&str; 58] = [
        "Abyssal Sire",
        "Alchemical Hydra",
        "Artio",
        "Barrows Chests",
        "Bryophyta",
        "Callisto",
        "Calvar'ion",
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
        "Duke Sucellus",
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
        "Phantom Muspah",
        "Sarachnis",
        "Scorpia",
        "Skotizo",
        "Spindel",
        "Tempoross",
        "Guantlet",
        "Corrupted Gauntlet",
        "The Leviathan",
        "The Whisperer",
        "ToB",
        "ToB: Hard",
        "Thermonuclear Smoke Devil",
        "ToA",
        "ToA: Expert",
        "TzKal-Zuk",
        "TzTok-Jad",
        "Vardorvis",
        "Venenatis",
        "Vet'ion",
        "Vorkath",
        "Wintertodt",
        "Zalcano",
        "Zulrah",
    ];

    let split: Vec<String> = convert_split_to_string(query.split(" ").collect());

    let (split, mut prefix, base_url) = process_account_type_flags(query, split);

    let stats = match get_stats(&rsn, &base_url) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    let mut boss_kills: Vec<String> = Vec::new();
    let mut index = 0 - 1 as isize;
    let offset = 41;

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
                    c2(&commas_from_string(kills, "d")),
                    p(&commas_from_string(rank, "d"))
                ));
            }
        }
    }

    let output = format!("{} {}", l("Boss"), unranked(boss_kills));

    Ok(vec![output])
}
