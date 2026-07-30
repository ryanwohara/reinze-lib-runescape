# `+chef` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `+chef` command that ranks every tradeable raw→cooked fish by profit per hour at live Grand Exchange prices, and details one fish with a burn-rate spread across four cooking setups plus a `+herbi`-style goal block.

**Architecture:** A new `src/fish.rs` owns the one fish table — names, cooking level, XP, and stop-burn levels — which both the existing `+noburn` and the new `+chef` read. `src/chef.rs` layers price, burn, tax and XP maths over it. Three display/price helpers that `degrime.rs` and `herbi.rs` already have privately move to `src/common.rs` so `chef.rs` shares rather than copies them.

**Tech Stack:** Rust 2024 edition, `anyhow` for errors, the workspace-local `common` crate for IRC colouring (`Source`), no new dependencies.

**Design spec:** `docs/superpowers/specs/2026-07-30-chef-command-design.md`

## Global Constraints

- **No new dependencies.** Everything needed is already in `Cargo.toml`.
- **`cargo test` must pass after every task.** The suite is 128 tests today and runs in ~5s.
- **Burn figures are estimates.** Every burn percentage, and every gp or fish count derived from one, is printed with a leading `~`.
- **`MAX_BURN = 0.50`** is the single tunable burn anchor. It appears exactly once, in `chef.rs`, with a comment saying it is a modelling choice and not a wiki figure.
- **`FISH_PER_HOUR = 1_300`** — the wiki money-making guide's default cook rate.
- **GE tax is 2%**, rounded down per item, capped at 5,000,000 gp. Computed with integer maths, never floating point.
- **Colour discipline:** `source.l` for the command prefix, `c1` for labels, `c2` for values, `p` for parentheticals. The only raw colour codes in the codebase are `chef.rs`'s `GREEN`/`RED` for signed gp.
- **Wiki stop-burn levels are upstream truth.** Where `noburn.rs` disagrees with the table in this plan, this plan wins.
- **Commit after every task** using the repo's `type(scope): subject` convention, with the `Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>` trailer.

## File Structure

| File | Responsibility |
|---|---|
| `src/fish.rs` (create) | The fish table: `Stop`, `Gauntlets`, `Fish`, `FISH`, `find_fish`. Pure data plus lookup. No formatting, no prices. |
| `src/noburn.rs` (rewrite) | Renders `FISH` as the `+noburn` table. Display only. |
| `src/common.rs` (modify) | Gains `short_gp`, `format_hours`, `price_of` — shared display/price helpers. |
| `src/degrime.rs` (modify) | Drops its private `short_gp` and `price_of`, imports them instead. |
| `src/herbi.rs` (modify) | Drops its private `format_hours`, imports it instead. |
| `src/chef.rs` (create) | Burn, tax, hourly profit, fish-to-goal maths, and the two output modes. |
| `src/lib.rs` (modify) | `mod fish;`, `mod chef;`, the `^chef$` trigger, dispatch, help text, trigger test. |
| `README.md` (modify) | `-chef` entry under Calculators. |

---

### Task 1: The shared fish table

**Files:**
- Create: `src/fish.rs`
- Modify: `src/lib.rs` (add `mod fish;` to the module list, alphabetically after `mod fairy;`)

**Interfaces:**
- Consumes: nothing.
- Produces: `pub enum Stop { Level(u32), Never, NoBurn }`; `pub struct Gauntlets { pub default: Stop, pub hosidius5: Stop, pub hosidius10: Stop }`; `pub struct Fish { pub name, pub raw, pub cooked: &'static str, pub level: u32, pub xp: f64, pub fire, pub range, pub hosidius5, pub hosidius10: Stop, pub gauntlets: Option<Gauntlets> }`; `pub const FISH: [Fish; 11]`; `pub fn find_fish(query: &str) -> Option<&'static Fish>`.

**Background the implementer needs:** the wiki prints three different things in these columns and they mean different things. A number is the level burning stops. A dash means burning *never* stops below 99 (sharks on an open fire). `N/A` in a Hosidius or gauntlet column means the opposite — you never burn it at all with that setup, because the level would fall below the elite Kourend & Kebos diary's own level 84 requirement. `N/A` in the Range column of the non-gauntlet table means only that a range is no better than a fire, so the fire value is copied across.

- [ ] **Step 1: Write the failing test**

Create `src/fish.rs` containing only this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_fish_has_a_raw_and_a_cooked_item() {
        for fish in FISH.iter() {
            assert!(!fish.name.is_empty(), "a fish has no name");
            assert!(
                fish.raw.starts_with("Raw "),
                "{} has a raw item that is not a 'Raw ...' item: {}",
                fish.name,
                fish.raw
            );
            assert!(!fish.cooked.is_empty(), "{} has no cooked item", fish.name);
            assert!(fish.xp > 0.0, "{} has no xp", fish.name);
            assert!(fish.level >= 1, "{} has no cooking level", fish.name);
        }

        let names: Vec<&str> = FISH.iter().map(|fish| fish.name).collect();
        let mut unique = names.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(names.len(), unique.len(), "duplicate fish in the table");
    }

    #[test]
    fn the_table_is_ordered_by_cooking_level() {
        for pair in FISH.windows(2) {
            assert!(
                pair[0].level <= pair[1].level,
                "{} (level {}) is listed after {} (level {})",
                pair[1].name,
                pair[1].level,
                pair[0].name,
                pair[0].level
            );
        }
    }

    #[test]
    fn only_the_five_gauntlet_fish_carry_gauntlet_levels() {
        let with: Vec<&str> = FISH
            .iter()
            .filter(|fish| fish.gauntlets.is_some())
            .map(|fish| fish.name)
            .collect();

        assert_eq!(
            with,
            vec!["Lobster", "Swordfish", "Monkfish", "Shark", "Anglerfish"]
        );
    }

    #[test]
    fn fish_are_found_by_exact_prefix_or_substring_name() {
        assert_eq!(find_fish("Shark").map(|f| f.name), Some("Shark"));
        assert_eq!(find_fish("shark").map(|f| f.name), Some("Shark"));
        assert_eq!(find_fish("SHARK").map(|f| f.name), Some("Shark"));
        // Prefix.
        assert_eq!(find_fish("angler").map(|f| f.name), Some("Anglerfish"));
        assert_eq!(find_fish("monk").map(|f| f.name), Some("Monkfish"));
        // Substring, which is how the two-word fish are reachable: the `@`
        // flag captures a single token, so `@sea turtle` cannot be typed.
        assert_eq!(find_fish("turtle").map(|f| f.name), Some("Sea turtle"));
        assert_eq!(find_fish("crab").map(|f| f.name), Some("Dark crab"));
        assert_eq!(find_fish("manta").map(|f| f.name), Some("Manta ray"));
    }

    #[test]
    fn an_exact_name_wins_over_a_longer_match() {
        // "Bass" is also a substring of nothing else today, but the ladder has
        // to hold if a "Bass pie" is ever added.
        assert_eq!(find_fish("bass").map(|f| f.name), Some("Bass"));
        assert_eq!(find_fish("  Tuna  ").map(|f| f.name), Some("Tuna"));
    }

    #[test]
    fn unknown_and_empty_queries_are_not_found() {
        assert!(find_fish("lobstre").is_none());
        assert!(find_fish("nature rune").is_none());
        assert!(find_fish("").is_none());
        assert!(find_fish("   ").is_none());
    }

    #[test]
    fn karambwan_stops_burning_on_a_fire_and_ignores_gauntlets() {
        let karambwan = find_fish("karambwan").expect("karambwan is in the table");

        assert_eq!(karambwan.fire, Stop::Level(99));
        assert_eq!(karambwan.hosidius10, Stop::Level(87));
        assert!(karambwan.gauntlets.is_none());
    }

    #[test]
    fn a_dash_and_an_n_a_are_opposite_outcomes() {
        let shark = find_fish("shark").expect("shark is in the table");
        // The wiki prints "-" here: sharks burn on a fire at every level.
        assert_eq!(shark.fire, Stop::Never);

        let lobster = find_fish("lobster").expect("lobster is in the table");
        // The wiki prints "N/A" here: with 10% favour lobsters never burn.
        assert_eq!(lobster.hosidius10, Stop::NoBurn);
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --lib fish::`
Expected: FAIL — the module is not declared and `FISH`, `Stop`, `find_fish` do not exist. You will first need `mod fish;` in `src/lib.rs` for the file to compile at all; add it now, then the failures become "cannot find value `FISH`".

- [ ] **Step 3: Write the implementation**

Put this above the test module in `src/fish.rs`:

```rust
/// The Cooking level at which one setup stops burning a fish.
///
/// The wiki's burn tables use three different notations here and two of them
/// look alike but mean opposite things, so they get distinct variants rather
/// than a shared "N/A" sentinel.
/// https://oldschool.runescape.wiki/w/Cooking/Burn_level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stop {
    /// Burning stops at this level. The wiki prints the number.
    Level(u32),
    /// Burning never stops below 99. The wiki prints "-".
    Never,
    /// The setup never burns the fish at any level. The wiki prints "N/A",
    /// because the level would fall below the elite Kourend & Kebos diary's
    /// own level 84 requirement and so is unreachable.
    NoBurn,
}

/// Stop-burn levels while wearing cooking gauntlets. Only the five fish the
/// gauntlets actually affect carry these.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gauntlets {
    pub default: Stop,
    pub hosidius5: Stop,
    pub hosidius10: Stop,
}

/// One fish that can be bought raw, cooked, and sold.
pub struct Fish {
    pub name: &'static str,
    pub raw: &'static str,
    pub cooked: &'static str,
    /// Cooking level needed to cook it at all.
    pub level: u32,
    /// Cooking XP for one successful cook. Burning earns nothing.
    pub xp: f64,
    pub fire: Stop,
    /// A normal range. Where the wiki prints "N/A" here it means a range is no
    /// better than a fire, so the fire level is copied across.
    pub range: Stop,
    pub hosidius5: Stop,
    pub hosidius10: Stop,
    /// `None` for the fish cooking gauntlets do not affect.
    pub gauntlets: Option<Gauntlets>,
}

/// Every fish with both a raw and a cooked Grand Exchange item, in cooking
/// level order. Levels, XP and stop-burn levels are the wiki's.
pub const FISH: [Fish; 11] = [
    Fish {
        name: "Tuna",
        raw: "Raw tuna",
        cooked: "Tuna",
        level: 30,
        xp: 100.0,
        fire: Stop::Level(63),
        range: Stop::Level(63),
        hosidius5: Stop::Level(59),
        hosidius10: Stop::NoBurn,
        gauntlets: None,
    },
    Fish {
        name: "Karambwan",
        raw: "Raw karambwan",
        cooked: "Cooked karambwan",
        level: 30,
        xp: 190.0,
        fire: Stop::Level(99),
        range: Stop::Level(99),
        hosidius5: Stop::Level(93),
        hosidius10: Stop::Level(87),
        gauntlets: None,
    },
    Fish {
        name: "Lobster",
        raw: "Raw lobster",
        cooked: "Lobster",
        level: 40,
        xp: 120.0,
        fire: Stop::Level(74),
        range: Stop::Level(74),
        hosidius5: Stop::Level(70),
        hosidius10: Stop::NoBurn,
        gauntlets: Some(Gauntlets {
            default: Stop::Level(64),
            hosidius5: Stop::Level(60),
            hosidius10: Stop::NoBurn,
        }),
    },
    Fish {
        name: "Bass",
        raw: "Raw bass",
        cooked: "Bass",
        level: 43,
        xp: 130.0,
        fire: Stop::Level(79),
        range: Stop::Level(79),
        hosidius5: Stop::Level(75),
        hosidius10: Stop::NoBurn,
        gauntlets: None,
    },
    Fish {
        name: "Swordfish",
        raw: "Raw swordfish",
        cooked: "Swordfish",
        level: 45,
        xp: 140.0,
        fire: Stop::Level(86),
        range: Stop::Level(80),
        hosidius5: Stop::Level(76),
        hosidius10: Stop::NoBurn,
        gauntlets: Some(Gauntlets {
            default: Stop::Level(80),
            hosidius5: Stop::Level(76),
            hosidius10: Stop::NoBurn,
        }),
    },
    Fish {
        name: "Monkfish",
        raw: "Raw monkfish",
        cooked: "Monkfish",
        level: 62,
        xp: 150.0,
        fire: Stop::Level(92),
        range: Stop::Level(90),
        hosidius5: Stop::Level(86),
        hosidius10: Stop::Level(82),
        gauntlets: Some(Gauntlets {
            default: Stop::Level(86),
            hosidius5: Stop::Level(82),
            hosidius10: Stop::NoBurn,
        }),
    },
    Fish {
        name: "Shark",
        raw: "Raw shark",
        cooked: "Shark",
        level: 80,
        xp: 210.0,
        fire: Stop::Never,
        range: Stop::Never,
        hosidius5: Stop::Never,
        hosidius10: Stop::Level(98),
        gauntlets: Some(Gauntlets {
            default: Stop::Level(94),
            hosidius5: Stop::Level(89),
            hosidius10: Stop::Level(84),
        }),
    },
    Fish {
        name: "Sea turtle",
        raw: "Raw sea turtle",
        cooked: "Sea turtle",
        level: 82,
        xp: 211.3,
        fire: Stop::Never,
        range: Stop::Never,
        hosidius5: Stop::Never,
        hosidius10: Stop::Never,
        gauntlets: None,
    },
    Fish {
        name: "Anglerfish",
        raw: "Raw anglerfish",
        cooked: "Anglerfish",
        level: 84,
        xp: 230.0,
        fire: Stop::Never,
        range: Stop::Never,
        hosidius5: Stop::Never,
        hosidius10: Stop::Never,
        gauntlets: Some(Gauntlets {
            default: Stop::Level(97),
            hosidius5: Stop::Level(93),
            hosidius10: Stop::Level(87),
        }),
    },
    Fish {
        name: "Dark crab",
        raw: "Raw dark crab",
        cooked: "Dark crab",
        level: 90,
        xp: 215.0,
        fire: Stop::Never,
        range: Stop::Never,
        hosidius5: Stop::Never,
        hosidius10: Stop::Never,
        gauntlets: None,
    },
    Fish {
        name: "Manta ray",
        raw: "Raw manta ray",
        cooked: "Manta ray",
        level: 91,
        xp: 216.3,
        fire: Stop::Never,
        range: Stop::Never,
        hosidius5: Stop::Never,
        hosidius10: Stop::Never,
        gauntlets: None,
    },
];

/// Match a query against the fish names, ignoring case and surrounding space.
/// Exact wins over a prefix, which wins over a substring - the two-word fish
/// are only reachable by substring, because the `@` flag captures one token.
pub fn find_fish(query: &str) -> Option<&'static Fish> {
    let query = query.trim().to_lowercase();

    if query.is_empty() {
        return None;
    }

    FISH.iter()
        .find(|fish| fish.name.to_lowercase() == query)
        .or_else(|| {
            FISH.iter()
                .find(|fish| fish.name.to_lowercase().starts_with(&query))
        })
        .or_else(|| {
            FISH.iter()
                .find(|fish| fish.name.to_lowercase().contains(&query))
        })
}
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --lib fish::`
Expected: PASS, 8 tests.

Then run the whole suite to be sure nothing else broke: `cargo test`
Expected: 136 passed.

- [ ] **Step 5: Commit**

```bash
git add src/fish.rs src/lib.rs
git commit -m "$(cat <<'EOF'
feat(fish): shared table of cookable fish

One table of every fish with a raw and a cooked GE item, carrying the
cooking level, XP and per-setup stop-burn levels. Stop::Never and
Stop::NoBurn keep apart the two things the wiki's tables mean by a dash
and by N/A: still burning at 99, and never burning at all.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 2: `+noburn` renders the shared table

**Files:**
- Modify: `src/noburn.rs` (replace the whole file)

**Interfaces:**
- Consumes: `crate::fish::{FISH, Fish, Gauntlets, Stop}` from Task 1.
- Produces: no new public API. `pub fn noburn(s: &Source) -> Result<Vec<String>>` keeps its signature.

**What changes for users:** five fish appear that didn't before (karambwan, bass, sea turtle, dark crab, manta ray), seven stale levels are corrected, and the single "N/A" splits into "N/A" (never stops burning) and "any" (never burns with that setup). The trailing legend line gains a note explaining the two.

- [ ] **Step 1: Write the failing test**

Append this test module to `src/noburn.rs`, leaving the current implementation in place for now — Step 3 replaces it:

```rust
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

        assert_eq!(columns, vec!["N/A", "N/A", "N/A"]);
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
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --lib noburn::`
Expected: FAIL — `render`, `gauntlet_columns` and `matches` do not exist.

- [ ] **Step 3: Write the implementation**

Replace everything above the test module in `src/noburn.rs` with:

```rust
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

/// The three gauntlet columns, or three N/As for the fish gauntlets do not
/// affect.
fn gauntlet_columns(fish: &Fish) -> Vec<String> {
    match &fish.gauntlets {
        Some(gauntlets) => vec![
            render(gauntlets.default),
            render(gauntlets.hosidius5),
            render(gauntlets.hosidius10),
        ],
        None => vec!["N/A".to_string(), "N/A".to_string(), "N/A".to_string()],
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
        format!("{} {}", s.l("NoBurn"), output.join(&s.c1(" | "))),
        s.p(
            "Fire | Range | Hosidius 5% | Hosidius 10% | (Gauntlets | Gauntlets + Hosidius 5% | Gauntlets + Hosidius 10%)",
        ),
        s.p("N/A = still burns at 99 | any = never burns"),
    ])
}
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --lib noburn::`
Expected: PASS, 7 tests.

Run: `cargo test`
Expected: 143 passed.

- [ ] **Step 5: Commit**

```bash
git add src/noburn.rs
git commit -m "$(cat <<'EOF'
refactor(noburn): render the shared fish table

-noburn reads src/fish.rs rather than its own copy, which adds karambwan,
bass, sea turtle, dark crab and manta ray, and refreshes seven stop-burn
levels that had drifted from the wiki.

"N/A" previously covered two opposite outcomes. A fish that still burns
at 99 keeps N/A; one that never burns with that setup now prints "any",
and the legend says which is which.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 3: Move the shared display and price helpers into `common.rs`

**Files:**
- Modify: `src/common.rs` (add three functions plus their tests)
- Modify: `src/degrime.rs` (delete `short_gp` and `price_of`, import them, move their tests)
- Modify: `src/herbi.rs` (delete `format_hours`, import it, move its tests)

**Interfaces:**
- Consumes: `crate::items::{Mapping, Price}` (already imported by `common.rs`).
- Produces: `pub fn short_gp(gp: i64) -> String`; `pub fn format_hours(hours: f64) -> String`; `pub fn price_of(items: &[Mapping], ge: &HashMap<u32, Price>, name: &str) -> Option<u32>`.

This is a pure refactor: no behaviour changes, and the moved tests must keep passing verbatim. Do it before `chef.rs` so `chef.rs` never has a copy to delete later.

- [ ] **Step 1: Move the three functions**

Cut `short_gp` and `price_of` from `src/degrime.rs` and `format_hours` from `src/herbi.rs`, and paste them into `src/common.rs` next to `short_xp` (around line 154), changed only from private to `pub`:

```rust
/// GP with a sign, shortened for display: -1,500 -> "-1.5k".
pub fn short_gp(gp: i64) -> String {
    let sign = if gp < 0 { "-" } else { "" };

    format!("{}{}", sign, short_xp(gp.unsigned_abs() as f64))
}

/// Hours for display: under an hour reads in minutes, otherwise one decimal.
pub fn format_hours(hours: f64) -> String {
    if hours < 1.0 {
        return format!("{}min", (hours * 60.0).round());
    }

    format!("{}h", commas(hours, ".1f"))
}

/// Price an item trades at, preferring the buy offer and falling back to the
/// sell offer when nothing has bought recently.
pub fn price_of(items: &[Mapping], ge: &HashMap<u32, Price>, name: &str) -> Option<u32> {
    let id = items
        .iter()
        .find(|item| item.name.eq_ignore_ascii_case(name))?
        .id;

    let price = ge.get(&id)?;

    price.high.or(price.low)
}
```

`common.rs` already imports `commas`, `HashMap`, `Mapping` and `Price`; add whichever are missing to its existing `use` statements rather than adding new ones at the bottom.

- [ ] **Step 2: Update the two callers**

In `src/degrime.rs`, extend the existing import to pull the two functions from `common`, and delete the now-duplicate `use std::collections::HashMap;` if nothing else in the file uses it:

```rust
use crate::common::{get_ge_data, get_item_db, price_of, short_gp, short_xp};
```

`short_xp` stays imported only if the file still uses it directly — after `short_gp` moves out, it does not, so drop it from the list.

In `src/herbi.rs`, extend the existing `crate::common` import with `format_hours`:

```rust
use crate::common::{
    Entry, HiscoreName, Listing, MAX_SKILL_LEVEL, collect_hiscores, format_hours, level_to_xp,
    xp_to_level,
};
```

- [ ] **Step 3: Move the tests with the functions**

Move these three tests out of `degrime.rs`'s and `herbi.rs`'s test modules and into `common.rs`'s test module, unchanged:

```rust
#[test]
fn gp_is_shortened_and_keeps_its_sign() {
    assert_eq!(short_gp(2_400_000), "2.4m");
    assert_eq!(short_gp(453_000), "453.0k");
    assert_eq!(short_gp(-1_500), "-1.5k");
    assert_eq!(short_gp(0), "0");
}

#[test]
fn hours_are_shown_to_one_decimal() {
    assert_eq!(format_hours(4.9), "4.9h");
    assert_eq!(format_hours(1.0), "1.0h");
    assert_eq!(format_hours(1234.5), "1,234.5h");
}

#[test]
fn under_an_hour_is_shown_in_minutes() {
    assert_eq!(format_hours(0.5), "30min");
    assert_eq!(format_hours(0.0), "0min");
}
```

If `common.rs` has no `#[cfg(test)] mod tests` block, add one at the end of the file with `use super::*;`.

- [ ] **Step 4: Run the full suite**

Run: `cargo test`
Expected: 143 passed — the same count as after Task 2, because tests moved rather than multiplied.

Run: `cargo build 2>&1 | grep -i warning`
Expected: no new warnings about unused imports.

- [ ] **Step 5: Commit**

```bash
git add src/common.rs src/degrime.rs src/herbi.rs
git commit -m "$(cat <<'EOF'
refactor(common): share short_gp, format_hours and price_of

All three were private to a single command and are about to be needed by
a second one. Moved to common.rs with their tests; no behaviour change.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 4: The chef maths

**Files:**
- Create: `src/chef.rs`
- Modify: `src/lib.rs` (add `mod chef;` alphabetically, before `mod clues;`)

**Interfaces:**
- Consumes: `crate::fish::{Fish, Stop}` (Task 1); `crate::common::{level_to_xp, xp_to_level, MAX_SKILL_LEVEL}`.
- Produces: `fn burn(level: u32, cook_level: u32, stop: Stop) -> f64`; `fn tax(price: u32) -> u32`; `struct Hourly { pub burn: f64, pub cost: i64, pub revenue: i64, pub profit: i64 }`; `fn hourly(raw: u32, cooked: u32, burn: f64) -> Hourly`; `fn fish_between(xp: u32, target_xp: u32, fish: &Fish, stop: Stop) -> Option<u64>`; `fn setups(fish: &Fish) -> Vec<(&'static str, Stop)>`.

No output code in this task — only the maths, which is what the tests can pin without a network.

- [ ] **Step 1: Write the failing test**

Create `src/chef.rs` with only this test module:

```rust
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
}
```

- [ ] **Step 2: Run the test to verify it fails**

Add `mod chef;` to `src/lib.rs` so the file compiles, then run: `cargo test --lib chef::`
Expected: FAIL — `burn`, `tax`, `hourly`, `fish_between`, `setups`, `MAX_BURN` do not exist.

- [ ] **Step 3: Write the implementation**

Put this above the test module in `src/chef.rs`:

```rust
use crate::common::{MAX_SKILL_LEVEL, level_to_xp, xp_to_level};
use crate::fish::{Fish, Stop};

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
    let level = (level as f64).max(cook);

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
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --lib chef::`
Expected: PASS, 17 tests.

Run: `cargo test`
Expected: 160 passed.

- [ ] **Step 5: Commit**

```bash
git add src/chef.rs src/lib.rs
git commit -m "$(cat <<'EOF'
feat(chef): burn, tax and profit maths

Burn falls linearly from a 50% anchor at a fish's cooking level to zero
where burning stops, interpolating towards a notional 100 for the setups
that never stop. Burnt fish cost a raw fish and return nothing, so they
cut revenue and XP without touching cost.

Sales pay the 2% GE tax, computed in integer maths and capped at 5m.
Fish-to-goal walks level bands rather than single fish so a 200m target
does not iterate millions of times.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

### Task 5: Output, lookup, and wiring

**Files:**
- Modify: `src/chef.rs` (add rendering and `lookup`)
- Modify: `src/lib.rs` (trigger, dispatch, help list, trigger test)
- Modify: `README.md` (entry under Calculators)

**Interfaces:**
- Consumes: everything from Task 4, plus `crate::common::{collect_hiscores, get_ge_data, get_item_db, price_of, short_gp, format_hours, Entry, HiscoreName, Listing}` and `crate::stats::{goal, goal_string, level_display, stats_parameters, strip_stats_parameters, Goal}`.
- Produces: `pub fn lookup(source: Source) -> Result<Vec<String>>`.

`goal`, `goal_string` and `level_display` are `pub(crate)` in `stats.rs`, so they are reachable from `chef.rs` without changing their visibility.

- [ ] **Step 1: Write the failing test**

Add these tests to `src/chef.rs`'s existing test module:

```rust
#[test]
fn profit_is_green_and_loss_is_red() {
    assert_eq!(gp(312_000), format!("{}312.0k", GREEN));
    assert_eq!(gp(-161_850), format!("{}-161.9k", RED));
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
    // 0% is exact - it comes from the wiki's table, not the model.
    assert_eq!(burn_label(0.0), "0%");
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --lib chef::`
Expected: FAIL — `gp`, `GREEN`, `RED` and `burn_label` do not exist.

- [ ] **Step 3: Write the rendering helpers**

Add to `src/chef.rs`, above the test module. Extend the `use` block at the top of the file to:

```rust
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
```

```rust
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
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --lib chef::`
Expected: PASS, 21 tests.

- [ ] **Step 5: Write `lookup`**

Add to `src/chef.rs`, below `fish_between`:

```rust
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
```

- [ ] **Step 6: Write the two output modes**

Add to `src/chef.rs`:

```rust
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
        vec![
            source.c2(&commas(fish.xp, "d")),
            source.c1("XP each"),
        ]
        .join(" "),
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
```

Note `crate::stats::StatsFlags` is already `pub`, and `crate::items::{Mapping, Price}` are `pub` — no visibility changes are needed.

- [ ] **Step 7: Wire the command into `lib.rs`**

Add the trigger to `TRIGGERS`, on its own line after `boss\d*`:

```
^chef$
```

Anchored at both ends because `lib.rs`'s `commands_are_not_dispatched_twice` test requires every command to fire exactly one trigger. Check the anchoring holds by eye: `chef` must not be matched by `(no)?burn`, `co?mb(at)?\d*$`, `^craft(ing)?\d*$` or `clues?\d*`, and `^chef$` must not match any other command name.

Add the dispatch arm immediately after the `"boss" | "bosses" | "kc"` arm:

```rust
"chef" => chef::lookup(source),
```

Add `chef` to the `"help"` list between `boost` and `clues[N]`:

```
chef
```

Add `"chef"` to the `commands_are_not_dispatched_twice` test's list, after `"boosts"`.

- [ ] **Step 8: Run the full suite**

Run: `cargo test`
Expected: 164 passed, 0 failed.

Run: `cargo build --release 2>&1 | tail -5`
Expected: builds clean, no warnings from `chef.rs`, `fish.rs` or `noburn.rs`.

- [ ] **Step 9: Document it in the README**

Add under `### Calculators`, after the `-herbi` entry:

```markdown
- `-chef[N] [@fish] [#goal] [^level] [RSN]` — Cooking profit at live Grand
  Exchange prices. With no `@fish`, every tradeable raw→cooked fish ranked by
  profit per hour; with one, that fish across four setups (fire, range, cooking
  gauntlets, Hosidius) with its burn rate, plus the fish, time and gp between a
  player's Cooking level and their next level or `#goal`. Profit is green and
  losses are red, and both account for the 2% Grand Exchange tax and for burnt
  fish, which cost a raw fish and return nothing. Burn rates are estimates and
  are marked `~`.
```

- [ ] **Step 10: Commit**

```bash
git add src/chef.rs src/lib.rs README.md
git commit -m "$(cat <<'EOF'
feat(chef): +chef cooking profit command

+chef ranks every tradeable raw->cooked fish by profit per hour; +chef
@shark details one across fire, range, gauntlets and Hosidius, with the
fish, time and gp between the player's Cooking level and their goal.

Profit prints green and losses red, since the c1/c2 palette is per-user
configurable and cannot carry that meaning. Revenue is net of the 2% GE
tax and of burnt fish, which cost a raw fish and return nothing.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Manual verification

`cargo test` covers the maths but not the two HTTP-backed paths, so check these by hand once Task 5 is in. They need `lib/ge.json` and `lib/item_db.json` present, which is how the plugin runs in production.

| Command | Expect |
|---|---|
| `+chef` | one or two lines, every fish with a gp figure, most profitable first, `*` on any fish above your level |
| `+chef @shark` | three lines: header, four setups, goal block |
| `+chef @karambwan` | three setups, not four — no gauntlet column |
| `+chef @turtle` | resolves to Sea turtle |
| `+chef @notafish` | the "isn't a cookable fish" line |
| `+chef @shark ^70` | the "Requires 80 Cooking" line, with no hiscore lookup |
| `+chef @shark ^99` | 0% burn at Hosidius, and a green profit |
| `+chef @shark #99` | goal block targeting 99 |
| `+noburn` | eleven fish, and the new legend line distinguishing "N/A" from "any" |
| `+noburn shark` | one row |

## Notes for the implementer

- **`+degrime` is not in the README.** That is pre-existing and out of scope; do not add it as a drive-by.
- **Do not "fix" the burn model.** `MAX_BURN` is deliberately a single constant with a comment saying it is invented. If the numbers look wrong in play, that is a tuning conversation, not a code change to make mid-task.
- **`level_to_xp` takes a `u32` level and returns `u32` XP**; `xp_to_level` is its inverse and saturates at 126. Both are already imported by the files that need them.
- **`commas(value, "d")`** formats an integer with thousands separators and keeps a minus sign; `commas(value, ".1f")` gives one decimal.
- **`pack_lines`** measures bytes including colour codes, so the `GREEN`/`RED` prefixes are accounted for automatically.
