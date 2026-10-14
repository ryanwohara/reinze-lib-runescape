use super::items::{Ge, GeItemPrice};
use crate::common::{eval_query, parse_item_db};
use crate::tax::{strip_tax_flag, tax_note, total_line};
use anyhow::{Context, Result};
use common::source::Source;
use common::{c3, c4, c5, commas};
use serde_json;

// Scan lib/item_db.json for up to 10 items that match the query
// and hit the OSRS Grand Exchange API to get the price of each item
pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let (query, taxed) = strip_tax_flag(&s.query);
    let item_db =
        parse_item_db(&query).map_err(|_| anyhow::anyhow!("failed to parse item database"))?;

    let mut output = s.l("Grand Exchange");
    let mut found_items: Vec<String> = vec![];
    let mut total_value = 0.0;
    let mut total_tax = 0;

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

        let gross = match count {
            0 => ge_json.current.price.num(),
            _ => match eval_query(ge_json.current.price.str().replace(" ", "")) {
                Ok(t) => t * count as f64,
                Err(_) => 0.0,
            },
        };

        // The API abbreviates large prices ("1.2m"), so the tax on those is
        // only as precise as the abbreviation.
        let (note, tax) = match taxed {
            true => {
                let (note, tax) = tax_note(s, item.id, ge_json.current.price.num() as u64, count);
                (format!(" {}", note), tax)
            }
            false => (String::new(), 0),
        };
        let value = gross - tax as f64;
        total_value += value;
        total_tax += tax;

        let name = match count {
            0 => item.name.to_string(),
            _ => format!("{}x {}", commas(count as f64, "d"), item.name),
        };
        // A lone untaxed item keeps the API's own wording of its price.
        let price = match (count, taxed) {
            (0, false) => ge_json.current.price.str(),
            _ => commas(value, "d"),
        };

        found_items.push(format!(
            "{}: {}{}{} {}",
            s.c1(&name),
            s.c2(&price),
            s.c1("gp"),
            note,
            s.p(price_change(&ge_json.today, count))
        ));
    }

    let item_count = found_items.len();
    output = format!("{} {}", output, s.not_found(found_items));

    let mut output_vec = vec![output];
    if item_count > 1 {
        output_vec.push(total_line(s, total_value, taxed.then_some(total_tax)));
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
