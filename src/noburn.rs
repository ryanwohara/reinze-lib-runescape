use anyhow::Result;
use common::source::Source;

use crate::fish::{FISH, Fish, Stop};

/// One stop-burn cell. A level prints as itself; the two "no level" cases are
/// opposites and must not look alike - `Never` means the fish still burns at
/// 99, `NoBurn` means that setup never burns it at any level.
fn render(stop: Stop) -> String {
    match stop {
        Stop::Level(level) => level.to_string(),
        Stop::Never => "N/A".to_string(),
        Stop::NoBurn => "any".to_string(),
    }
}

/// The three gauntlet columns, or three dashes for the fish gauntlets do not
/// affect.
fn gauntlet_columns(fish: &Fish) -> Vec<String> {
    match &fish.gauntlets {
        Some(gauntlets) => vec![
            render(gauntlets.default),
            render(gauntlets.hosidius5),
            render(gauntlets.hosidius10),
        ],
        None => vec!["-".to_string(), "-".to_string(), "-".to_string()],
    }
}

/// An empty query matches everything; anything else is a case-insensitive
/// substring of the fish's name.
fn matches(fish: &Fish, query: &str) -> bool {
    query.is_empty() || fish.name.to_lowercase().contains(&query.to_lowercase())
}

fn row(fish: &Fish, s: &Source) -> String {
    let gauntlets = gauntlet_columns(fish);

    format!(
        "{} {} {} {} {} {}",
        s.c1(fish.name),
        s.c2(&render(fish.fire)),
        s.c2(&render(fish.range)),
        s.c2(&render(fish.hosidius5)),
        s.c2(&render(fish.hosidius10)),
        s.p(&gauntlets.join(" ")),
    )
}

pub fn noburn(s: &Source) -> Result<Vec<String>> {
    let query = s.query.trim();

    let output: Vec<String> = FISH
        .iter()
        .filter(|fish| matches(fish, query))
        .map(|fish| row(fish, s))
        .collect();

    Ok(vec![
        format!("{} {}", s.l("NoBurn"), s.not_found(output)),
        s.p(
            "Fire | Range | Hosidius 5% | Hosidius 10% | (Gauntlets | Gauntlets + Hosidius 5% | Gauntlets + Hosidius 10%)",
        ),
        s.p("N/A = still burns at 99 | any = never burns | - = gauntlets don't apply"),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fish::{FISH, Stop, find_fish};

    #[test]
    fn a_stop_level_renders_as_its_number() {
        assert_eq!(render(Stop::Level(74)), "74");
        assert_eq!(render(Stop::Level(99)), "99");
    }

    #[test]
    fn never_and_no_burn_render_differently() {
        // They are opposites, so they must not both print "N/A".
        assert_eq!(render(Stop::Never), "N/A");
        assert_eq!(render(Stop::NoBurn), "any");
        assert_ne!(render(Stop::Never), render(Stop::NoBurn));
    }

    #[test]
    fn a_fish_without_gauntlets_renders_them_as_not_applicable() {
        let karambwan = find_fish("karambwan").expect("karambwan is in the table");
        let columns = gauntlet_columns(karambwan);

        assert_eq!(columns, vec!["-", "-", "-"]);
    }

    #[test]
    fn a_gauntlet_fish_renders_all_three_gauntlet_columns() {
        let shark = find_fish("shark").expect("shark is in the table");

        assert_eq!(gauntlet_columns(shark), vec!["94", "89", "84"]);
    }

    #[test]
    fn an_empty_query_matches_every_fish() {
        let matched: Vec<&str> = FISH
            .iter()
            .filter(|fish| matches(fish, ""))
            .map(|fish| fish.name)
            .collect();

        assert_eq!(matched.len(), FISH.len());
    }

    #[test]
    fn a_query_narrows_to_the_named_fish() {
        let matched: Vec<&str> = FISH
            .iter()
            .filter(|fish| matches(fish, "shark"))
            .map(|fish| fish.name)
            .collect();

        assert_eq!(matched, vec!["Shark"]);
    }

    #[test]
    fn a_query_is_case_insensitive_and_can_be_partial() {
        let matched: Vec<&str> = FISH
            .iter()
            .filter(|fish| matches(fish, "CRAB"))
            .map(|fish| fish.name)
            .collect();

        assert_eq!(matched, vec!["Dark crab"]);
    }

    #[test]
    fn the_gauntlet_not_applicable_mark_differs_from_never() {
        // "-" is for gauntlets that don't apply; "N/A" means still burns at 99.
        // They must never be the same string.
        assert_ne!(
            gauntlet_columns(&find_fish("karambwan").expect("karambwan exists"))[0],
            render(Stop::Never)
        );
    }

    #[test]
    fn a_query_that_matches_no_fish_reports_not_found() {
        use ::common::{ColorResult, author::Author};
        use common::source::Source;
        use std::os::raw::c_char;

        extern "C" fn stub_color(_host: *const c_char, _colors: *const c_char) -> ColorResult {
            ColorResult::default()
        }

        let s = Source::create("0", Author::create("test!test@test", stub_color), "noburn", "zzz");

        let result = noburn(&s).expect("should not error on no-match");
        assert_eq!(result.len(), 3); // three lines: data, header, legend
        assert!(result[0].contains("Not found"));
    }
}
