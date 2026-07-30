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
