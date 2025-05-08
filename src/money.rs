use crate::common::parse_item_db;
use crate::items::{Data, Mapping};
use common::{c1, c2, commas, l, not_found};
use std::fs::read_to_string;

const NATURE_RUNE_ID: u32 = 561;

pub fn printer() -> Result<Vec<String>, ()> {
    let item_db = match parse_item_db(".*") {
        Ok(item_db) => item_db,
        Err(_) => return Err(()),
    };

    let ge_filename = "lib/ge.json";
    let mut output = l("$Money$");
    let mut found_items: Vec<(&Mapping, i64)> = vec![];

    let ge_file_contents = match read_to_string(ge_filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening ge.json: {}", e);
            return Err(());
        }
    };

    let ge_json = match serde_json::from_str::<Data>(&ge_file_contents) {
        Ok(json) => json,
        Err(e) => {
            println!("Error parsing ge.json into JSON: {}", e);
            return Err(());
        }
    };

    let ge_data = ge_json.data;

    let nature_rune = match ge_data.get(&NATURE_RUNE_ID) {
        Some(item) => item.high.unwrap_or(0) as i64,
        None => {
            println!("Error getting item: {} (nature rune)", NATURE_RUNE_ID);
            return Err(());
        }
    };

    for item in item_db.iter() {
        let item_metadata = match ge_data.get(&item.id) {
            Some(item) => item,
            None => {
                println!("Error getting item: {}", item.id);
                continue;
            }
        };

        let highalch = item.highalch.unwrap_or(0) as i64;
        let high = item_metadata.high.unwrap_or(0) as i64;
        let profit = highalch - nature_rune - high;

        if profit > 0 {
            found_items.push((item, profit));
        }
    }

    found_items.sort_by(|(_item1, profit1), (_item2, profit2)| profit2.cmp(profit1));

    let mut sorted_items: Vec<String> = found_items
        .into_iter()
        .map(|(item, profit)| {
            format!(
                "{}: {}{}",
                c1(&item.name),
                c2(&commas(profit as f64, "d")),
                c1("gp")
            )
        })
        .collect();

    let mut iterator = 0;
    sorted_items.retain(|_x| {
        iterator += 1;

        iterator <= 5
    });

    output = format!("{} {}", output, not_found(sorted_items));

    let output_vec: Vec<String> = vec![output];

    Ok(output_vec)
}
