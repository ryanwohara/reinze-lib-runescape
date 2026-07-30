use anyhow::Result;
use common::commas;
use common::source::Source;

use crate::common::{
    Entry, HiscoreName, Listing, MAX_SKILL_LEVEL, collect_hiscores, format_hours, level_to_xp,
    xp_to_level,
};
use crate::stats::{
    Goal, goal, goal_string, level_display, stats_parameters, strip_stats_parameters,
};

/// Hunter level needed to hunt herbiboar, and the level it becomes reachable
/// with a super hunter potion.
const HERBIBOAR_LEVEL: u32 = 80;
const HERBIBOAR_BOOSTED_LEVEL: u32 = 74;

/// Hunter XP for one herbiboar at the given *base* Hunter level.
/// https://oldschool.runescape.wiki/w/Herbiboar#Experience_and_harvesting
fn herbiboar_xp(level: u32) -> Option<u32> {
    match level {
        // Not huntable at all below the boosted entry point.
        ..=73 => None,
        74..=94 => Some(1770 + 30 * (level - 74)),
        // Jagex's curve steps by 15 at 95, then by 19 per level to 99.
        95..=99 => Some(2385 + 19 * (level - 95)),
        // The rate is set by the base level, which stops at 99 - virtual
        // levels earn no more per catch.
        _ => Some(2461),
    }
}

/// Catches an hour with stamina potions, and the gp an hour that earns with a
/// herb sack in use - both estimates, so the figures they feed are marked `~`.
const CATCHES_PER_HOUR: f64 = 60.0;
const GP_PER_HOUR: f64 = 400_000.0;

/// Hours `catches` herbiboars take at the assumed rate.
fn hours(catches: u32) -> f64 {
    catches as f64 / CATCHES_PER_HOUR
}

/// GP earned along the way to `catches` herbiboars.
fn profit(catches: u32) -> f64 {
    catches as f64 * GP_PER_HOUR / CATCHES_PER_HOUR
}

/// Herbiboars needed to carry `xp` up to `target_xp`, re-rating each catch as
/// the level rises. `None` when the level is too low to hunt them at all.
fn catches_between(xp: u32, target_xp: u32) -> Option<u32> {
    let mut current = xp;
    let mut catches = 0;

    while current < target_xp {
        current = current.saturating_add(herbiboar_xp(xp_to_level(current))?);
        catches += 1;
    }

    Some(catches)
}

pub fn lookup(source: Source) -> Result<Vec<String>> {
    let prefix = source.l("Herbiboar");
    let flags = stats_parameters(&source.query);
    let joined: String = strip_stats_parameters(&source.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    // `^` supplies a starting level (or raw XP) to calculate from, so the
    // command works as a plain calculator with no player involved.
    let listing = if flags.start > 0 {
        let xp = if flags.start > MAX_SKILL_LEVEL {
            flags.start
        } else {
            level_to_xp(flags.start)
        };

        Listing::Entry(Entry {
            name: HiscoreName::Hunter,
            level: xp_to_level(xp),
            xp,
            rank: 0,
        })
    } else {
        let hiscores = match collect_hiscores(&joined, &source, &flags) {
            Ok(hiscores) => hiscores,
            Err(_) => {
                return Ok(vec![format!(
                    "{} {}",
                    prefix,
                    source.c1("No hiscores found")
                )]);
            }
        };

        match hiscores.skill("Hunter") {
            Some(listing) => listing,
            None => {
                return Ok(vec![format!(
                    "{} {}",
                    prefix,
                    source.c1("No Hunter level found")
                )]);
            }
        }
    };

    let actual_level = listing.actual_level();
    let (reported_level, virtual_level) = level_display(listing.level(), actual_level);

    let level_string = vec![
        source.c1("Hunter"),
        source.c2(&reported_level.to_string()),
        virtual_level.map_or(String::new(), |level| source.p(&level.to_string())),
    ]
    .join(" ")
    .trim_end()
    .to_string();

    let xp_each = match herbiboar_xp(actual_level) {
        Some(xp) => xp,
        None => {
            let requirement = source.c1(&format!(
                "Requires {} Hunter ({} with a super hunter potion)",
                HERBIBOAR_LEVEL, HERBIBOAR_BOOSTED_LEVEL
            ));

            return Ok(vec![format!(
                "{} {}",
                prefix,
                vec![level_string, requirement].join(&source.c1(" | "))
            )]);
        }
    };

    let each_string = vec![
        source.c2(&commas(xp_each as f64, "d")),
        source.c1("XP each"),
        // 74-79 only earns that rate while a super hunter potion is up.
        if actual_level < HERBIBOAR_LEVEL {
            source.p("boosted")
        } else {
            String::new()
        },
    ]
    .join(" ")
    .trim_end()
    .to_string();

    let goal = goal(listing.xp(), actual_level, listing.next_level(&flags));

    let mut parts = vec![level_string, each_string, goal_string(&goal, &source)];

    let target_xp = listing.xp().saturating_add(goal.remaining());

    if goal != Goal::Maxed {
        if let Some(catches) = catches_between(listing.xp(), target_xp) {
            parts.push(
                vec![
                    source.c2(&commas(catches as f64, "d")),
                    source.c1("herbiboars"),
                ]
                .join(" "),
            );
            parts.push(source.c2(&format!("~{}", format_hours(hours(catches)))));
            parts.push(
                vec![
                    source.c2(&format!("~{}", commas(profit(catches), "d"))),
                    source.c1("gp"),
                ]
                .join(" "),
            );
        }
    }

    Ok(vec![format!(
        "{} {}",
        prefix,
        parts.join(&source.c1(" | "))
    )])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn herbiboar_xp_matches_the_wiki_table() {
        assert_eq!(herbiboar_xp(74), Some(1770));
        assert_eq!(herbiboar_xp(80), Some(1950));
        assert_eq!(herbiboar_xp(94), Some(2370));
        // The curve steps by 15 at 95, then 19 per level.
        assert_eq!(herbiboar_xp(95), Some(2385));
        assert_eq!(herbiboar_xp(96), Some(2404));
        assert_eq!(herbiboar_xp(99), Some(2461));
    }

    #[test]
    fn herbiboar_xp_is_none_below_the_boosted_requirement() {
        assert_eq!(herbiboar_xp(73), None);
        assert_eq!(herbiboar_xp(1), None);
    }

    #[test]
    fn herbiboar_xp_is_rated_on_the_base_level_past_99() {
        assert_eq!(herbiboar_xp(113), Some(2461));
        assert_eq!(herbiboar_xp(MAX_SKILL_LEVEL), Some(2461));
    }

    #[test]
    fn catches_round_up_to_cover_the_remaining_xp() {
        let start = level_to_xp(92);
        assert_eq!(catches_between(start, start + 2310 * 3), Some(3));
        assert_eq!(catches_between(start, start + 2310 * 3 + 1), Some(4));
    }

    #[test]
    fn catches_use_the_higher_rate_after_a_level_up() {
        // One catch short of 95, where the rate goes 2,370 -> 2,385.
        let start = level_to_xp(95) - 10;
        assert_eq!(catches_between(start, start + 2370 + 2385), Some(2));
    }

    #[test]
    fn no_catches_needed_when_the_target_is_already_met() {
        let xp = level_to_xp(92);
        assert_eq!(catches_between(xp, xp), Some(0));
        assert_eq!(catches_between(xp, xp - 1), Some(0));
    }

    #[test]
    fn catches_are_unavailable_below_the_requirement() {
        assert_eq!(catches_between(level_to_xp(70), level_to_xp(71)), None);
    }

    #[test]
    fn time_and_profit_scale_with_the_catch_count() {
        assert_eq!(hours(60), 1.0);
        assert_eq!(profit(60), 400_000.0);

        assert_eq!(hours(294), 4.9);
        assert_eq!(profit(294), 1_960_000.0);
    }
}
