use crate::common::parse_item_db;
use crate::items::Data;
use crate::tax::{strip_tax_flag, tax_note, total_line};
use anyhow::{Context, Result, bail};
use common::commas;
use common::source::Source;
use log::error;
use serde_json;
use std::fs::read_to_string;

// Scan lib/item_db.json for up to 10 items that match the query
pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let (query, taxed) = strip_tax_flag(&s.query);
    let item_db = parse_item_db(&query).context("failed to parse item db")?;

    let ge_filename = "lib/ge.json";
    let mut output = s.l("Price");
    let mut found_items: Vec<String> = vec![];
    let mut total_value = 0.0;
    let mut total_tax = 0;

    let ge_file_contents = match read_to_string(ge_filename) {
        Ok(file) => file,
        Err(e) => {
            error!("Error opening ge.json: {}", e);
            bail!("Error opening ge.json: {}", e);
        }
    };

    let ge_json = match serde_json::from_str::<Data>(&ge_file_contents) {
        Ok(json) => json,
        Err(e) => {
            error!("Error parsing ge.json into JSON: {}", e);
            bail!("Error parsing ge.json into JSON: {}", e);
        }
    };

    let ge_data = ge_json.data;

    for item in item_db.iter() {
        let item_values = match ge_data.get(&item.id) {
            Some(item) => item,
            None => {
                error!("Error getting item: {}", item.id);
                continue;
            }
        };

        let count = match item.total {
            Some(0) | None => 1,
            Some(total) => total,
        };
        let name = match count {
            1 => item.name.to_string(),
            _ => format!("{}x {}", commas(count as f64, "d"), item.name),
        };
        let price = item_values.high.unwrap_or(0);
        let (note, tax) = match taxed {
            true => {
                let (note, tax) = tax_note(s, item.id, price, count);
                (format!(" {}", note), tax)
            }
            false => (String::new(), 0),
        };
        let value = price as f64 * count as f64 - tax as f64;
        total_value += value;
        total_tax += tax;

        let entry = format!(
            "{}: {}{}{}",
            s.c1(&name),
            s.c2(&commas(value, "d")),
            s.c1("gp"),
            note
        );

        found_items.push(entry);

        if found_items.len() >= 10 {
            break;
        }
    }

    let item_count = found_items.len();
    output = format!("{} {}", output, s.not_found(found_items));

    let mut output_vec: Vec<String> = vec![output];
    if item_count > 1 {
        output_vec.push(total_line(s, total_value, taxed.then_some(total_tax)));
    }

    Ok(output_vec)
}
