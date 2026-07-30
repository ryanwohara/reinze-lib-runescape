use anyhow::Result;
use common::commas;
use common::source::Source;

use crate::common::{
    Entry, HiscoreName, Listing, MAX_SKILL_LEVEL, collect_hiscores, format_hours, get_ge_data,
    get_item_db, level_to_xp, price_of, short_gp, xp_to_level,
};
use crate::fish::{FISH, Fish, Stop, find_fish};
use crate::items::{Mapping, Price};
use crate::stats::{
    Goal, StatsFlags, goal, goal_string, level_display, stats_parameters, strip_stats_parameters,
};
use crate::track::{MAX_LINE_LEN, pack_lines};
use std::collections::HashMap;

/// Fish cooked per hour, the rate the wiki's money making guides assume.
/// https://oldschool.runescape.wiki/w/Money_making_guide/Cooking_raw_sharks
const FISH_PER_HOUR: u32 = 1_300;

/// Burn rate at a fish's own cooking level, from which it falls linearly to
/// nothing at the level burning stops.
///
/// The game's real burn curve is not published - the wiki gives only the level
/// where burning stops - so this anchor is a modelling choice, not a wiki
/// figure. Everything derived from it is printed with a `~`.
const MAX_BURN: f64 = 0.50;

/// `Stop::Never` means burning continues past 99, so the curve is interpolated
/// towards a notional level 100 instead of a real stop level.
const NEVER_STOPS_AT: f64 = 100.0;

/// Old School RuneScape cooking uses only the real Cooking level (capped at 99)
/// to determine burn rate. Virtual levels above 99 are XP artefacts and do not
/// reduce burn; a player at 200M XP burns exactly as much as one at 13.03M.
const REAL_LEVEL_CAP: f64 = 99.0;

/// The Grand Exchange takes 2% of a sale, rounded down, capped per item.
/// 1% until 29 May 2025. https://oldschool.runescape.wiki/w/Grand_Exchange
const GE_TAX_PERCENT: u64 = 2;
const GE_TAX_CAP: u64 = 5_000_000;

/// The share of fish burnt at `level` for one setup. Burnt fish earn no XP and
/// sell for nothing, but still cost a raw fish.
fn burn(level: u32, cook_level: u32, stop: Stop) -> f64 {
    let stop = match stop {
        Stop::NoBurn => return 0.0,
        Stop::Never => NEVER_STOPS_AT,
        Stop::Level(stop) => stop as f64,
    };

    let cook = cook_level as f64;
    // Below the cooking level the fish cannot be cooked at all; rating it at
    // the cooking level keeps the curve inside 0..MAX_BURN.
    // Virtual levels above 99 do not reduce burn rate, so clamp to the real
    // level cap after the cooking-level floor.
    let level = (level as f64).max(cook).min(REAL_LEVEL_CAP);

    if stop <= cook || level >= stop {
        return 0.0;
    }

    MAX_BURN * (stop - level) / (stop - cook)
}

/// The tax on selling one item. Integer maths throughout: 2% of 991 is 19, not
/// 19.82, and a float would round the wrong way on exact multiples.
fn tax(price: u32) -> u32 {
    (price as u64 * GE_TAX_PERCENT / 100).min(GE_TAX_CAP) as u32
}

/// An hour of cooking, in gp. Signed: cooking often loses money.
#[derive(Debug, PartialEq)]
pub struct Hourly {
    pub burn: f64,
    pub cost: i64,
    pub revenue: i64,
    pub profit: i64,
}

fn hourly(raw: u32, cooked: u32, burn: f64) -> Hourly {
    let fish = FISH_PER_HOUR as f64;
    let sold = (cooked - tax(cooked)) as f64;

    let cost = fish * raw as f64;
    let revenue = fish * (1.0 - burn) * sold;

    Hourly {
        burn,
        cost: cost.round() as i64,
        revenue: revenue.round() as i64,
        profit: (revenue - cost).round() as i64,
    }
}

/// The setups reported for a fish, worst first. The gauntlet row is omitted
/// for the fish gauntlets do not affect rather than repeating the range figure
/// under a label that would imply they help.
fn setups(fish: &Fish) -> Vec<(&'static str, Stop)> {
    let mut setups = vec![("Fire", fish.fire), ("Range", fish.range)];

    match &fish.gauntlets {
        Some(gauntlets) => {
            setups.push(("Gauntlets", gauntlets.default));
            setups.push(("Hosidius", gauntlets.hosidius10));
        }
        None => setups.push(("Hosidius", fish.hosidius10)),
    }

    setups
}

/// Raw fish needed to carry `xp` up to `target_xp`, re-rating burn as the level
/// rises. Walks level bands rather than single fish, because burn is constant
/// within a level and a 200m-XP target would otherwise iterate millions of
/// times. `None` when the level is below the fish's cooking level.
fn fish_between(xp: u32, target_xp: u32, fish: &Fish, stop: Stop) -> Option<u64> {
    if xp_to_level(xp) < fish.level {
        return None;
    }

    let mut current = xp as f64;
    let mut count: u64 = 0;

    while (current as u32) < target_xp {
        let level = xp_to_level(current as u32);
        let each = fish.xp * (1.0 - burn(level, fish.level, stop));

        if each <= 0.0 {
            return None;
        }

        // Burn only changes at a level up, so the whole band is one rate.
        let band_end = if level >= MAX_SKILL_LEVEL {
            target_xp
        } else {
            level_to_xp(level + 1).min(target_xp)
        };

        let in_band = (((band_end as f64 - current) / each).ceil() as u64).max(1);

        count += in_band;
        current += in_band as f64 * each;
    }

    Some(count)
}

/// Signed gp is coloured by sign rather than by the c1/c2 palette: those two
/// colours are per-user configurable and carry no profit/loss meaning, so a
/// themed colour cannot say "this loses money".
const GREEN: &str = "\x0303";
const RED: &str = "\x0304";

fn gp(amount: i64) -> String {
    format!(
        "{}{}",
        if amount < 0 { RED } else { GREEN },
        short_gp(amount)
    )
}

/// A burn rate for display. A modelled rate is marked `~`; an exact 0% from the
/// wiki's own table is not.
fn burn_label(burn: f64) -> String {
    if burn <= 0.0 {
        return "0%".to_string();
    }

    format!("~{}%", (burn * 100.0).round())
}

/// A fish's per-cook XP for display. Whole XP prints as an integer; the two
/// fractional-XP fish (Sea turtle at 211.3, Manta ray at 216.3) keep their
/// decimal - truncating it would print a number that is not the one in the
/// table.
fn xp_label(xp: f64) -> String {
    if xp.fract() == 0.0 {
        commas(xp, "d")
    } else {
        commas(xp, ".1f")
    }
}

/// The Cooking level a listing reports, and the level to calculate from.
struct Cook {
    listing: Listing,
    level: u32,
}

/// The player's Cooking listing, or a synthetic one when `^N` supplied a level
/// (or raw XP) to calculate from instead.
fn cooking(source: &Source, prefix: &str, flags: &StatsFlags) -> Result<Cook, Vec<String>> {
    let joined: String = strip_stats_parameters(&source.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let listing = if flags.start > 0 {
        let xp = if flags.start > MAX_SKILL_LEVEL {
            flags.start
        } else {
            level_to_xp(flags.start)
        };

        Listing::Entry(Entry {
            name: HiscoreName::Cooking,
            level: xp_to_level(xp),
            xp,
            rank: 0,
        })
    } else {
        let hiscores = match collect_hiscores(&joined, source, flags) {
            Ok(hiscores) => hiscores,
            Err(_) => {
                return Err(vec![format!(
                    "{} {}",
                    prefix,
                    source.c1("No hiscores found")
                )]);
            }
        };

        match hiscores.skill("Cooking") {
            Some(listing) => listing,
            None => {
                return Err(vec![format!(
                    "{} {}",
                    prefix,
                    source.c1("No Cooking level found")
                )]);
            }
        }
    };

    let level = listing.actual_level();

    Ok(Cook { listing, level })
}

/// The best of the reported setups - the one the ranked list and the goal block
/// are quoted at.
fn best_setup(fish: &Fish) -> (&'static str, Stop) {
    let setups = setups(fish);

    *setups.last().expect("every fish reports at least one setup")
}

pub fn lookup(source: Source) -> Result<Vec<String>> {
    let prefix = source.l("Chef");
    let flags = stats_parameters(&source.query);

    let items = get_item_db()?;
    let ge = get_ge_data()?;

    let cook = match cooking(&source, &prefix, &flags) {
        Ok(cook) => cook,
        Err(lines) => return Ok(lines),
    };

    let (reported_level, virtual_level) = level_display(cook.listing.level(), cook.level);
    let level_string = vec![
        source.c1("Cooking"),
        source.c2(&reported_level.to_string()),
        virtual_level.map_or(String::new(), |level| source.p(&level.to_string())),
    ]
    .join(" ")
    .trim_end()
    .to_string();

    if flags.search.is_empty() {
        return Ok(ranked(&source, &prefix, &level_string, cook.level, &items, &ge));
    }

    let fish = match find_fish(&flags.search) {
        Some(fish) => fish,
        None => {
            return Ok(vec![format!(
                "{} {}",
                prefix,
                source.c1(&format!(
                    "'{}' isn't a cookable fish - try +chef for the full list",
                    flags.search
                ))
            )]);
        }
    };

    Ok(detail(
        &source,
        &prefix,
        &level_string,
        &cook,
        fish,
        &flags,
        &items,
        &ge,
    ))
}

/// Every fish ranked by profit at its best setup, most profitable first. Fish
/// above the caller's level are kept - the market answer is useful before the
/// level is - but marked, and quoted with no burning: rating a fish you cannot
/// cook at the worst burn rate on the curve says nothing about the market.
fn ranked(
    source: &Source,
    prefix: &str,
    level_string: &str,
    level: u32,
    items: &[Mapping],
    ge: &HashMap<u32, Price>,
) -> Vec<String> {
    let mut ranked: Vec<(&Fish, Hourly, bool)> = FISH
        .iter()
        .filter_map(|fish| {
            let raw = price_of(items, ge, fish.raw)?;
            let cooked = price_of(items, ge, fish.cooked)?;
            let (_, stop) = best_setup(fish);
            let locked = level < fish.level;

            let burnt = if locked {
                0.0
            } else {
                burn(level, fish.level, stop)
            };

            Some((fish, hourly(raw, cooked, burnt), locked))
        })
        .collect();

    if ranked.is_empty() {
        return vec![format!("{} {}", prefix, source.c1("No fish prices"))];
    }

    ranked.sort_by(|(_, a, _), (_, b, _)| b.profit.cmp(&a.profit));

    let locked = ranked.iter().any(|(_, _, locked)| *locked);

    let parts: Vec<String> = ranked
        .iter()
        .map(|(fish, hour, locked)| {
            format!(
                "{} {}{}",
                source.c1(fish.name),
                gp(hour.profit),
                if *locked { source.c1("*") } else { String::new() }
            )
        })
        .collect();

    let mut lines = pack_lines(
        &format!("{} {} {}", prefix, level_string, source.c1("Profit/hr:")),
        &parts,
        &source.c1(" | "),
        MAX_LINE_LEN,
    );

    if locked {
        lines.push(format!(
            "{} {}",
            prefix,
            source.p("* above your Cooking level, quoted with no burning")
        ));
    }

    lines
}

/// One fish across every setup, plus what it takes to reach the goal.
fn detail(
    source: &Source,
    prefix: &str,
    level_string: &str,
    cook: &Cook,
    fish: &Fish,
    flags: &StatsFlags,
    items: &[Mapping],
    ge: &HashMap<u32, Price>,
) -> Vec<String> {
    let (raw, cooked) = match (
        price_of(items, ge, fish.raw),
        price_of(items, ge, fish.cooked),
    ) {
        (Some(raw), Some(cooked)) => (raw, cooked),
        _ => {
            return vec![format!(
                "{} {}",
                prefix,
                source.c1(&format!("No price for {}", fish.name))
            )];
        }
    };

    let header = vec![
        source.c2(fish.name),
        level_string.to_string(),
        vec![source.c2(&xp_label(fish.xp)), source.c1("XP each")].join(" "),
        vec![
            source.c1("Raw"),
            source.c2(&commas(raw as f64, "d")),
            source.c1("Cooked"),
            source.c2(&commas(cooked as f64, "d")),
            source.p(&format!("-{} tax", commas(tax(cooked) as f64, "d"))),
        ]
        .join(" "),
    ]
    .join(&source.c1(" | "));

    let mut lines = vec![format!("{} {}", prefix, header)];

    if cook.level < fish.level {
        lines.push(format!(
            "{} {}",
            prefix,
            source.c1(&format!("Requires {} Cooking", fish.level))
        ));

        return lines;
    }

    let rates: Vec<String> = setups(fish)
        .iter()
        .map(|(name, stop)| {
            let burnt = burn(cook.level, fish.level, *stop);

            format!(
                "{} {} {}{}",
                source.c1(name),
                source.c2(&burn_label(burnt)),
                gp(hourly(raw, cooked, burnt).profit),
                source.c1("/hr")
            )
        })
        .collect();

    lines.push(format!("{} {}", prefix, rates.join(&source.c1(" | "))));

    let (best_name, best_stop) = best_setup(fish);
    let goal = goal(
        cook.listing.xp(),
        cook.level,
        cook.listing.next_level(flags),
    );

    let mut progress = vec![goal_string(&goal, source)];

    if goal != Goal::Maxed {
        let target_xp = cook.listing.xp().saturating_add(goal.remaining());

        if let Some(count) = fish_between(cook.listing.xp(), target_xp, fish, best_stop) {
            let hours = count as f64 / FISH_PER_HOUR as f64;
            let hour = hourly(raw, cooked, burn(cook.level, fish.level, best_stop));
            // Per fish, not per hour - an integer division by the hourly rate
            // first would throw away most of the margin on a cheap fish.
            let total = (hour.profit as f64 / FISH_PER_HOUR as f64 * count as f64).round() as i64;

            progress.push(
                vec![
                    source.c2(&commas(count as f64, "d")),
                    source.c1(&fish.name.to_lowercase()),
                ]
                .join(" "),
            );
            progress.push(source.c2(&format!("~{}", format_hours(hours))));
            progress.push(
                vec![
                    gp(total),
                    source.p(&format!("{}, {}/hr", best_name, commas(FISH_PER_HOUR as f64, "d"))),
                ]
                .join(" "),
            );
        }
    }

    lines.push(format!(
        "{} {}",
        prefix,
        progress.join(&source.c1(" | "))
    ));

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::level_to_xp;
    use crate::fish::find_fish;

    #[test]
    fn burning_stops_at_and_above_the_stop_level() {
        assert_eq!(burn(74, 40, Stop::Level(74)), 0.0);
        assert_eq!(burn(99, 40, Stop::Level(74)), 0.0);
    }

    #[test]
    fn a_no_burn_setup_never_burns() {
        assert_eq!(burn(40, 40, Stop::NoBurn), 0.0);
        assert_eq!(burn(1, 40, Stop::NoBurn), 0.0);
    }

    #[test]
    fn burning_is_worst_at_the_cooking_level() {
        assert_eq!(burn(40, 40, Stop::Level(74)), MAX_BURN);
        // Below the cooking level reads as the cooking level rather than
        // running past 100%: you cannot cook it there at all.
        assert_eq!(burn(1, 40, Stop::Level(74)), MAX_BURN);
    }

    #[test]
    fn burning_falls_linearly_across_the_window() {
        // Halfway from 40 to 74 is 57, so half of MAX_BURN.
        assert!((burn(57, 40, Stop::Level(74)) - MAX_BURN / 2.0).abs() < 0.001);
    }

    #[test]
    fn a_setup_that_never_stops_interpolates_towards_100() {
        // Shark on a fire: 80 to a notional 100, so level 85 is 3/4 of the way
        // up the window and burns a quarter under MAX_BURN.
        let burnt = burn(85, 80, Stop::Never);

        assert!((burnt - 0.375).abs() < 0.001, "got {}", burnt);
        // Still burning at 99, which is what "-" means on the wiki.
        assert!(burn(99, 80, Stop::Never) > 0.0);
    }

    #[test]
    fn tax_is_two_percent_rounded_down() {
        assert_eq!(tax(991), 19);
        assert_eq!(tax(100), 2);
        assert_eq!(tax(50), 1);
    }

    #[test]
    fn tax_below_fifty_gp_rounds_away_to_nothing() {
        assert_eq!(tax(49), 0);
        assert_eq!(tax(1), 0);
        assert_eq!(tax(0), 0);
    }

    #[test]
    fn tax_is_capped_at_five_million() {
        assert_eq!(tax(250_000_000), 5_000_000);
        assert_eq!(tax(1_000_000_000), 5_000_000);
    }

    #[test]
    fn an_hour_with_no_burning_is_the_taxed_margin() {
        // Shark at the wiki's quoted prices: 991 sells for 972 after tax.
        let hour = hourly(732, 991, 0.0);

        assert_eq!(hour.cost, 1_300 * 732);
        assert_eq!(hour.revenue, 1_300 * 972);
        assert_eq!(hour.profit, 1_300 * 240);
    }

    #[test]
    fn burnt_fish_cost_a_raw_fish_and_return_nothing() {
        let hour = hourly(732, 991, 0.5);

        // Cost is unchanged - you still bought every fish.
        assert_eq!(hour.cost, 1_300 * 732);
        // Revenue halves.
        assert_eq!(hour.revenue, 1_300 * 972 / 2);
        assert!(hour.profit < 0, "expected a loss, got {}", hour.profit);
    }

    #[test]
    fn enough_burning_turns_a_profit_into_a_loss() {
        let clean = hourly(732, 991, 0.0);
        let burning = hourly(732, 991, 0.375);

        assert!(clean.profit > 0);
        assert_eq!(burning.profit, -161_850);
    }

    #[test]
    fn no_fish_are_needed_when_the_target_is_already_met() {
        let shark = find_fish("shark").expect("shark is in the table");
        let xp = level_to_xp(90);

        assert_eq!(fish_between(xp, xp, shark, Stop::NoBurn), Some(0));
        assert_eq!(fish_between(xp, xp - 1, shark, Stop::NoBurn), Some(0));
    }

    #[test]
    fn fish_needed_rounds_up() {
        let shark = find_fish("shark").expect("shark is in the table");
        let xp = level_to_xp(90);

        // 210 XP each with no burning.
        assert_eq!(fish_between(xp, xp + 420, shark, Stop::NoBurn), Some(2));
        assert_eq!(fish_between(xp, xp + 421, shark, Stop::NoBurn), Some(3));
    }

    #[test]
    fn burning_costs_extra_fish() {
        let shark = find_fish("shark").expect("shark is in the table");
        let xp = level_to_xp(85);
        let target = xp + 100_000;

        let clean = fish_between(xp, target, shark, Stop::NoBurn).expect("cookable");
        let burning = fish_between(xp, target, shark, Stop::Never).expect("cookable");

        assert!(
            burning > clean,
            "burning {} should need more than clean {}",
            burning,
            clean
        );
    }

    #[test]
    fn fish_are_unavailable_below_the_cooking_level() {
        let shark = find_fish("shark").expect("shark is in the table");
        let xp = level_to_xp(70);

        assert_eq!(fish_between(xp, level_to_xp(71), shark, Stop::NoBurn), None);
    }

    #[test]
    fn a_gauntlet_fish_reports_four_setups() {
        let shark = find_fish("shark").expect("shark is in the table");
        let names: Vec<&str> = setups(shark).iter().map(|(name, _)| *name).collect();

        assert_eq!(names, vec!["Fire", "Range", "Gauntlets", "Hosidius"]);
        assert_eq!(setups(shark)[2].1, Stop::Level(94));
        assert_eq!(setups(shark)[3].1, Stop::Level(84));
    }

    #[test]
    fn a_fish_gauntlets_do_not_affect_omits_the_gauntlet_setup() {
        let karambwan = find_fish("karambwan").expect("karambwan is in the table");
        let names: Vec<&str> = setups(karambwan).iter().map(|(name, _)| *name).collect();

        // Repeating the range figure under a "Gauntlets" label would imply the
        // gauntlets are doing something.
        assert_eq!(names, vec!["Fire", "Range", "Hosidius"]);
        assert_eq!(setups(karambwan)[2].1, Stop::Level(87));
    }

    #[test]
    fn virtual_levels_above_99_do_not_reduce_burn() {
        // Virtual level 126 (200M XP) should burn at the same rate as level 99.
        let rate_99 = burn(99, 80, Stop::Never);
        let rate_126 = burn(126, 80, Stop::Never);

        assert_eq!(rate_99, rate_126);
        assert!(rate_99 > 0.0, "shark on fire should still burn at level 99");
    }

    #[test]
    fn stop_level_fish_cap_at_real_level_99_with_virtual_levels() {
        // Karambwan fire burns until level 99, so a virtual level 105 should
        // still return 0.0 (no burn above the stop level).
        assert_eq!(burn(105, 50, Stop::Level(99)), 0.0);
        // Verify the level just below still burns.
        assert!(burn(98, 50, Stop::Level(99)) > 0.0);
    }

    #[test]
    fn profit_is_green_and_loss_is_red() {
        assert_eq!(gp(312_000), format!("{}312.0k", GREEN));
        // 161_850 / 1000.0 is not exactly representable in f64 (it lands at
        // 161.849999999999994...), so it formats to one decimal as 161.8k,
        // not the 161.9k a decimal reading of -161,850 would suggest.
        assert_eq!(gp(-161_850), format!("{}-161.8k", RED));
    }

    #[test]
    fn breaking_even_is_not_a_loss() {
        assert_eq!(gp(0), format!("{}0", GREEN));
    }

    #[test]
    fn a_burn_percentage_is_marked_as_an_estimate() {
        assert_eq!(burn_label(0.375), "~38%");
        assert_eq!(burn_label(0.321), "~32%");
    }

    #[test]
    fn a_setup_that_never_burns_is_not_marked_as_an_estimate() {
        // 0% is exact - it comes from the wiki's stop-burn table, not the model.
        assert_eq!(burn_label(0.0), "0%");
    }

    #[test]
    fn whole_xp_prints_as_an_integer() {
        // Shark: 210.0 XP each - printing "210.0" would be a made-up decimal.
        assert_eq!(xp_label(210.0), "210");
    }

    #[test]
    fn fractional_xp_keeps_its_decimal() {
        // Sea turtle and Manta ray are the only fish with fractional XP;
        // truncating to "211" or "216" would print a number that is not the
        // one in the table.
        assert_eq!(xp_label(211.3), "211.3");
        assert_eq!(xp_label(216.3), "216.3");
    }
}
