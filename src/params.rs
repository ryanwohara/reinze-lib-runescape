extern crate ini;

use crate::common::skill as common_skill;
use anyhow::Result;
use common::capitalize;
use common::source::Source;
use ini::Ini;
use std::sync::OnceLock;

/// The XP database. Embedded rather than read at runtime: the old
/// `load_from_file` path resolved against the bot's working directory, which
/// is what kept this file in the other repository.
const DATABASE_INI: &str = include_str!("../lib/Database.ini");

/// Parsed once per process. The previous code re-read and re-parsed 116 KB on
/// every invocation of the command.
fn database() -> &'static Ini {
    static DB: OnceLock<Ini> = OnceLock::new();
    DB.get_or_init(|| Ini::load_from_str(DATABASE_INI).expect("embedded Database.ini must parse"))
}

mod agility;
mod construction;
mod cooking;
mod crafting;
mod farming;
mod firemaking;
mod fishing;
mod fletching;
mod herblore;
mod hunter;
mod magic;
mod mining;
mod prayer;
mod runecraft;
mod smithing;
mod thieving;
mod woodcutting;

/// Sections still served from the embedded INI, pending the npc/data.rs
/// consolidation. Everything else has a generated table.
const INI_SECTIONS: &[&str] = &[
    "Attack",
    "Defence",
    "Hitpoints",
    "Ranged",
    "Slayer",
    "Strength",
];

/// The generated table for a capitalised skill name, or None if that skill is
/// still served from the embedded INI.
fn table_for(skill: &str) -> Option<&'static [(&'static str, &'static str)]> {
    Some(match skill {
        "Agility" => agility::ENTRIES,
        "Construction" => construction::ENTRIES,
        "Cooking" => cooking::ENTRIES,
        "Crafting" => crafting::ENTRIES,
        "Farming" => farming::ENTRIES,
        "Firemaking" => firemaking::ENTRIES,
        "Fishing" => fishing::ENTRIES,
        "Fletching" => fletching::ENTRIES,
        "Herblore" => herblore::ENTRIES,
        "Hunter" => hunter::ENTRIES,
        "Magic" => magic::ENTRIES,
        "Mining" => mining::ENTRIES,
        "Prayer" => prayer::ENTRIES,
        "Runecraft" => runecraft::ENTRIES,
        "Smithing" => smithing::ENTRIES,
        "Thieving" => thieving::ENTRIES,
        "Woodcutting" => woodcutting::ENTRIES,
        _ => return None,
    })
}

/// Every entry whose key matches `query`, best first. The caller caps the
/// count.
///
/// Ranked by tier (exact, then prefix, then substring), then by fewest extra
/// underscore-separated tokens, then case-insensitively alphabetically by
/// key. The token count is what lifts a buried canonical answer above its
/// longer variants; the alphabetical tiebreak is what sorts a list of peers
/// into a stable order regardless of input order. Values are carried through
/// untouched and paired with their own key, so a duplicate key does not
/// cause one entry's value to be reported for another's.
pub(crate) fn rank_matches<'a>(
    entries: &[(&'a str, &'a str)],
    query: &str,
) -> Vec<(&'a str, &'a str)> {
    let needle = query.replace(' ', "_").to_ascii_lowercase();
    if needle.is_empty() {
        return Vec::new();
    }
    let needle_tokens = needle.split('_').count();

    let mut scored: Vec<(u8, usize, String, &'a str, &'a str)> = entries
        .iter()
        .filter_map(|(key, value)| {
            let lower = key.to_ascii_lowercase();
            let tier = if lower == needle {
                0
            } else if lower.starts_with(&needle) {
                1
            } else if lower.contains(&needle) {
                2
            } else {
                return None;
            };
            let extra = lower.split('_').count().saturating_sub(needle_tokens);
            Some((tier, extra, lower, *key, *value))
        })
        .collect();

    // Sort on tier, extra tokens, lowercased key, and the original key for a
    // total order — deliberately not on value, so a stable sort leaves a
    // duplicate key's entries in their original relative order rather than
    // reordering by value.
    scored.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)).then(a.3.cmp(&b.3)));
    scored
        .into_iter()
        .map(|(_, _, _, key, value)| (key, value))
        .collect()
}

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let prefix = s.l("Params");

    let (skill, param) = match s.query.split_once(" ") {
        Some((skill, param)) if !skill.is_empty() && !param.is_empty() => {
            (common_skill(skill), param)
        }
        _ => {
            return Ok(vec![format!(
                "{} {}",
                prefix,
                s.c2("Invalid number of arguments")
            )]);
        }
    };

    if skill.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, s.c2("Invalid skill"))]);
    }

    let name = capitalize(&skill);
    let prefix = s.l(&name);

    let entries: Vec<(&str, &str)> = match table_for(&name) {
        Some(table) => table.to_vec(),
        None => match database().section(Some(name.clone())) {
            Some(section) => section.iter().collect(),
            None => return Ok(vec![format!("{} {}", prefix, s.c1("No results found"))]),
        },
    };

    let found_params: Vec<String> = rank_matches(&entries, param)
        .into_iter()
        .take(10)
        .map(|(k, v)| {
            format!(
                "{} {}",
                s.c1(&k.replace("_", " ")),
                s.c2(&format!("{}xp", v))
            )
        })
        .collect();
    Ok(vec![format!("{} {}", prefix, s.not_found(found_params))])
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::ColorResult;
    use common::author::Author;
    use std::ffi::CString;
    use std::os::raw::c_char;

    /// Returns distinctive colors so a hard-coded default is easy to spot.
    /// `Author::colors` takes ownership of both pointers and frees them, so
    /// these must be freshly allocated on every call.
    extern "C" fn stub_color(_host: *const c_char, _colors: *const c_char) -> ColorResult {
        ColorResult {
            c1: CString::new("07").unwrap().into_raw(),
            c2: CString::new("13").unwrap().into_raw(),
        }
    }

    fn source_with(query: &str) -> Source {
        Source::create(
            "0",
            Author::create("nick!ident@host", stub_color),
            "params",
            query,
        )
    }

    fn assert_caller_colors(text: &str) {
        assert!(
            text.contains("\x0307"),
            "expected caller c1 (07) in: {text:?}"
        );
        assert!(
            text.contains("\x0313"),
            "expected caller c2 (13) in: {text:?}"
        );
        assert!(
            !text.contains("\x0314"),
            "hard-coded default c1 (14) leaked into: {text:?}"
        );
        assert!(
            !text.contains("\x0304"),
            "hard-coded default c2 (04) leaked into: {text:?}"
        );
    }

    // Both cases return before the Database.ini load, so these stay offline.

    /// Wraps bare keys as `(key, "0")` pairs so the ranking tests below can
    /// keep asserting on keys alone without hand-building tuples.
    fn pairs<'a>(keys: &[&'a str]) -> Vec<(&'a str, &'a str)> {
        keys.iter().map(|k| (*k, "0")).collect()
    }

    fn ranked_keys<'a>(keys: &[&'a str], query: &str) -> Vec<&'a str> {
        rank_matches(&pairs(keys), query)
            .into_iter()
            .map(|(k, _)| k)
            .collect()
    }

    #[test]
    fn params_bad_arguments_use_the_callers_colors() {
        let out = lookup(&source_with("")).unwrap();
        assert_caller_colors(&out[0]);
    }

    #[test]
    fn params_invalid_skill_uses_the_callers_colors() {
        let out = lookup(&source_with("notaskill somequery")).unwrap();
        assert_caller_colors(&out[0]);
    }

    #[test]
    fn ranking_puts_an_exact_match_first() {
        let keys = ["Steel_bar", "Bar_magnet", "Bar"];
        assert_eq!(ranked_keys(&keys, "bar")[0], "Bar");
    }

    #[test]
    fn ranking_puts_a_prefix_match_above_a_substring() {
        let keys = ["Steel_bar", "Bar_magnet"];
        assert_eq!(ranked_keys(&keys, "bar"), vec!["Bar_magnet", "Steel_bar"]);
    }

    #[test]
    fn ranking_prefers_fewer_extra_tokens() {
        // The plain cannonballs must outrank the chainshot and incendiary
        // variants; this is the regression that motivated the change.
        let keys = [
            "Adamant_chainshot_cannonball",
            "Steel_cannonball",
            "Bronze_incendiary_cannonball",
            "Rune_cannonball",
        ];
        assert_eq!(
            ranked_keys(&keys, "cannonball"),
            vec![
                "Rune_cannonball",
                "Steel_cannonball",
                "Adamant_chainshot_cannonball",
                "Bronze_incendiary_cannonball",
            ]
        );
    }

    #[test]
    fn ranking_keeps_peers_alphabetical() {
        // A list of same-shaped matches must not be reshuffled.
        let keys = ["Camelot_Teleport", "Annakarl_Teleport", "Ardougne_Teleport"];
        assert_eq!(
            ranked_keys(&keys, "teleport"),
            vec!["Annakarl_Teleport", "Ardougne_Teleport", "Camelot_Teleport"]
        );
    }

    #[test]
    fn ranking_treats_spaces_in_the_query_as_underscores() {
        let keys = ["Oak_bird_house", "Bird_house", "Birdsong"];
        assert_eq!(
            ranked_keys(&keys, "bird house"),
            vec!["Bird_house", "Oak_bird_house"]
        );
    }

    #[test]
    fn ranking_is_case_insensitive_both_ways() {
        let keys = ["GOLD_BAR", "gold_ore"];
        assert_eq!(ranked_keys(&keys, "GoLd_BaR"), vec!["GOLD_BAR"]);
    }

    #[test]
    fn ranking_returns_nothing_when_no_key_matches() {
        let keys = ["Gold_bar", "Iron_bar"];
        assert!(ranked_keys(&keys, "dragon").is_empty());
    }

    #[test]
    fn ranking_returns_nothing_for_an_empty_query() {
        let keys = ["Gold_bar"];
        assert!(ranked_keys(&keys, "").is_empty());
    }

    #[test]
    fn ranking_carries_each_entrys_own_value() {
        // A duplicated key must not make both rows show the first value.
        let entries = [("Gold_bar", "22.5"), ("Gold_bar", "56.2")];
        assert_eq!(
            rank_matches(&entries, "gold bar"),
            vec![("Gold_bar", "22.5"), ("Gold_bar", "56.2")]
        );
    }

    #[test]
    fn generated_tables_match_the_ini() {
        let ini = Ini::load_from_str(DATABASE_INI).expect("embedded ini parses");
        let mut checked = 0;

        for section in ini.sections() {
            let Some(name) = section else { continue };
            if INI_SECTIONS.contains(&name) {
                assert!(
                    table_for(name).is_none(),
                    "[{name}] is listed as INI-served but also has a generated table"
                );
                continue;
            }

            let table = table_for(name).unwrap_or_else(|| {
                panic!("[{name}] has no generated table; run bin/gen-params.py and add it to table_for")
            });
            let from_ini: Vec<(&str, &str)> = ini
                .section(Some(name))
                .expect("section exists")
                .iter()
                .collect();

            assert_eq!(
                table.len(),
                from_ini.len(),
                "[{name}] has {} generated entries but {} in the INI; regenerate",
                table.len(),
                from_ini.len()
            );
            for (i, (generated, ini_entry)) in table.iter().zip(from_ini.iter()).enumerate() {
                assert_eq!(
                    generated, ini_entry,
                    "[{name}] entry {i} differs; regenerate"
                );
            }
            checked += 1;
        }

        assert_eq!(checked, 17, "expected 17 generated sections, checked {checked}");
    }
}
