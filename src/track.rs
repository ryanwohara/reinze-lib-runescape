use anyhow::Result;
use common::snapshot;
use common::source::Source;
use log::error;

use crate::common::{
    HiscoreName, Listing, Listings, fetch_hiscores_raw, parse_hiscores_raw, resolve_rsn,
};
use crate::stats::{StatsFlags, stats_parameters, strip_stats_parameters};

pub struct Change {
    pub name: HiscoreName,
    pub is_skill: bool,
    pub old_level: u32,
    pub new_level: u32,
    pub old_xp: u32,
    pub new_xp: u32,
}

pub fn diff_listings(old: &Listings, new: &Listings) -> Vec<Change> {
    let mut changes = vec![];

    for new_listing in new.iter() {
        let name = new_listing.name();
        if name == HiscoreName::None {
            continue;
        }

        if let Some(old_listing) = old.skill(&name.to_string()) {
            let (old_level, new_level, old_xp, new_xp, is_skill) = match (new_listing, &old_listing)
            {
                (Listing::Entry(n), Listing::Entry(o)) => (o.level, n.level, o.xp, n.xp, true),
                (Listing::SubEntry(n), Listing::SubEntry(o)) => (o.xp, n.xp, o.xp, n.xp, false),
                _ => continue,
            };

            if old_xp != new_xp || old_level != new_level {
                changes.push(Change {
                    name,
                    is_skill,
                    old_level,
                    new_level,
                    old_xp,
                    new_xp,
                });
            }
        }
    }

    changes
}

fn format_xp_delta(delta: u32) -> String {
    if delta >= 1_000_000 {
        format!("{:.1}m", delta as f64 / 1_000_000.0)
    } else if delta >= 1_000 {
        format!("{:.1}k", delta as f64 / 1_000.0)
    } else {
        format!("{}", delta)
    }
}

fn format_single_change(c: &Change, source: &Source) -> String {
    if c.is_skill {
        let xp_delta = c.new_xp.saturating_sub(c.old_xp);
        if c.old_level != c.new_level {
            format!(
                "{} {}→{} (+{} XP)",
                source.c1(&c.name.to_string()),
                c.old_level,
                c.new_level,
                format_xp_delta(xp_delta)
            )
        } else {
            format!(
                "{} +{} XP",
                source.c1(&c.name.to_string()),
                format_xp_delta(xp_delta)
            )
        }
    } else {
        let delta = c.new_level.saturating_sub(c.old_level);
        format!(
            "{} {}→{} (+{})",
            source.c1(&c.name.to_string()),
            c.old_level,
            c.new_level,
            delta
        )
    }
}

pub fn format_changes(
    changes: &[Change],
    source: &Source,
    rsn: &str,
    duration_str: &str,
) -> Vec<String> {
    let display_rsn = rsn.replace("_", " ");

    if changes.is_empty() {
        return vec![format!(
            "{} {} {}: {}",
            source.l("Track"),
            source.p(&display_rsn),
            source.c2(&format!("({})", duration_str)),
            source.c1("No changes")
        )];
    }

    let skill_parts: Vec<String> = changes
        .iter()
        .filter(|c| c.is_skill)
        .map(|c| format_single_change(c, source))
        .collect();

    let activity_parts: Vec<String> = changes
        .iter()
        .filter(|c| !c.is_skill)
        .map(|c| format_single_change(c, source))
        .collect();

    let mut parts = skill_parts;
    parts.extend(activity_parts);

    vec![format!(
        "{} {} {}: {}",
        source.l("Track"),
        source.p(&display_rsn),
        source.c2(&format!("({})", duration_str)),
        parts.join(&source.c2(" | "))
    )]
}

pub fn lookup(source: Source) -> Result<Vec<String>> {
    let query = source.query.clone();
    let flags = stats_parameters(&query);
    let cleaned = strip_stats_parameters(&query);

    let rsn = resolve_rsn(cleaned.trim(), &source);
    let mode = flags.account_type.mode();

    let live_raw = fetch_hiscores_raw(&rsn, &flags)?;
    let live_listings = parse_hiscores_raw(&live_raw);

    let _ = snapshot::save_snapshot("osrs", mode, &rsn, &live_raw);

    let (old_raw, duration_str) = if flags.search.is_empty() {
        // No @duration — use most recent snapshot
        match snapshot::get_latest_snapshot("osrs", mode, &rsn)? {
            Some(data) => (data, "latest".to_string()),
            None => {
                return Ok(vec![format!(
                    "{} {}",
                    source.l("Track"),
                    source.c1(&format!(
                        "No snapshot found for {}",
                        rsn.replace("_", " ")
                    ))
                )]);
            }
        }
    } else {
        let hours = match snapshot::parse_duration(&flags.search) {
            Ok(h) => h,
            Err(_) => {
                return Ok(vec![format!(
                    "{} {}",
                    source.l("Track"),
                    source.c1(&format!(
                        "Invalid duration '{}'. Use e.g. @3d, @1w, @12h, @2w3d",
                        flags.search
                    ))
                )]);
            }
        };
        match snapshot::get_snapshot("osrs", mode, &rsn, hours)? {
            Some(data) => (data, flags.search.clone()),
            None => {
                return Ok(vec![format!(
                    "{} {}",
                    source.l("Track"),
                    source.c1(&format!(
                        "No snapshot found for {} within {}",
                        rsn.replace("_", " "),
                        flags.search
                    ))
                )]);
            }
        }
    };

    let old_listings = parse_hiscores_raw(&old_raw);
    let changes = diff_listings(&old_listings, &live_listings);
    Ok(format_changes(&changes, &source, &rsn, &duration_str))
}

/// Called by the bot timer system every 6h.
pub fn snapshot_all() -> Result<Vec<String>> {
    let rsns = snapshot::get_tracked_players("osrs")?;
    let flags = StatsFlags::default();
    let mut count = 0;

    for rsn in &rsns {
        match fetch_hiscores_raw(rsn, &flags) {
            Ok(raw) => {
                let _ = snapshot::save_snapshot("osrs", "main", rsn, &raw);
                count += 1;
            }
            Err(e) => {
                error!("Failed to snapshot {}: {}", rsn, e);
            }
        }
    }

    Ok(vec![format!(
        "Snapshotted {}/{} players",
        count,
        rsns.len()
    )])
}
