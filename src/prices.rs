use crate::common::parse_item_db;
use crate::items::Data;
use anyhow::{Context, Result, bail};
use common::source::Source;
use common::{commas, not_found};
use log::error;
use serde_json;
use std::fs::read_to_string;

// Scan lib/item_db.json for up to 10 items that match the query
pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let item_db = parse_item_db(&s.query).context("failed to parse item db")?;

    let ge_filename = "lib/ge.json";
    let mut output = s.l("Price");
    let mut found_items: Vec<String> = vec![];
    let mut total_value = 0.0;

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

        match item.total {
            Some(0) | Some(1) => {
                found_items.push(format!(
                    "{}: {}{}",
                    s.c1(&item.name),
                    match item_values.high {
                        Some(value) => {
                            total_value += value as f64;
                            s.c2(&commas(value as f64, "d"))
                        }
                        None => s.c2("0"),
                    },
                    s.c1("gp")
                ));
            }
            total => {
                let total = total.unwrap_or_else(|| 1);

                found_items.push(format!(
                    "{}: {}{}",
                    s.c1(&format!("{}x {}", commas(total as f64, "d"), item.name)),
                    match item_values.high {
                        Some(value) => {
                            let multiplied_value = value as f64 * total as f64;
                            total_value += multiplied_value;
                            s.c2(&commas(multiplied_value, "d"))
                        }
                        None => s.c2("0"),
                    },
                    s.c1("gp")
                ));
            }
        }

        if found_items.len() >= 10 {
            break;
        }
    }

    let item_count = found_items.len();
    output = format!("{} {}", output, not_found(found_items));

    let mut output_vec: Vec<String> = vec![output];
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
