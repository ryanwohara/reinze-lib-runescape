use anyhow::Result;
use common::source::Source;
use regex::Regex;

use crate::common::{
    MAX_SKILL_XP, fetch_hiscores_raw, parse_hiscores_raw, resolve_rsn, short_xp, xp_to_level,
};
use crate::npc::data::{Npc, NpcMetadata};
use crate::stats::StatsFlags;
use crate::stats::skill::Skill;

#[derive(Debug, PartialEq)]
pub struct TaskQuery {
    pub rsn_input: String,
    pub count: u32,
    pub monster: String,
}

/// Split a `[rsn] @<count> <monster>` query. Text before the `@<digits>`
/// token is the (optional) rsn, everything after it is the monster name.
fn parse_task_query(query: &str) -> Option<TaskQuery> {
    let re = Regex::new(r"^(.*?)@(\d+)\s+(\S.*)$").unwrap();
    let caps = re.captures(query.trim())?;

    let count: u32 = caps[2].parse().ok()?;
    if count == 0 {
        return None;
    }

    Some(TaskQuery {
        rsn_input: caps[1].trim().to_string(),
        count,
        monster: caps[3].trim().to_string(),
    })
}

/// Total XP for `count` kills, in display order, omitting zero-value skills.
fn task_totals(
    slayer_xp: f64,
    combat_xp: f64,
    hitpoints_xp: f64,
    count: u32,
) -> Vec<(f64, &'static str)> {
    let count = count as f64;

    [
        (slayer_xp * count, "Slayer XP"),
        (combat_xp * count, "Combat XP"),
        (hitpoints_xp * count, "HP XP"),
    ]
    .into_iter()
    .filter(|(total, _)| *total > 0.0)
    .collect()
}

/// Where `gained` slayer XP lands from `current_xp`: (new xp, new level),
/// clamped to the 200m XP cap and level 99.
fn projected_slayer(current_xp: u32, gained: f64) -> (u32, u32) {
    let new_xp = (current_xp as f64 + gained).min(MAX_SKILL_XP as f64) as u32;
    (new_xp, xp_to_level(new_xp).min(99))
}

pub fn lookup(source: Source) -> Result<Vec<String>> {
    let prefix = source.l("Task");

    let task = match parse_task_query(&source.query) {
        Some(task) => task,
        None => {
            return Ok(vec![format!(
                "{} {}",
                prefix,
                source.c1("Usage: +task [rsn] @<count> <monster>")
            )]);
        }
    };

    let result = Npc::lookup(&task.monster);
    if result == Npc::None {
        let suggestions = Npc::search(&task.monster)
            .iter()
            .map(|npc| NpcMetadata::from(npc).name)
            .take(10)
            .collect::<Vec<String>>();

        return Ok(vec![if suggestions.is_empty() {
            format!(
                "{} {}",
                prefix,
                source.c1(&format!("No NPC named '{}'", task.monster))
            )
        } else {
            format!(
                "{} {}",
                prefix,
                source.c1(&format!(
                    "No exact match for '{}' — did you mean: {}",
                    task.monster,
                    suggestions.join(", ")
                ))
            )
        }]);
    }

    let npc = NpcMetadata::from(&result);
    let totals = task_totals(npc.slayer_xp, npc.combat_xp, npc.hitpoints_xp, task.count);
    if totals.is_empty() {
        return Ok(vec![format!(
            "{} {}",
            prefix,
            source.c1(&format!("No XP data for {}", npc.name))
        )]);
    }

    let mut parts: Vec<String> = totals
        .iter()
        .map(|(total, label)| format!("{} {}", source.c2(&short_xp(*total)), source.c1(label)))
        .collect();

    // Project the slayer XP onto the player's hiscores; degrade to the plain
    // calculator when the player can't be resolved or has no Slayer entry.
    if npc.slayer_xp > 0.0 {
        let rsn = resolve_rsn(&task.rsn_input, &source);
        if let Ok(raw) = fetch_hiscores_raw(&rsn, &StatsFlags::default()) {
            let listings = parse_hiscores_raw(&raw);
            if let Some(entry) = listings.skill("Slayer") {
                if entry.xp() > 0 {
                    let (new_xp, new_level) =
                        projected_slayer(entry.xp(), npc.slayer_xp * task.count as f64);
                    parts.push(format!(
                        "{} {}→{} {}",
                        source.c1("Slayer"),
                        entry.level(),
                        new_level,
                        source.p(&format!(
                            "{}→{}",
                            short_xp(entry.xp() as f64),
                            short_xp(new_xp as f64)
                        ))
                    ));
                }
            }
        }
    }

    Ok(vec![format!(
        "{} {} {} {}: {}",
        prefix,
        source.c2(&task.count.to_string()),
        source.c1("x"),
        source.c2(&npc.name),
        parts.join(&source.c2(" | "))
    )])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::level_to_xp;

    #[test]
    fn parses_rsn_count_and_multiword_monster() {
        let q = parse_task_query("dra @500 abyssal demon").unwrap();
        assert_eq!(q.rsn_input, "dra");
        assert_eq!(q.count, 500);
        assert_eq!(q.monster, "abyssal demon");
    }

    #[test]
    fn parses_with_rsn_omitted() {
        let q = parse_task_query("@25 vardorvis").unwrap();
        assert_eq!(q.rsn_input, "");
        assert_eq!(q.count, 25);
        assert_eq!(q.monster, "vardorvis");
    }

    #[test]
    fn tolerates_extra_whitespace() {
        let q = parse_task_query("  dra   @10   kalphite queen  ").unwrap();
        assert_eq!(q.rsn_input, "dra");
        assert_eq!(q.count, 10);
        assert_eq!(q.monster, "kalphite queen");
    }

    #[test]
    fn rejects_zero_count_missing_monster_and_garbage() {
        assert!(parse_task_query("@0 zulrah").is_none());
        assert!(parse_task_query("zulrah").is_none());
        assert!(parse_task_query("@12").is_none());
        assert!(parse_task_query("").is_none());
    }

    #[test]
    fn totals_multiply_and_keep_display_order() {
        let totals = task_totals(210.0, 4200.0, 1400.0, 500);
        assert_eq!(
            totals,
            vec![
                (105_000.0, "Slayer XP"),
                (2_100_000.0, "Combat XP"),
                (700_000.0, "HP XP"),
            ]
        );
    }

    #[test]
    fn totals_omit_zero_value_segments() {
        let totals = task_totals(0.0, 4.0, 1.5, 10);
        assert_eq!(totals, vec![(40.0, "Combat XP"), (15.0, "HP XP")]);
    }

    #[test]
    fn projection_crosses_level_boundary() {
        let start = level_to_xp(85);
        let gained = (level_to_xp(86) - level_to_xp(85)) as f64;
        let (new_xp, new_level) = projected_slayer(start, gained);
        assert_eq!(new_xp, level_to_xp(86));
        assert_eq!(new_level, 86);
    }

    #[test]
    fn projection_clamps_at_xp_cap_and_level_99() {
        let (new_xp, new_level) = projected_slayer(199_999_999, 50_000_000.0);
        assert_eq!(new_xp, 200_000_000);
        assert_eq!(new_level, 99);
    }
}
