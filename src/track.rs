use anyhow::Result;
use common::snapshot;
use common::source::Source;
use log::error;

use crate::common::{
    HiscoreName, Listing, Listings, fetch_hiscores_raw, parse_hiscores_raw, parse_snapshot_data,
    resolve_rsn, short_xp, skill, to_snapshot_data,
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

/// One `^name` token's resolution outcome, kept in the order typed so the
/// output columns match what the caller asked for.
#[derive(Debug, PartialEq)]
pub enum Requested {
    Row(HiscoreName),
    Unmatched(String),
}

/// Resolve a `^name` token to a hiscore row.
///
/// The alias table comes first because no row's display name *contains* the
/// substring "mine" — `^mine`, `^att` and `^cmb` only work through it. The
/// substring pass then covers activities (`^zulrah`, `^cox`, `^clue`).
///
/// `HiscoreName::None` is a safe miss sentinel: its `Display` is the empty
/// string, and `""` never contains a non-empty needle, so the substring pass
/// can never select it by accident.
fn resolve_name(token: &str) -> HiscoreName {
    let aliased = skill(token);

    if aliased.is_empty() {
        HiscoreName::from(token)
    } else {
        HiscoreName::from(aliased.as_str())
    }
}

/// Resolve every token, dropping repeats so `^mining ^mine` yields one column,
/// while preserving the order typed.
fn resolve_requested(tokens: &[String]) -> Vec<Requested> {
    let mut requested: Vec<Requested> = Vec::new();

    for token in tokens {
        let resolved = match resolve_name(token) {
            HiscoreName::None => Requested::Unmatched(token.clone()),
            name => Requested::Row(name),
        };

        if !requested.contains(&resolved) {
            requested.push(resolved);
        }
    }

    requested
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
                short_xp(xp_delta as f64)
            )
        } else {
            format!(
                "{} +{} XP",
                source.c1(&c.name.to_string()),
                short_xp(xp_delta as f64)
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

/// Maximum byte length of a single emitted IRC line. Kept below the bot's
/// 400-byte send-splitter (see `process_message` in rust-reinze) so that lines
/// are never blind-chopped mid-segment, which would orphan a fragment like
/// `1031→1205 (+174)` onto its own prefix-less line.
pub(crate) const MAX_LINE_LEN: usize = 400;

/// Greedily pack pre-formatted `parts` into lines that each start with `prefix`
/// and join their segments with `sep`, keeping every line's byte length at or
/// below `max_len`. Each returned line is self-contained (carries the prefix),
/// so a split never produces a meaningless fragment. A single part that cannot
/// fit even on its own line is emitted anyway rather than dropped.
pub(crate) fn pack_lines(prefix: &str, parts: &[String], sep: &str, max_len: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut has_part = false;

    for part in parts {
        // Bytes a fresh line costs for this part: prefix + " " + part.
        // Bytes appending to the current line costs: sep + part.
        let append_cost = sep.len() + part.len();
        if has_part && current.len() + append_cost > max_len {
            lines.push(std::mem::take(&mut current));
            has_part = false;
        }

        if has_part {
            current.push_str(sep);
            current.push_str(part);
        } else {
            current = format!("{} {}", prefix, part);
            has_part = true;
        }
    }

    if has_part {
        lines.push(current);
    }

    lines
}

/// `[Track] (1d):` — the self-contained prefix every emitted line carries, so a
/// line split never produces a meaningless fragment.
fn track_prefix(source: &Source, duration_str: &str) -> String {
    format!(
        "{} {}:",
        source.l("Track"),
        source.c2(&format!("({})", duration_str))
    )
}

pub fn format_changes(changes: &[Change], source: &Source, duration_str: &str) -> Vec<String> {
    let prefix = track_prefix(source, duration_str);

    if changes.is_empty() {
        return vec![format!("{} {}", prefix, source.c1("No changes"))];
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

    pack_lines(&prefix, &parts, &source.c2(" | "), MAX_LINE_LEN)
}

/// A requested row that did not change still has to report something, so it
/// falls back to its live standing with an explicit zero delta. Jagex reports
/// unranked rows as `-1`, which the `u32` parse floors to 0 — that is what
/// distinguishes "unranked" from a genuine level 1 / score 0.
fn format_current_standing(listing: &Listing, source: &Source) -> String {
    let name = source.c1(&listing.name().to_string());

    match listing {
        Listing::Entry(e) if e.level == 0 => format!("{} {}", name, source.c1("Unranked")),
        Listing::Entry(e) => format!("{} {} ({} XP) +0 XP", name, e.level, short_xp(e.xp as f64)),
        Listing::SubEntry(s) if s.xp == 0 => format!("{} {}", name, source.c1("Unranked")),
        Listing::SubEntry(s) => format!("{} {} +0", name, s.xp),
    }
}

/// One segment per requested row, in the order typed. Unlike the unfiltered
/// path this never collapses to a bare "No changes" — each requested row
/// reports its own state, and an unresolvable token is flagged in place rather
/// than failing the whole lookup.
pub fn format_requested(
    requested: &[Requested],
    changes: &[Change],
    live: &Listings,
    source: &Source,
    duration_str: &str,
) -> Vec<String> {
    let parts: Vec<String> = requested
        .iter()
        .map(|r| match r {
            Requested::Unmatched(token) => source.c1(&format!("no match for '{}'", token)),
            Requested::Row(name) => match changes.iter().find(|c| c.name == *name) {
                Some(change) => format_single_change(change, source),
                None => match live.skill(&name.to_string()) {
                    Some(listing) => format_current_standing(&listing, source),
                    None => format!("{} {}", source.c1(&name.to_string()), source.c1("No data")),
                },
            },
        })
        .collect();

    pack_lines(
        &track_prefix(source, duration_str),
        &parts,
        &source.c2(" | "),
        MAX_LINE_LEN,
    )
}

/// Every `^name` token was a typo, so there is nothing to diff. Reported under a
/// duration-less prefix, matching the shape of the other early returns in
/// `lookup`.
fn format_all_unmatched(requested: &[Requested], source: &Source) -> Vec<String> {
    let parts: Vec<String> = requested
        .iter()
        .filter_map(|r| match r {
            Requested::Unmatched(token) => Some(source.c1(&format!("no match for '{}'", token))),
            Requested::Row(_) => None,
        })
        .collect();

    pack_lines(&source.l("Track"), &parts, &source.c2(" | "), MAX_LINE_LEN)
}

pub fn lookup(source: Source) -> Result<Vec<String>> {
    let query = source.query.clone();
    let flags = stats_parameters(&query);
    let cleaned = strip_stats_parameters(&query);
    let requested = resolve_requested(&flags.names);

    // Every `^name` was a typo, so there is nothing worth diffing. Bail before
    // the rsn lookup and the hiscores fetch so a typo costs no round trip and
    // records no snapshot.
    if !requested.is_empty()
        && requested
            .iter()
            .all(|r| matches!(r, Requested::Unmatched(_)))
    {
        return Ok(format_all_unmatched(&requested, &source));
    }

    let rsn = resolve_rsn(cleaned.trim(), &source);
    let mode = flags.account_type.mode();

    let live_raw = fetch_hiscores_raw(&rsn, &flags)?;
    let live_listings = parse_hiscores_raw(&live_raw);

    // Resolve the comparison baseline BEFORE recording the current snapshot.
    // Otherwise the no-duration "latest" lookup just returns the snapshot we are
    // about to save (identical to live), so every result is "No changes".
    let baseline: Option<(String, String)> = if flags.search.is_empty() {
        match snapshot::get_latest_snapshot("osrs", mode, &rsn) {
            Ok(opt) => opt.map(|data| (data, "latest".to_string())),
            Err(e) => {
                return Ok(vec![format!(
                    "{} {}",
                    source.l("Track"),
                    source.c1(&format!("Snapshot lookup failed: {}", e))
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
        match snapshot::get_snapshot("osrs", mode, &rsn, hours) {
            Ok(opt) => opt.map(|data| (data, flags.search.clone())),
            Err(e) => {
                return Ok(vec![format!(
                    "{} {}",
                    source.l("Track"),
                    source.c1(&format!("Snapshot lookup failed: {}", e))
                )]);
            }
        }
    };

    // Record the current snapshot for future comparisons (also bootstraps the
    // first-ever lookup for a player, which has no baseline yet).
    let _ = snapshot::save_snapshot("osrs", mode, &rsn, &to_snapshot_data(&live_raw));

    let (old_raw, duration_str) = match baseline {
        Some(b) => b,
        None => {
            let scope = if flags.search.is_empty() {
                String::new()
            } else {
                format!(" within {}", flags.search)
            };
            return Ok(vec![format!(
                "{} {}",
                source.l("Track"),
                source.c1(&format!(
                    "No snapshot found for {}{}",
                    rsn.replace("_", " "),
                    scope
                ))
            )]);
        }
    };

    let old_listings = match parse_snapshot_data(&old_raw) {
        Ok(listings) => listings,
        Err(e) => {
            return Ok(vec![format!(
                "{} {}",
                source.l("Track"),
                source.c1(&format!(
                    "Can't compare against the {} snapshot: {}",
                    duration_str, e
                ))
            )]);
        }
    };
    let changes = diff_listings(&old_listings, &live_listings);

    if requested.is_empty() {
        Ok(format_changes(&changes, &source, &duration_str))
    } else {
        Ok(format_requested(
            &requested,
            &changes,
            &live_listings,
            &source,
            &duration_str,
        ))
    }
}

/// Called by the bot timer system every 6h.
pub fn snapshot_all() -> Result<Vec<String>> {
    let rsns = snapshot::get_tracked_players("osrs")?;
    let flags = StatsFlags::default();
    let mut count = 0;

    for rsn in &rsns {
        match fetch_hiscores_raw(rsn, &flags) {
            Ok(raw) => {
                let _ = snapshot::save_snapshot("osrs", "main", rsn, &to_snapshot_data(&raw));
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::{Entry, SubEntry};
    use ::common::ColorResult;
    use ::common::author::Author;
    use regex::Regex;
    use std::os::raw::c_char;

    extern "C" fn stub_color(_host: *const c_char, _colors: *const c_char) -> ColorResult {
        ColorResult::default()
    }

    fn stub_source() -> Source {
        Source::create(
            "0",
            Author::create("nick!ident@host", stub_color),
            "track",
            "",
        )
    }

    /// Strip IRC colour codes so assertions read as plain text.
    fn plain(s: &str) -> String {
        Regex::new(r"\x03\d{0,2}")
            .unwrap()
            .replace_all(s, "")
            .to_string()
    }

    fn entry(name: HiscoreName, level: u32, xp: u32) -> Listing {
        Listing::Entry(Entry {
            name,
            rank: 1,
            level,
            xp,
        })
    }

    fn sub(name: HiscoreName, score: u32) -> Listing {
        Listing::SubEntry(SubEntry {
            name,
            rank: 1,
            xp: score,
        })
    }

    fn skill_change(name: HiscoreName, old_level: u32, new_level: u32, gained: u32) -> Change {
        Change {
            name,
            is_skill: true,
            old_level,
            new_level,
            old_xp: 2_000_000,
            new_xp: 2_000_000 + gained,
        }
    }

    fn parts(n: usize, each: &str) -> Vec<String> {
        (0..n).map(|_| each.to_string()).collect()
    }

    #[test]
    fn single_line_when_everything_fits() {
        let lines = pack_lines("[Track] (dra) (1d):", &parts(3, "Attack +1k"), " | ", 400);
        assert_eq!(lines.len(), 1);
        assert_eq!(
            lines[0],
            "[Track] (dra) (1d): Attack +1k | Attack +1k | Attack +1k"
        );
    }

    #[test]
    fn every_line_carries_the_prefix() {
        let prefix = "[Track] (dra) (1d):";
        // Force several lines with a tight budget.
        let lines = pack_lines(prefix, &parts(10, "Brutus 1031->1205 (+174)"), " | ", 60);
        assert!(lines.len() > 1, "expected multiple lines");
        for line in &lines {
            assert!(line.starts_with(prefix), "line is self-contained: {line:?}");
            assert!(line.len() <= 60, "line within budget: {line:?}");
        }
    }

    #[test]
    fn no_segment_is_orphaned_or_dropped() {
        let prefix = "P:";
        let input = vec![
            "Overall +303k".to_string(),
            "Attack +31k".to_string(),
            "Brutus 1031->1205 (+174)".to_string(),
        ];
        let lines = pack_lines(prefix, &input, " | ", 24);
        // Reconstruct the segments from the emitted lines and confirm none were
        // lost or split mid-segment (the original blind-split bug).
        let mut seen: Vec<String> = Vec::new();
        for line in &lines {
            let body = line.strip_prefix(&format!("{prefix} ")).expect("prefixed");
            for seg in body.split(" | ") {
                seen.push(seg.to_string());
            }
        }
        assert_eq!(seen, input);
    }

    #[test]
    fn empty_parts_yields_no_lines() {
        let lines = pack_lines("P:", &[], " | ", 400);
        assert!(lines.is_empty());
    }

    #[test]
    fn oversized_single_part_is_kept_not_dropped() {
        let big = "x".repeat(100);
        let lines = pack_lines("P:", &[big.clone()], " | ", 20);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains(&big));
    }

    #[test]
    fn aliases_resolve_through_the_skill_table() {
        // No row's display name contains "mine", so these only work via the
        // alias table, not the substring pass.
        assert_eq!(resolve_name("mine"), HiscoreName::Mining);
        assert_eq!(resolve_name("att"), HiscoreName::Attack);
        assert_eq!(resolve_name("cmb"), HiscoreName::Overall);
    }

    #[test]
    fn activities_resolve_by_substring() {
        assert_eq!(resolve_name("zulrah"), HiscoreName::Zulrah);
        assert_eq!(resolve_name("cox"), HiscoreName::CoX);
        assert_eq!(resolve_name("clue"), HiscoreName::ClueScrollAll);
    }

    #[test]
    fn an_unknown_token_is_a_miss_not_a_row() {
        assert_eq!(resolve_name("asdf"), HiscoreName::None);
    }

    #[test]
    fn repeated_tokens_collapse_to_one_column() {
        let requested = resolve_requested(&["mining".to_string(), "mine".to_string()]);
        assert_eq!(requested, vec![Requested::Row(HiscoreName::Mining)]);
    }

    #[test]
    fn resolution_preserves_the_order_typed() {
        let requested = resolve_requested(&["fishing".to_string(), "mining".to_string()]);
        assert_eq!(
            requested,
            vec![
                Requested::Row(HiscoreName::Fishing),
                Requested::Row(HiscoreName::Mining),
            ]
        );
    }

    #[test]
    fn misses_are_kept_in_place_alongside_hits() {
        let requested =
            resolve_requested(&["mining".to_string(), "asdf".to_string(), "asdf".to_string()]);
        assert_eq!(
            requested,
            vec![
                Requested::Row(HiscoreName::Mining),
                Requested::Unmatched("asdf".to_string()),
            ]
        );
    }

    #[test]
    fn a_changed_skill_reports_its_delta() {
        let live = Listings::new(vec![entry(HiscoreName::Mining, 83, 2_031_000)]);
        let out = format_requested(
            &[Requested::Row(HiscoreName::Mining)],
            &[skill_change(HiscoreName::Mining, 82, 83, 31_000)],
            &live,
            &stub_source(),
            "1d",
        );
        assert_eq!(plain(&out[0]), "[Track] (1d): Mining 82→83 (+31.0k XP)");
    }

    #[test]
    fn a_changed_activity_reports_its_score_delta() {
        // `diff_listings` stores an activity's score in every level/xp field, so
        // the Change for a boss carries scores, not levels.
        let change = Change {
            name: HiscoreName::Zulrah,
            is_skill: false,
            old_level: 1031,
            new_level: 1205,
            old_xp: 1031,
            new_xp: 1205,
        };
        let live = Listings::new(vec![sub(HiscoreName::Zulrah, 1205)]);
        let out = format_requested(
            &[Requested::Row(HiscoreName::Zulrah)],
            &[change],
            &live,
            &stub_source(),
            "1d",
        );
        assert_eq!(plain(&out[0]), "[Track] (1d): Zulrah 1031→1205 (+174)");
    }

    #[test]
    fn an_unchanged_skill_reports_its_current_standing() {
        let live = Listings::new(vec![entry(HiscoreName::Mining, 82, 13_034_431)]);
        let out = format_requested(
            &[Requested::Row(HiscoreName::Mining)],
            &[],
            &live,
            &stub_source(),
            "1d",
        );
        assert_eq!(plain(&out[0]), "[Track] (1d): Mining 82 (13.0m XP) +0 XP");
    }

    #[test]
    fn an_unchanged_activity_reports_its_score() {
        let live = Listings::new(vec![sub(HiscoreName::Zulrah, 1205)]);
        let out = format_requested(
            &[Requested::Row(HiscoreName::Zulrah)],
            &[],
            &live,
            &stub_source(),
            "1w",
        );
        assert_eq!(plain(&out[0]), "[Track] (1w): Zulrah 1205 +0");
    }

    #[test]
    fn an_unranked_row_says_so_instead_of_showing_zeroes() {
        // Jagex reports unranked rows as `-1`, which the u32 parse floors to 0.
        let live = Listings::new(vec![
            entry(HiscoreName::Mining, 0, 0),
            sub(HiscoreName::Zulrah, 0),
        ]);
        let out = format_requested(
            &[
                Requested::Row(HiscoreName::Mining),
                Requested::Row(HiscoreName::Zulrah),
            ],
            &[],
            &live,
            &stub_source(),
            "1d",
        );
        assert_eq!(
            plain(&out[0]),
            "[Track] (1d): Mining Unranked | Zulrah Unranked"
        );
    }

    #[test]
    fn a_row_absent_from_the_response_says_no_data() {
        let out = format_requested(
            &[Requested::Row(HiscoreName::Zulrah)],
            &[],
            &Listings::new(vec![]),
            &stub_source(),
            "1d",
        );
        assert_eq!(plain(&out[0]), "[Track] (1d): Zulrah No data");
    }

    #[test]
    fn a_typo_keeps_the_good_column_and_flags_the_bad_token() {
        let live = Listings::new(vec![entry(HiscoreName::Mining, 82, 2_031_000)]);
        let out = format_requested(
            &[
                Requested::Row(HiscoreName::Mining),
                Requested::Unmatched("asdf".to_string()),
            ],
            &[skill_change(HiscoreName::Mining, 82, 82, 31_000)],
            &live,
            &stub_source(),
            "1d",
        );
        assert_eq!(
            plain(&out[0]),
            "[Track] (1d): Mining +31.0k XP | no match for 'asdf'"
        );
    }

    #[test]
    fn all_tokens_bad_reports_the_misses_without_a_duration() {
        let requested = resolve_requested(&["asdf".to_string(), "qwer".to_string()]);
        let out = format_all_unmatched(&requested, &stub_source());
        assert_eq!(
            plain(&out[0]),
            "[Track] no match for 'asdf' | no match for 'qwer'"
        );
    }

    #[test]
    fn the_unfiltered_path_still_collapses_to_no_changes() {
        let out = format_changes(&[], &stub_source(), "1d");
        assert_eq!(plain(&out[0]), "[Track] (1d): No changes");
    }
}
