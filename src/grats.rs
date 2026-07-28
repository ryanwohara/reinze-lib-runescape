extern crate common;

use crate::common::{eval_query, skill as get_skill};
use crate::grats::tiers::Tier;
use anyhow::Result;
use common::source::Source;
use rand::Rng;
use regex::Regex;

mod tiers;

pub fn get(s: &Source) -> Result<Vec<String>> {
    let mut split = s.query.split_whitespace();

    let first_token = split.next().unwrap_or_default();
    let second_token = split.next().unwrap_or_default();
    let third_token = split.next().unwrap_or_default();

    let mut nick = s.author.nick.to_string();
    let mut milestone = first_token.to_string();
    let mut skill_token = second_token.to_string();

    let err = Ok(vec!["Syntax: +congrats [nick] (level) (skill)".to_string()]);

    if !third_token.is_empty() {
        nick = first_token.to_string();
        milestone = second_token.to_string();
        skill_token = third_token.to_string();
    } else if second_token.is_empty() {
        return err;
    }

    let skill_name = match combat_skill(&skill_token) {
        Some(name) => name.to_string(),
        None => {
            let mut resolved = get_skill(&skill_token);
            if resolved.is_empty() {
                resolved = rs3_skill(&skill_token);
            }
            resolved
        }
    };

    if skill_name.is_empty() {
        return err;
    }

    let skill = &skill_name;

    let re = Regex::new(r"^([\d.]+)[kmb]?$").unwrap();
    if !re.is_match(&milestone) {
        return err;
    }

    let milestone_value = eval_query(&milestone.replace(",", ""))
        .map_err(|e| anyhow::anyhow!("Failed to evaluate milestone: {}", e))?
        as u64;
    let comma_milestone = common::commas(milestone_value as f64, "d");

    if skill_name == "Combat" && !(4..=126).contains(&milestone_value) {
        return Ok(vec!["Combat level must be between 4 and 126".to_string()]);
    }

    // Overall carries two ladders — total level below the derived maximum,
    // total XP above it. Everything else switches from levels to XP at 150.
    let (tier, is_xp) = if skill == "Overall" {
        if milestone_value <= tiers::max_total_level() {
            (tiers::overall_level_tier(milestone_value), false)
        } else {
            (tiers::overall_xp_tier(milestone_value), true)
        }
    } else if milestone_value <= 150 {
        (tiers::level_tier(milestone_value as u32), false)
    } else {
        (tiers::xp_tier(milestone_value), true)
    };

    let value = if is_xp {
        format!("{} {} xp", comma_milestone, skill)
    } else {
        format!("{} {}", comma_milestone, skill)
    };

    Ok(vec![render(s, tier, &nick, &value)])
}

/// Splices `value` into the variant's single `{}`, colouring prose with c1 and
/// the value with c2. The value is wrapped as one unit so its digits and skill
/// name are never separated by a colour control byte. Nick and emoji stay
/// uncoloured — the former so client nick-highlighting still fires.
fn render_variant(s: &Source, emoji: &str, variant: &str, nick: &str, value: &str) -> String {
    let (head, tail) = variant.split_once("{}").unwrap_or((variant, ""));

    let mut out = format!("{}: {} ", nick, emoji);

    if !head.is_empty() {
        out.push_str(&s.c1(head));
    }
    out.push_str(&s.c2(value));
    if !tail.is_empty() {
        out.push_str(&s.c1(tail));
    }

    out
}

/// Picks one of the tier's variants at random and renders it. Randomness is
/// isolated here so the tier tables and render_variant stay deterministic.
fn render(s: &Source, tier: &Tier, nick: &str, value: &str) -> String {
    let index = rand::rng().random_range(0..tier.variants.len());

    render_variant(s, tier.emoji, tier.variants[index], nick, value)
}

fn rs3_skill(s: &str) -> String {
    match s.to_lowercase().as_str() {
        "archaeology" | "arch" => "Archaeology".to_string(),
        "invention" | "inv" | "invent" => "Invention".to_string(),
        "divination" | "div" => "Divination".to_string(),
        "summoning" | "sum" | "summon" => "Summoning".to_string(),
        _ => String::new(),
    }
}

/// Recognises the combat-level triggers for +gz. Case-insensitive.
fn combat_skill(token: &str) -> Option<&'static str> {
    match token.to_lowercase().as_str() {
        "combat" | "cmb" | "cmbt" => Some("Combat"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::ColorResult;
    use common::author::Author;
    use std::os::raw::c_char;

    extern "C" fn stub_color(_host: *const c_char, _colors: *const c_char) -> ColorResult {
        ColorResult::default()
    }

    fn source_with(query: &str) -> Source {
        Source::create(
            "0",
            Author::create("nick!ident@host", stub_color),
            "gz",
            query,
        )
    }

    #[test]
    fn render_variant_splices_value_into_placeholder() {
        let s = source_with("");
        let out = render_variant(&s, "🏆", "Grats on {}! You are a CHAMPION!", "bob", "70 Attack");

        assert!(out.starts_with("bob: 🏆 "), "got: {}", out);
        // The value must survive as one contiguous run — no control byte splitting
        // "70" from "Attack".
        assert!(out.contains("70 Attack"), "got: {}", out);
        assert!(out.contains("Grats on "), "got: {}", out);
        assert!(out.contains("! You are a CHAMPION!"), "got: {}", out);
    }

    #[test]
    fn render_variant_handles_leading_placeholder() {
        let s = source_with("");
        let out = render_variant(&s, "💎", "{}. That is rare air.", "bob", "92 Mining");

        assert!(out.starts_with("bob: 💎 "), "got: {}", out);
        assert!(out.contains("92 Mining"), "got: {}", out);
        assert!(out.contains(". That is rare air."), "got: {}", out);
        // No empty colour-wrapped prose segment before the value.
        assert!(!out.contains("\u{3}\u{3}"), "got: {:?}", out);
    }

    #[test]
    fn render_variant_handles_trailing_placeholder() {
        let s = source_with("");
        let out = render_variant(&s, "❌", "Absolutely not, {}", "bob", "999 Attack");

        assert!(out.contains("Absolutely not, "), "got: {}", out);
        assert!(out.contains("999 Attack"), "got: {}", out);
        assert!(!out.ends_with("\u{3}"), "got: {:?}", out);
    }

    #[test]
    fn render_picks_a_variant_from_the_tier() {
        let s = source_with("");
        let tier = tiers::level_tier(70);

        // Call enough times that a broken index would almost certainly panic or
        // produce something off-tier.
        for _ in 0..50 {
            let out = render(&s, tier, "bob", "70 Attack");
            assert!(out.starts_with("bob: 🏆 "), "got: {}", out);
            assert!(out.contains("70 Attack"), "got: {}", out);
        }
    }

    #[test]
    fn combat_skill_recognises_triggers() {
        assert_eq!(combat_skill("combat"), Some("Combat"));
        assert_eq!(combat_skill("cmb"), Some("Combat"));
        assert_eq!(combat_skill("cmbt"), Some("Combat"));
        assert_eq!(combat_skill("CMB"), Some("Combat"));
        assert_eq!(combat_skill("Combat"), Some("Combat"));
    }

    #[test]
    fn combat_skill_rejects_non_combat() {
        assert_eq!(combat_skill("attack"), None);
        assert_eq!(combat_skill("overall"), None);
        assert_eq!(combat_skill(""), None);
    }

    #[test]
    fn gz_combat_low_level() {
        let out = get(&source_with("4 combat")).unwrap();
        assert!(out[0].contains("4 Combat"), "got: {}", out[0]);
    }

    #[test]
    fn gz_combat_max_level() {
        let out = get(&source_with("126 cmb")).unwrap();
        assert!(out[0].contains("126 Combat"), "got: {}", out[0]);
        assert!(out[0].contains("🌌"), "got: {}", out[0]);
    }

    #[test]
    fn gz_combat_mid_tier_uses_ladder() {
        // 70 lands in the 70-79 tier — proves a combat level flows through the
        // reused level ladder labelled "Combat", not just the end tiers.
        let out = get(&source_with("70 combat")).unwrap();
        assert!(out[0].contains("70 Combat"), "got: {}", out[0]);
        assert!(out[0].contains("🏆"), "got: {}", out[0]);
    }

    #[test]
    fn gz_level_uses_the_level_ladder_emoji() {
        let out = get(&source_with("70 attack")).unwrap();
        assert!(out[0].contains("🏆"), "got: {}", out[0]);
        assert!(out[0].contains("70 Attack"), "got: {}", out[0]);
    }

    #[test]
    fn gz_exact_99_xp_gets_the_cape_tier() {
        let out = get(&source_with("13034431 slayer")).unwrap();
        assert!(out[0].contains("🎓"), "got: {}", out[0]);
        assert!(out[0].contains("13,034,431 Slayer xp"), "got: {}", out[0]);
    }

    #[test]
    fn gz_level_99_gets_the_cape_tier() {
        let out = get(&source_with("99 attack")).unwrap();
        assert!(out[0].contains("🎓"), "got: {}", out[0]);
        // A level, so no "xp" suffix.
        assert!(!out[0].contains(" xp"), "got: {}", out[0]);
    }

    #[test]
    fn gz_200m_is_maxed_not_impossible() {
        let out = get(&source_with("200m runecraft")).unwrap();
        assert!(out[0].contains("🌌"), "got: {}", out[0]);
        assert!(out[0].contains("200,000,000 Runecraft xp"), "got: {}", out[0]);
    }

    #[test]
    fn gz_above_200m_is_rejected() {
        let out = get(&source_with("250m runecraft")).unwrap();
        assert!(out[0].contains("❌"), "got: {}", out[0]);
    }

    #[test]
    fn gz_overall_routes_to_the_total_level_ladder() {
        let out = get(&source_with("2376 overall")).unwrap();
        assert!(out[0].contains("🌌"), "got: {}", out[0]);
        assert!(out[0].contains("2,376 Overall"), "got: {}", out[0]);
        assert!(!out[0].contains(" xp"), "got: {}", out[0]);
    }

    #[test]
    fn gz_overall_routes_to_the_total_xp_ladder_above_max_level() {
        // 2377 is one past max total level, so it must be read as XP.
        let out = get(&source_with("2377 overall")).unwrap();
        assert!(out[0].contains("2,377 Overall xp"), "got: {}", out[0]);
    }

    #[test]
    fn gz_overall_max_xp_no_longer_overflows() {
        // 4.8b exceeds u32::MAX — this returned "not even a thing" before the
        // u64 migration.
        let out = get(&source_with("4800m overall")).unwrap();
        assert!(out[0].contains("🌌"), "got: {}", out[0]);
        assert!(!out[0].contains("❌"), "got: {}", out[0]);
    }

    #[test]
    fn gz_combat_still_resolves_before_overall() {
        // common::skill() maps "combat"/"cmb" to "Overall". combat_skill() must
        // win, or this becomes an Overall grats at total level 99.
        let out = get(&source_with("99 cmb")).unwrap();
        assert!(out[0].contains("99 Combat"), "got: {}", out[0]);
    }

    #[test]
    fn gz_combat_below_range_errors() {
        let out = get(&source_with("3 cmb")).unwrap();
        assert_eq!(out[0], "Combat level must be between 4 and 126");
    }

    #[test]
    fn gz_combat_above_range_errors() {
        let out = get(&source_with("127 cmbt")).unwrap();
        assert_eq!(out[0], "Combat level must be between 4 and 126");
    }

    #[test]
    fn gz_non_combat_skill_unaffected() {
        let out = get(&source_with("99 attack")).unwrap();
        assert!(out[0].contains("99 Attack"), "got: {}", out[0]);
    }
}
