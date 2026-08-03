use crate::common::skill as common_skill;
use anyhow::Result;
use common::capitalize;
use common::source::Source;

/// The XP database, read only by `generated_tables_match_the_ini`.
///
/// Every skill is served from a generated table now, so nothing parses this at
/// runtime and it is no longer embedded in the shipped library. The file stays
/// because it is what `bin/gen-params.py` builds seventeen of those tables
/// from, which makes it worth keeping the guard that the two still agree.
#[cfg(test)]
const DATABASE_INI: &str = include_str!("../lib/Database.ini");

mod agility;
mod attack;
mod construction;
mod cooking;
mod crafting;
mod defence;
mod farming;
mod firemaking;
mod fishing;
mod fletching;
mod herblore;
mod hitpoints;
mod hunter;
mod magic;
mod mining;
mod prayer;
mod ranged;
mod runecraft;
mod slayer;
mod smithing;
mod strength;
mod thieving;
mod woodcutting;

/// Sections that exist in the INI but are served from `src/npc/data.rs`.
///
/// The INI keeps its own combat and Slayer sections and they are stale --
/// they disagree with the NPC data they shadow -- so nothing is generated
/// from them, and `generated_tables_match_the_ini` has to skip them rather
/// than compare. Compiled only for tests, which is where both guards live.
#[cfg(test)]
const NPC_SERVED_SECTIONS: &[&str] = &[
    "Attack",
    "Defence",
    "Hitpoints",
    "Ranged",
    "Slayer",
    "Strength",
];

/// Every skill served from `src/npc/data.rs` rather than the INI, paired with
/// the module holding its table. Drives `npc_tables_match_the_npc_data`.
#[cfg(test)]
const NPC_TABLES: &[(&str, &[(&str, &str)])] = &[
    ("Attack", attack::ENTRIES),
    ("Defence", defence::ENTRIES),
    ("Hitpoints", hitpoints::ENTRIES),
    ("Ranged", ranged::ENTRIES),
    ("Slayer", slayer::ENTRIES),
    ("Strength", strength::ENTRIES),
];

/// The generated table for a capitalised skill name, or None if the name is
/// not a skill. Every skill has one; there is no runtime fallback left.
fn table_for(skill: &str) -> Option<&'static [(&'static str, &'static str)]> {
    Some(match skill {
        "Agility" => agility::ENTRIES,
        "Attack" => attack::ENTRIES,
        "Construction" => construction::ENTRIES,
        "Cooking" => cooking::ENTRIES,
        "Crafting" => crafting::ENTRIES,
        "Defence" => defence::ENTRIES,
        "Farming" => farming::ENTRIES,
        "Firemaking" => firemaking::ENTRIES,
        "Fishing" => fishing::ENTRIES,
        "Fletching" => fletching::ENTRIES,
        "Herblore" => herblore::ENTRIES,
        "Hitpoints" => hitpoints::ENTRIES,
        "Hunter" => hunter::ENTRIES,
        "Magic" => magic::ENTRIES,
        "Mining" => mining::ENTRIES,
        "Prayer" => prayer::ENTRIES,
        "Ranged" => ranged::ENTRIES,
        "Runecraft" => runecraft::ENTRIES,
        "Slayer" => slayer::ENTRIES,
        "Smithing" => smithing::ENTRIES,
        "Strength" => strength::ENTRIES,
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
    scored.sort_by(|a, b| {
        a.0.cmp(&b.0)
            .then(a.1.cmp(&b.1))
            .then(a.2.cmp(&b.2))
            .then(a.3.cmp(&b.3))
    });
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

    let Some(entries) = table_for(&name) else {
        return Ok(vec![format!("{} {}", prefix, s.c1("No results found"))]);
    };

    let found_params: Vec<String> = rank_matches(entries, param)
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
    use ini::Ini;
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

    // Both cases return before `lookup` reaches `table_for`/`database()`, so
    // these two tests stay offline; `generated_tables_match_the_ini` below is
    // the one test in this module that actually parses the embedded INI.

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
            if NPC_SERVED_SECTIONS.contains(&name) {
                // The INI still carries these sections and they are stale.
                // Their table comes from npc/data.rs, so comparing the two
                // here would fail by design; npc_tables_match_the_npc_data
                // is what guards them.
                assert!(
                    table_for(name).is_some(),
                    "[{name}] is served from npc/data.rs but has no table"
                );
                continue;
            }

            let table = table_for(name).unwrap_or_else(|| {
                panic!(
                    "[{name}] has no generated table; run bin/gen-params.py and add it to table_for"
                )
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

        assert_eq!(
            checked, 17,
            "expected 17 generated sections, checked {checked}; if you added a skill, update this count"
        );
    }

    /// The npc-served counterpart of `generated_tables_match_the_ini`.
    ///
    /// Checks against `NpcMetadata` rather than re-parsing `npc/data.rs`,
    /// because a second parser here would be free to drift from the one in
    /// `bin/gen-npc-params.py` while both kept passing. Rust renders `f64`
    /// with the shortest form that round-trips, which is the same text the
    /// generator produces by dropping a trailing `.0`.
    #[test]
    fn npc_tables_match_the_npc_data() {
        use crate::npc::data::{Npc, NpcMetadata};
        use crate::stats::skill::Skill;

        for (section, table) in NPC_TABLES {
            let expected: Vec<(String, String)> = Npc::all()
                .iter()
                .map(NpcMetadata::from)
                .filter(|npc| !npc.name.is_empty())
                .filter_map(|npc| {
                    let value = match *section {
                        "Hitpoints" => npc.hitpoints_xp,
                        "Slayer" => npc.slayer_xp,
                        _ => npc.combat_xp,
                    };
                    (value != 0.0).then(|| (npc.name.replace(' ', "_"), format!("{value}")))
                })
                .collect();

            assert_eq!(
                table.len(),
                expected.len(),
                "[{section}] has {} generated entries but {} in npc/data.rs; \
                 run bin/gen-npc-params.py",
                table.len(),
                expected.len()
            );
            for (i, (generated, want)) in table.iter().zip(expected.iter()).enumerate() {
                assert_eq!(
                    (generated.0, generated.1),
                    (want.0.as_str(), want.1.as_str()),
                    "[{section}] entry {i} differs; run bin/gen-npc-params.py"
                );
            }
        }
    }

    /// There is no runtime fallback left, so a skill `table_for` does not know
    /// answers "No results found" for every query rather than reaching a data
    /// source. The INI is the canonical list of section names.
    #[test]
    fn every_skill_has_a_table() {
        let ini = Ini::load_from_str(DATABASE_INI).expect("embedded ini parses");
        let mut count = 0;

        for name in ini.sections().flatten() {
            assert!(
                table_for(name).is_some(),
                "[{name}] is a skill section with no table in table_for"
            );
            count += 1;
        }

        assert_eq!(count, 23, "expected 23 skill sections, saw {count}");
        assert!(table_for("Notaskill").is_none());
    }
}
