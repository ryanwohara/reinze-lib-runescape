use crate::params::rank_matches;
use crate::spell::data::{SPELLS, Spell};
use anyhow::Result;
use common::source::Source;

pub mod data;

/// Names paired with their index into `SPELLS`, for `rank_matches`.
fn keys() -> Vec<(&'static str, usize)> {
    SPELLS
        .iter()
        .enumerate()
        .map(|(i, s)| (s.name, i))
        .collect()
}

/// `4 Fire, 3 Air, 1 Chaos`, or `Free` for the home teleports.
fn runes(spell: &Spell, s: &Source) -> String {
    if spell.runes.is_empty() {
        return s.c2("Free");
    }
    spell
        .runes
        .iter()
        .map(|(count, rune)| format!("{} {}", s.c2(&count.to_string()), s.c2(rune)))
        .collect::<Vec<String>>()
        .join(&s.c1(", "))
}

fn describe(spell: &Spell, s: &Source) -> Vec<String> {
    let prefix = s.l("Spell");
    let members = if spell.members { "P2P" } else { "F2P" };

    let mut facts = vec![
        s.c1(&["Lvl:", &s.c2(&spell.level.to_string())].join("")),
        s.c1(&["XP:", &s.c2(&format!("{}", spell.xp))].join("")),
    ];
    if let Some(damage) = spell.damage {
        facts.push(s.c1(&["Max:", &s.c2(&damage.to_string())].join("")));
    }

    vec![
        vec![
            prefix.to_string(),
            s.l(spell.name),
            s.p(members),
            s.p(&format!("{} {}", spell.spellbook, spell.kind)),
            s.c1("|"),
            facts.join(" "),
        ]
        .join(" "),
        vec![prefix, s.c1("Runes:"), runes(spell, s)].join(" "),
    ]
}

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let prefix = s.l("Spell");

    let ranked = rank_matches(&keys(), &s.query);
    let Some((_, index)) = ranked.first() else {
        return Ok(vec![vec![prefix, s.c1("Not found")].join(" ")]);
    };

    let mut output = describe(&SPELLS[*index], s);

    // The rest of the ranking is worth showing -- "fire" is eight spells --
    // but only as names, and only when there is more than one.
    let others: Vec<String> = ranked
        .iter()
        .skip(1)
        .take(9)
        .map(|(name, _)| s.c2(name))
        .collect();
    if !others.is_empty() {
        output.push(vec![s.l("Spell"), s.p("Also"), others.join(&s.c1(", "))].join(" "));
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::ColorResult;
    use common::author::Author;
    use std::ffi::CString;
    use std::os::raw::c_char;

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
            "spell",
            query,
        )
    }

    fn find(name: &str) -> &'static Spell {
        SPELLS
            .iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("{name} missing from SPELLS"))
    }

    #[test]
    fn every_spell_has_a_level_and_a_spellbook() {
        for spell in SPELLS {
            assert!(!spell.name.is_empty(), "a spell has no name");
            assert!(
                ["Normal", "Ancient", "Lunar", "Arceuus", "All"].contains(&spell.spellbook),
                "{}: unexpected spellbook {:?}",
                spell.name,
                spell.spellbook
            );
            assert!(
                ["Combat", "Teleport", "Utility"].contains(&spell.kind),
                "{}: unexpected kind {:?}",
                spell.name,
                spell.kind
            );
        }
    }

    #[test]
    fn an_exact_name_wins_over_its_longer_variants() {
        // "fire bolt" must not resolve to Fire Bolt (Guardians of the Rift)
        // or Fire Blast; this is what the shared ranking buys.
        let ranked = rank_matches(&keys(), "fire bolt");
        assert_eq!(SPELLS[ranked[0].1].name, "Fire Bolt");
    }

    #[test]
    fn a_name_with_spaces_matches_an_underscored_query() {
        let ranked = rank_matches(&keys(), "ice_barrage");
        assert_eq!(SPELLS[ranked[0].1].name, "Ice Barrage");
    }

    #[test]
    fn fire_bolt_reports_its_runes_and_max_hit() {
        let out = describe(find("Fire Bolt"), &source_with("fire bolt")).join(" ");
        for expected in [
            "Fire Bolt",
            "Lvl:",
            "35",
            "22.5",
            "Max:",
            "12",
            "Fire",
            "Chaos",
        ] {
            assert!(out.contains(expected), "missing {expected:?} in {out:?}");
        }
    }

    #[test]
    fn a_runeless_teleport_says_free() {
        let out = describe(find("Lumbridge Home Teleport"), &source_with("lumb")).join(" ");
        assert!(out.contains("Free"), "expected Free in {out:?}");
    }

    #[test]
    fn an_unmatched_query_says_not_found() {
        let out = lookup(&source_with("notaspell")).unwrap();
        assert_eq!(out.len(), 1);
        assert!(out[0].contains("Not found"), "got {:?}", out[0]);
    }

    #[test]
    fn output_uses_the_callers_colors() {
        let out = lookup(&source_with("fire bolt")).unwrap().join(" ");
        assert!(out.contains("\x0307"), "expected caller c1 in {out:?}");
        assert!(out.contains("\x0313"), "expected caller c2 in {out:?}");
    }

    /// `bin/gen-spells.py` emits `src/spell/data.rs` and
    /// `src/stats/magic.rs` from one parse, so `+spell` and `-mage` cannot
    /// disagree. This is what catches one being regenerated without the
    /// other.
    #[test]
    fn the_magic_skill_table_matches_the_spell_data() {
        use crate::stats::magic::Magic;
        use crate::stats::skill::{Details, Skill};

        let mut checked = 0;
        for magic in Magic::all() {
            let Details::Magic(details) = magic.details() else {
                panic!("Magic::details returned something other than Details::Magic");
            };
            let spell = SPELLS
                .iter()
                .find(|s| s.name == details.name)
                .unwrap_or_else(|| panic!("{} is in stats/magic.rs but not SPELLS", details.name));

            assert_eq!(details.level, spell.level, "{}: level", spell.name);
            assert_eq!(details.xp, spell.xp, "{}: xp", spell.name);
            assert_eq!(details.members, spell.members, "{}: members", spell.name);
            checked += 1;
        }

        assert_eq!(
            checked,
            SPELLS.len(),
            "stats/magic.rs has {checked} spells against SPELLS' {}; \
             run bin/gen-spells.py",
            SPELLS.len()
        );
    }
}
