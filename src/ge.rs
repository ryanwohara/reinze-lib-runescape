use super::items::{Ge, GeItemPrice};
use crate::common::{eval_query, parse_item_db};
use anyhow::{Context, Result};
use common::source::Source;
use common::{c3, c4, c5, commas, not_found};
use serde_json;

// Scan lib/item_db.json for up to 10 items that match the query
// and hit the OSRS Grand Exchange API to get the price of each item
pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let item_db =
        parse_item_db(&s.query).map_err(|_| anyhow::anyhow!("failed to parse item database"))?;

    let mut output = s.l("Grand Exchange");
    let mut found_items: Vec<String> = vec![];
    let mut total_value = 0.0;

    for item in item_db.iter() {
        // Hit the OSRS API with the item ID
        let url = format!(
            "http://services.runescape.com/m=itemdb_oldschool/api/catalogue/detail.json?item={}",
            item.id
        );

        let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");

        let response = rt
            .block_on(reqwest::get(&url))
            .context("failed to get response from OSRS API")?;

        let j: String = rt
            .block_on(response.text())
            .context("failed to parse response from OSRS API into text")?;

        let json: Ge =
            serde_json::from_str(&j).context("failed to parse response from OSRS API into JSON")?;

        let ge_json = &json.item;

        let count = item.total.unwrap_or(0);

        if count == 0 {
            total_value += ge_json.current.price.num();

            found_items.push(format!(
                "{}: {}{} {}",
                s.c1(&item.name),
                s.c2(&ge_json.current.price.str()),
                s.c1("gp"),
                s.p(price_change(&ge_json.today, count))
            ));
        } else {
            let total = match eval_query(ge_json.current.price.str().replace(" ", "")) {
                Ok(t) => t * count as f64,
                Err(_) => 0.0,
            };
            total_value += total;

            found_items.push(format!(
                "{}: {}{} {}",
                s.c1(&format!("{}x {}", commas(count as f64, "d"), item.name)),
                s.c2(&commas(total, "d")),
                s.c1("gp"),
                s.p(price_change(&ge_json.today, count))
            ));
        }
    }

    let item_count = found_items.len();
    output = format!("{} {}", output, not_found(found_items));

    let mut output_vec = vec![output];
    if item_count > 1 {
        output_vec.push(format!(
            "{} {}{}",
            s.l("Total"),
            s.c2(&commas(total_value, "d")),
            s.c1("gp")
        ));
    }

    Ok(output_vec)
}

fn price_change(today: &GeItemPrice, count: u64) -> String {
    let mut output = String::new();
    let str_price = today.price.str().replace(" ", "");

    let price = match count {
        0..2 => str_price,
        _ => match eval_query(str_price.clone()) {
            Ok(s) => commas(s * count as f64, "d"),
            Err(_) => str_price,
        },
    };

    if today.trend == "neutral" {
        output = format!("{}{}", c5(&price), c5("▬"));
    } else if today.trend == "positive" {
        output = format!("{}{}", c4(&price), c4("▲"));
    } else if today.trend == "negative" {
        output = format!("{}{}", c3(&price), c3("▼"));
    }

    output
}
