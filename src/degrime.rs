use anyhow::Result;
use common::commas;
use common::source::Source;

use crate::common::{get_ge_data, get_item_db, price_of, short_gp};
use crate::track::{MAX_LINE_LEN, pack_lines};

/// One Degrime cast cleans an inventory of herbs for 2 nature runes (the 4
/// earth runes come from the staff), and 600 casts fit in an hour.
/// https://oldschool.runescape.wiki/w/Money_making_guide/Degriming_grimy_irit_leaf
const HERBS_PER_CAST: u32 = 27;
const NATURE_RUNES_PER_CAST: u32 = 2;
const CASTS_PER_HOUR: u32 = 600;
const HERBS_PER_HOUR: u32 = HERBS_PER_CAST * CASTS_PER_HOUR;
const NATURE_RUNES_PER_HOUR: u32 = NATURE_RUNES_PER_CAST * CASTS_PER_HOUR;
const MAGIC_XP_PER_CAST: f64 = 83.0;
const MAGIC_LEVEL: u32 = 70;

pub struct Herb {
    pub clean: &'static str,
    pub grimy: &'static str,
    /// Herblore level the method needs: the spell's own 50, or the herb's
    /// cleaning level where that is higher.
    pub herblore: u32,
    /// Herblore XP per herb degrimed (half the usual cleaning XP).
    pub xp: f64,
}

/// Every herb with a degriming money making guide on the wiki.
const HERBS: [Herb; 14] = [
    Herb {
        clean: "Guam leaf",
        grimy: "Grimy guam leaf",
        herblore: 50,
        xp: 1.2,
    },
    Herb {
        clean: "Marrentill",
        grimy: "Grimy marrentill",
        herblore: 50,
        xp: 1.9,
    },
    Herb {
        clean: "Tarromin",
        grimy: "Grimy tarromin",
        herblore: 50,
        xp: 2.5,
    },
    Herb {
        clean: "Harralander",
        grimy: "Grimy harralander",
        herblore: 50,
        xp: 3.1,
    },
    Herb {
        clean: "Ranarr weed",
        grimy: "Grimy ranarr weed",
        herblore: 50,
        xp: 3.7,
    },
    Herb {
        clean: "Toadflax",
        grimy: "Grimy toadflax",
        herblore: 50,
        xp: 4.0,
    },
    Herb {
        clean: "Irit leaf",
        grimy: "Grimy irit leaf",
        herblore: 50,
        xp: 4.4,
    },
    Herb {
        clean: "Avantoe",
        grimy: "Grimy avantoe",
        herblore: 50,
        xp: 5.0,
    },
    Herb {
        clean: "Kwuarm",
        grimy: "Grimy kwuarm",
        herblore: 54,
        xp: 5.6,
    },
    Herb {
        clean: "Snapdragon",
        grimy: "Grimy snapdragon",
        herblore: 59,
        xp: 5.9,
    },
    Herb {
        clean: "Cadantine",
        grimy: "Grimy cadantine",
        herblore: 65,
        xp: 6.2,
    },
    Herb {
        clean: "Lantadyme",
        grimy: "Grimy lantadyme",
        herblore: 67,
        xp: 6.5,
    },
    Herb {
        clean: "Dwarf weed",
        grimy: "Grimy dwarf weed",
        herblore: 70,
        xp: 6.9,
    },
    Herb {
        clean: "Torstol",
        grimy: "Grimy torstol",
        herblore: 75,
        xp: 7.5,
    },
];

/// An hour of degriming, in gp. Signed: cleaning can lose money.
#[derive(Debug, PartialEq)]
pub struct Hourly {
    pub cost: i64,
    pub revenue: i64,
    pub profit: i64,
}

fn hourly(grimy_price: u32, clean_price: u32, nature_price: u32) -> Hourly {
    let cost = HERBS_PER_HOUR as i64 * grimy_price as i64
        + NATURE_RUNES_PER_HOUR as i64 * nature_price as i64;
    let revenue = HERBS_PER_HOUR as i64 * clean_price as i64;

    Hourly {
        cost,
        revenue,
        profit: revenue - cost,
    }
}

/// Match a query against the herb names, ignoring case and a `grimy` prefix.
fn find_herb(query: &str) -> Option<&'static Herb> {
    let query = query.trim().to_lowercase();
    let query = query.strip_prefix("grimy ").unwrap_or(&query).trim();

    if query.is_empty() {
        return None;
    }

    HERBS
        .iter()
        .find(|herb| herb.clean.to_lowercase() == query)
        .or_else(|| {
            HERBS
                .iter()
                .find(|herb| herb.clean.to_lowercase().starts_with(query))
        })
}

pub fn lookup(source: &Source) -> Result<Vec<String>> {
    let prefix = source.l("Degrime");
    let items = get_item_db()?;
    let ge = get_ge_data()?;

    let nature_price = match price_of(&items, &ge, "Nature rune") {
        Some(price) => price,
        None => {
            return Ok(vec![format!(
                "{} {}",
                prefix,
                source.c1("No price for nature runes")
            )]);
        }
    };

    let query = source.query.trim();

    if query.is_empty() {
        // Rank every herb so the profitable ones are obvious at a glance.
        let mut ranked: Vec<(&Herb, Hourly)> = HERBS
            .iter()
            .filter_map(|herb| {
                let grimy = price_of(&items, &ge, herb.grimy)?;
                let clean = price_of(&items, &ge, herb.clean)?;

                Some((herb, hourly(grimy, clean, nature_price)))
            })
            .collect();

        if ranked.is_empty() {
            return Ok(vec![format!("{} {}", prefix, source.c1("No herb prices"))]);
        }

        ranked.sort_by(|(_, a), (_, b)| b.profit.cmp(&a.profit));

        let parts: Vec<String> = ranked
            .iter()
            .map(|(herb, hour)| {
                format!(
                    "{} {}",
                    source.c1(herb.clean),
                    source.c2(&short_gp(hour.profit))
                )
            })
            .collect();

        return Ok(pack_lines(
            &format!("{} {}", prefix, source.c1("Profit/hr:")),
            &parts,
            &source.c1(" | "),
            MAX_LINE_LEN,
        ));
    }

    let herb = match find_herb(query) {
        Some(herb) => herb,
        None => {
            return Ok(vec![format!(
                "{} {}",
                prefix,
                source.c1(&format!(
                    "'{}' can't be degrimed - try +degrime for the full list",
                    query
                ))
            )]);
        }
    };

    let (grimy_price, clean_price) = match (
        price_of(&items, &ge, herb.grimy),
        price_of(&items, &ge, herb.clean),
    ) {
        (Some(grimy), Some(clean)) => (grimy, clean),
        _ => {
            return Ok(vec![format!(
                "{} {}",
                prefix,
                source.c1(&format!("No price for {}", herb.clean))
            )]);
        }
    };

    let hour = hourly(grimy_price, clean_price, nature_price);

    // Everything below is per hour: HERBS_PER_HOUR herbs cleaned by
    // CASTS_PER_HOUR casts.
    let parts = vec![
        vec![
            source.c2(herb.clean),
            source.p(&format!(
                "{} @ {}",
                commas(HERBS_PER_HOUR as f64, "d"),
                commas(grimy_price as f64, "d")
            )),
        ]
        .join(" "),
        vec![
            source.c1("Profit/hr"),
            source.c2(&commas(hour.profit as f64, "d")),
        ]
        .join(" "),
        vec![
            source.c1("Cost/hr"),
            source.c2(&commas(hour.cost as f64, "d")),
        ]
        .join(" "),
        vec![
            source.c1("Revenue/hr"),
            source.c2(&commas(hour.revenue as f64, "d")),
        ]
        .join(" "),
        vec![
            source.c2(&commas(herb.xp * HERBS_PER_HOUR as f64, "d")),
            source.c1("Herblore xp/hr"),
        ]
        .join(" "),
        vec![
            source.c2(&commas(MAGIC_XP_PER_CAST * CASTS_PER_HOUR as f64, "d")),
            source.c1("Magic xp/hr"),
        ]
        .join(" "),
        source.p(&format!(
            "{} Herblore, {} Magic",
            herb.herblore, MAGIC_LEVEL
        )),
    ];

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
    fn an_hour_buys_16200_herbs_and_1200_nature_runes() {
        let hour = hourly(10, 20, 100);

        assert_eq!(hour.cost, 16_200 * 10 + 1_200 * 100);
        assert_eq!(hour.revenue, 16_200 * 20);
        assert_eq!(hour.profit, hour.revenue - hour.cost);
    }

    #[test]
    fn a_loss_is_reported_as_negative_profit() {
        // Clean sells for less than the grimy costs.
        let hour = hourly(100, 90, 100);

        assert!(hour.profit < 0, "expected a loss, got {}", hour.profit);
        assert_eq!(hour.profit, 16_200 * 90 - (16_200 * 100 + 1_200 * 100));
    }

    #[test]
    fn herbs_are_found_by_short_name_full_name_or_grimy_name() {
        assert_eq!(find_herb("irit").map(|h| h.clean), Some("Irit leaf"));
        assert_eq!(find_herb("Irit leaf").map(|h| h.clean), Some("Irit leaf"));
        assert_eq!(find_herb("grimy irit").map(|h| h.clean), Some("Irit leaf"));
        assert_eq!(find_herb("RANARR").map(|h| h.clean), Some("Ranarr weed"));
        assert_eq!(find_herb("dwarf").map(|h| h.clean), Some("Dwarf weed"));
    }

    #[test]
    fn an_exact_name_wins_over_a_partial_one() {
        // "Guam leaf" contains "guam", but so would any future "Guam x".
        assert_eq!(find_herb("guam leaf").map(|h| h.clean), Some("Guam leaf"));
    }

    #[test]
    fn unknown_herbs_are_not_found() {
        assert!(find_herb("torstul").is_none());
        assert!(find_herb("").is_none());
        assert!(find_herb("nature rune").is_none());
    }

    #[test]
    fn a_loss_keeps_its_minus_sign_when_written_out() {
        assert_eq!(commas(-65_400.0, "d"), "-65,400");
    }

    #[test]
    fn every_herb_is_named_and_distinct() {
        for herb in HERBS.iter() {
            assert_eq!(
                herb.grimy.to_lowercase(),
                format!("grimy {}", herb.clean.to_lowercase()),
                "{} has a mismatched grimy name",
                herb.clean
            );
            assert!(
                herb.herblore >= 50,
                "{} is below the spell's level",
                herb.clean
            );
            assert!(herb.xp > 0.0, "{} has no xp", herb.clean);
        }

        let names: Vec<&str> = HERBS.iter().map(|herb| herb.clean).collect();
        let mut unique = names.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(names.len(), unique.len(), "duplicate herb in the table");
    }
}
