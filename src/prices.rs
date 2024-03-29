use crate::items::Data;
use crate::items::Mapping;
use common::{c1, c2, commas, l, not_found};
use regex::Regex;
use serde_json;
use std::fs::read_to_string;

// Scan lib/item_db.json for up to 10 items that match the query
pub fn prices(query: &str) -> Result<Vec<String>, ()> {
    let mapping_filename = "lib/item_db.json";
    let ge_filename = "lib/ge.json";

    let mut output = l("Price");
    let mut found_items: Vec<String> = vec![];

    let mapping_file_contents = match read_to_string(mapping_filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening item_db.json: {}", e);
            return Err(());
        }
    };

    let mapping_json = match serde_json::from_str::<Vec<Mapping>>(&mapping_file_contents) {
        Ok(json) => json,
        Err(e) => {
            println!("Error parsing item_db.json into JSON: {}", e);
            return Err(());
        }
    };

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

    for item in mapping_json.iter() {
        let regex_string = format!(r"(?i){}", query);
        let re = match Regex::new(&regex_string) {
            Ok(re) => re,
            Err(e) => {
                println!("Error creating regex: {}", e);
                return Err(());
            }
        };

        let matched = re.captures(&item.name);
        if matched.is_some() {
            let item_values = match ge_data.get(&item.id) {
                Some(item) => item,
                None => {
                    println!("Error getting item: {}", item.id);
                    continue;
                }
            };
            found_items.push(format!(
                "{}: {}{}",
                c1(&item.name),
                match item_values.high {
                    Some(value) => c2(&commas(value as f64, "d")),
                    None => c2("0"),
                },
                c1("gp")
            ));
        }

        if found_items.len() >= 10 {
            break;
        }
    }

    output = format!("{} {}", output, not_found(found_items));

    let output_vec: Vec<String> = vec![output];

    Ok(output_vec)
}
