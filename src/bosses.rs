use super::stats::process_stats_subsection;
use common::l;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 43;

    let categories: Vec<&str> = vec![
        "Abyssal Sire",
        "Alchemical Hydra",
        "Amoxliatl",
        "Araxxor",
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
        "Lunar Chests",
        "Mimic",
        "Nex",
        "Nightmare",
        "Phosani's Nightmare",
        "Obor",
        "Phantom Muspah",
        "Sarachnis",
        "Scorpia",
        "Scurrius",
        "Skotizo",
        "Sol Heredit",
        "Spindel",
        "Tempoross",
        "Gauntlet", // these lines should have the prefix of `The`
        "Corrupted Gauntlet",
        "Hueycoatl",
        "Royal Titans",
        "Leviathan",
        "Whisperer", // but it would be _even more_ to send on IRC
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

    process_stats_subsection(query, author, rsn_n, &l("Boss"), categories, OFFSET)
}
