use anyhow::Result;
use common::source::Source;

use crate::track::{MAX_LINE_LEN, pack_lines};

/// A fairy ring: its three-letter code and where it lands.
type Ring = (&'static str, &'static str);

/// Every working fairy ring code, in code order. Locations follow the wiki's
/// `Region: Place` wording, with a parenthetical for the thing people actually
/// search by (a boss, a minigame) where the place name doesn't say it.
/// https://oldschool.runescape.wiki/w/Fairy_rings
const RINGS: [Ring; 55] = [
    ("AIQ", "Asgarnia: Mudskipper Point"),
    ("AIR", "Islands: South-east of Ardougne"),
    ("AIS", "Varlamore: Auburn Valley"),
    ("AJP", "Varlamore: Avium Savannah"),
    ("AJQ", "Dungeons: Cave south of Dorgesh-Kaan"),
    ("AJR", "Kandarin: Slayer cave south-east of Rellekka"),
    ("AJS", "Islands: Penguins near Miscellania"),
    ("AKP", "Kharidian Desert: Necropolis (Tombs of Amascut)"),
    ("AKQ", "Kandarin: Piscatoris Hunter area"),
    ("AKR", "Great Kourend: Hosidius Vinery"),
    ("AKS", "Feldip Hills: Feldip Hunter area"),
    ("ALP", "Islands: Lighthouse"),
    ("ALQ", "Morytania: Haunted Woods east of Canifis"),
    ("ALR", "Other Realms: Abyssal Area (Abyss)"),
    ("ALS", "Kandarin: McGrubor's Wood"),
    (
        "BIP",
        "Islands: South-west of Mort Myre, on the River Salve",
    ),
    ("BIQ", "Kharidian Desert: Near the Kalphite Hive"),
    ("BIS", "Kandarin: Ardougne Zoo - Unicorns"),
    ("BJP", "Islands: Isle of Souls"),
    (
        "BJR",
        "Other Realms: Realm of the Fisher King (Grail Castle)",
    ),
    ("BJS", "Islands: Near Zul-Andra (Zulrah)"),
    ("BKP", "Feldip Hills: Chompy Marsh, south of Castle Wars"),
    ("BKQ", "Other Realms: Enchanted Valley"),
    ("BKR", "Morytania: Mort Myre Swamp, south of Canifis"),
    ("BKS", "Other Realms: Zanaris"),
    ("BLP", "Dungeons: TzHaar area"),
    ("BLQ", "Other Realms: Yu'biusk"),
    ("BLR", "Kandarin: Legends' Guild"),
    ("BLS", "Kebos Lowlands: South of Mount Quidamortem"),
    ("CIP", "Islands: Miscellania"),
    ("CIQ", "Kandarin: North-west of Yanille"),
    ("CIR", "Kebos Lowlands: South of Mount Karuulm"),
    ("CIS", "Great Kourend: Arceuus Library"),
    ("CJQ", "The Great Conch"),
    ("CJR", "Kandarin: Sinclair Mansion (east)"),
    ("CKP", "Other Realms: Cosmic entity's plane"),
    ("CKQ", "Varlamore: Aldarin"),
    (
        "CKR",
        "Karamja: South of Tai Bwo Wannai Village (Cairn Isle)",
    ),
    ("CKS", "Morytania: Canifis (Slayer Tower)"),
    ("CLP", "Islands: Draynor island"),
    ("CLR", "Islands: Ape Atoll"),
    ("CLS", "Islands: Yanille Chain (Hazelmere)"),
    ("DIP", "Other Realms: Abyssal Nexus (Abyssal Sire)"),
    ("DIQ", "Player-owned house: Superior garden"),
    ("DIR", "Other Realms: Gorak's Plane"),
    ("DIS", "Misthalin: Wizards' Tower"),
    ("DJP", "Kandarin: Tower of Life"),
    ("DJR", "Great Kourend: Chasm of Fire"),
    ("DKP", "Karamja: Gnome glider, south near Shilo Village"),
    ("DKR", "Misthalin: Edgeville"),
    ("DKS", "Fremennik: Polar Hunter area (Keldagrim entrance)"),
    ("DLP", "Grimstone (Grimstone Dungeon)"),
    ("DLQ", "Kharidian Desert: North of Nardah"),
    ("DLR", "Islands: Poison Waste south of Isafdar (Zulrah)"),
    ("DLS", "Dungeons: Myreque hideout under The Hollows"),
];

/// Rings listed before the count is cut short, so a broad query can't flood
/// the channel.
const MAX_RESULTS: usize = 12;

/// Rings whose code or location matches `query`, ignoring case.
fn search(query: &str) -> Vec<&'static Ring> {
    let query = query.trim().to_lowercase();

    if query.is_empty() {
        return vec![];
    }

    // An exact code is unambiguous - don't dilute it with substring hits.
    if let Some(ring) = RINGS.iter().find(|(code, _)| code.to_lowercase() == query) {
        return vec![ring];
    }

    RINGS
        .iter()
        .filter(|(code, location)| {
            code.to_lowercase().contains(&query) || location.to_lowercase().contains(&query)
        })
        .collect()
}

pub fn lookup(source: Source) -> Result<Vec<String>> {
    let prefix = source.l("Fairy Ring");
    let query = source.query.trim();

    if query.is_empty() {
        return Ok(vec![vec![prefix, source.c1("No query provided")].join(" ")]);
    }

    let matches = search(query);

    if matches.is_empty() {
        return Ok(vec![format!(
            "{}: {}",
            prefix,
            source.c1("No results found")
        )]);
    }

    if let [(code, location)] = matches[..] {
        return Ok(vec![format!(
            "{} {} {}",
            prefix,
            source.p(code),
            source.c2(location)
        )]);
    }

    let mut parts: Vec<String> = matches
        .iter()
        .take(MAX_RESULTS)
        .map(|(code, location)| format!("{} {}", source.p(code), source.c1(location)))
        .collect();

    if matches.len() > MAX_RESULTS {
        parts.push(source.c1(&format!("+{} more", matches.len() - MAX_RESULTS)));
    }

    Ok(pack_lines(&prefix, &parts, &source.c1(" | "), MAX_LINE_LEN))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_code_is_well_formed_unique_and_ordered() {
        for (code, location) in RINGS.iter() {
            let letters: Vec<char> = code.chars().collect();
            assert_eq!(letters.len(), 3, "{code} is not three letters");
            assert!("ABCD".contains(letters[0]), "{code} has a bad first letter");
            assert!(
                "IJKL".contains(letters[1]),
                "{code} has a bad second letter"
            );
            assert!("PQRS".contains(letters[2]), "{code} has a bad third letter");
            assert!(!location.is_empty(), "{code} has no location");
        }

        let codes: Vec<&str> = RINGS.iter().map(|(code, _)| *code).collect();
        let mut sorted = codes.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(codes, sorted, "codes must be unique and in code order");
    }

    #[test]
    fn akp_lands_at_the_necropolis() {
        let found = search("akp");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].0, "AKP");
        assert!(found[0].1.contains("Necropolis"), "got: {}", found[0].1);
    }

    #[test]
    fn an_exact_code_returns_only_that_ring() {
        // "cip" also appears inside "Piscatoris", so the exact code must win.
        let found = search("CIP");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].0, "CIP");
    }

    #[test]
    fn a_location_query_returns_every_match() {
        let found: Vec<&str> = search("zulrah").iter().map(|(code, _)| *code).collect();

        assert_eq!(found, vec!["BJS", "DLR"]);
    }

    #[test]
    fn searching_is_case_insensitive() {
        let upper: Vec<&str> = search("MISCELLANIA").iter().map(|(c, _)| *c).collect();
        let lower: Vec<&str> = search("miscellania").iter().map(|(c, _)| *c).collect();

        assert_eq!(upper, lower);
        assert_eq!(upper, vec!["AJS", "CIP"]);
    }

    #[test]
    fn nothing_matches_an_unknown_place() {
        assert!(search("narnia").is_empty());
        assert!(search("").is_empty());
    }
}
