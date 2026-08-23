//! `+cmb%` -- how much of an account's XP is combat.
//!
//! Reports the share of total XP earned in the seven combat skills, with the
//! levels and XP on each side of that split.

use crate::common::{Listing, Listings, collect_hiscores};
use crate::stats::{stats_parameters, strip_stats_parameters};
use anyhow::Result;
use common::{commas, source::Source};

/// The seven skills that feed the combat level.
const COMBAT: [&str; 7] = [
    "Attack",
    "Strength",
    "Defence",
    "Prayer",
    "Hitpoints",
    "Ranged",
    "Magic",
];

/// Levels and XP for one side of the split.
#[derive(Default)]
struct Tally {
    levels: u64,
    xp: u64,
}

/// Combat and non-combat totals for a set of hiscores.
///
/// Only skills count. A `SubEntry` is a boss kill count or minigame score --
/// its "xp" is a score, and adding those to a skill total would be nonsense.
/// `Overall` is skipped too, being the sum of the others.
///
/// The XP accumulates in `u64` on purpose: 23 skills at 200m is 4.6 billion
/// against a `u32`'s 4.29 billion, so the combat-only sums elsewhere in this
/// crate do not generalise to every skill at once.
fn split(hiscores: &Listings) -> (Tally, Tally) {
    let mut combat = Tally::default();
    let mut other = Tally::default();

    for listing in hiscores.iter() {
        if !matches!(listing, Listing::Entry(_)) {
            continue;
        }

        let name = listing.name().to_string();
        if name == "Overall" {
            continue;
        }

        let side = if COMBAT.contains(&name.as_str()) {
            &mut combat
        } else {
            &mut other
        };
        side.levels += listing.level() as u64;
        side.xp += listing.xp() as u64;
    }

    (combat, other)
}

fn describe(tally: &Tally, s: &Source) -> String {
    vec![
        s.c2(&commas(tally.levels as f64, "d")),
        s.c1("lvls"),
        s.c2(&commas(tally.xp as f64, "d")),
        s.c1("xp"),
    ]
    .join(" ")
}

pub fn percent(s: Source) -> Result<Vec<String>> {
    let prefix = s.l("Combat%");
    let not_found: Vec<String> = vec![vec![prefix.as_str(), &s.c1("No stats found")].join(" ")];

    let flags = stats_parameters(&s.query);
    let joined: String = strip_stats_parameters(&s.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let hiscores = match collect_hiscores(&joined, &s, &flags) {
        Ok(hiscores) => hiscores,
        Err(_) => return Ok(not_found),
    };

    let (combat, other) = split(&hiscores);
    let total = combat.xp + other.xp;
    if total == 0 {
        return Ok(not_found);
    }

    let share = combat.xp as f64 / total as f64 * 100.0;

    let output = vec![
        prefix,
        s.c1("Combat:"),
        s.c2(&format!("{share:.1}%")),
        s.c1("of"),
        s.c2(&commas(total as f64, "d")),
        s.c1("xp"),
        s.c1("|"),
        s.c1("Combat:"),
        s.p(&describe(&combat, &s)),
        s.c1("Non-combat:"),
        s.p(&describe(&other, &s)),
    ]
    .join(" ");

    Ok(vec![output])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Entry, HiscoreName, SubEntry};

    fn skill(name: HiscoreName, level: u32, xp: u32) -> Listing {
        Listing::Entry(Entry {
            name,
            rank: 0,
            level,
            xp,
        })
    }

    fn activity(name: HiscoreName, score: u32) -> Listing {
        Listing::SubEntry(SubEntry {
            name,
            rank: 0,
            xp: score,
        })
    }

    #[test]
    fn overall_is_not_counted_on_either_side() {
        // Overall is the sum of the rest; counting it would double everything.
        let hiscores = Listings::new(vec![
            skill(HiscoreName::Overall, 200, 3_000_000),
            skill(HiscoreName::Attack, 99, 2_000_000),
            skill(HiscoreName::Cooking, 99, 1_000_000),
        ]);

        let (combat, other) = split(&hiscores);
        assert_eq!(combat.xp, 2_000_000);
        assert_eq!(other.xp, 1_000_000);
        assert_eq!(combat.levels + other.levels, 198);
    }

    #[test]
    fn boss_scores_are_not_mistaken_for_skill_xp() {
        // A SubEntry's "xp" is a kill count. Folding 30,000 Zulrah kills into
        // the non-combat total would be silently, badly wrong.
        let hiscores = Listings::new(vec![
            skill(HiscoreName::Attack, 99, 13_034_431),
            activity(HiscoreName::Leagues, 30_000),
            activity(HiscoreName::Gridmaster, 12_345),
        ]);

        let (combat, other) = split(&hiscores);
        assert_eq!(combat.xp, 13_034_431);
        assert_eq!(other.xp, 0, "an activity score leaked into the skill total");
        assert_eq!(other.levels, 0);
    }

    #[test]
    fn the_seven_combat_skills_land_on_the_combat_side() {
        let hiscores = Listings::new(
            [
                HiscoreName::Attack,
                HiscoreName::Strength,
                HiscoreName::Defence,
                HiscoreName::Prayer,
                HiscoreName::Hitpoints,
                HiscoreName::Ranged,
                HiscoreName::Magic,
            ]
            .into_iter()
            .map(|name| skill(name, 1, 100))
            .collect(),
        );

        let (combat, other) = split(&hiscores);
        assert_eq!(combat.xp, 700);
        assert_eq!(other.xp, 0);
    }

    #[test]
    fn non_combat_skills_land_on_the_other_side() {
        let hiscores = Listings::new(vec![
            skill(HiscoreName::Cooking, 99, 13_034_431),
            skill(HiscoreName::Construction, 99, 13_034_431),
        ]);

        let (combat, other) = split(&hiscores);
        assert_eq!(combat.xp, 0);
        assert_eq!(other.xp, 26_068_862);
        assert_eq!(other.levels, 198);
    }

    #[test]
    fn a_total_that_would_overflow_a_u32_still_adds_up() {
        // 23 skills at 200m is 4.6b; a u32 stops at 4.29b, so this sum has to
        // be u64 or it wraps (and panics in a debug build).
        let hiscores = Listings::new(
            [
                HiscoreName::Attack,
                HiscoreName::Strength,
                HiscoreName::Defence,
                HiscoreName::Prayer,
                HiscoreName::Hitpoints,
                HiscoreName::Ranged,
                HiscoreName::Magic,
                HiscoreName::Cooking,
                HiscoreName::Woodcutting,
                HiscoreName::Fletching,
                HiscoreName::Fishing,
                HiscoreName::Firemaking,
                HiscoreName::Crafting,
                HiscoreName::Smithing,
                HiscoreName::Mining,
                HiscoreName::Herblore,
                HiscoreName::Agility,
                HiscoreName::Thieving,
                HiscoreName::Slayer,
                HiscoreName::Farming,
                HiscoreName::Runecraft,
                HiscoreName::Hunter,
                HiscoreName::Construction,
            ]
            .into_iter()
            .map(|name| skill(name, 99, 200_000_000))
            .collect(),
        );

        let (combat, other) = split(&hiscores);
        assert_eq!(combat.xp + other.xp, 4_600_000_000);
        assert_eq!(combat.xp, 1_400_000_000);
        assert!(
            combat.xp + other.xp > u32::MAX as u64,
            "the fixture no longer exceeds a u32, so it stopped testing anything"
        );
    }
}
