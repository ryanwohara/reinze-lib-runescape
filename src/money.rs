use crate::common::{eval_query, parse_item_db};
use crate::items::{Data, Mapping};
use common::source::Source;
use common::{commas, not_found};
use regex::Regex;
use std::fs::read_to_string;

const NATURE_RUNE_ID: u32 = 561;

pub fn printer(s: &Source) -> Result<Vec<String>, ()> {
    let query = s.query.as_str();

    let limit_flag_re = Regex::new(r"-l ?(\d+[kmb])").unwrap();
    let limit_flag = match limit_flag_re.captures(query) {
        Some(captured) => captured.get(1).map_or(0i64, |limit| {
            eval_query(limit.as_str()).unwrap_or(0f64) as i64
        }),
        None => 0i64,
    };

    let query = if limit_flag > 0 {
        &limit_flag_re.replace(query, "").to_string()
    } else {
        query
    };

    let input = if query.is_empty() {
        r"^(rune|adamant|mithril|magic|yew|arrow)[\w\s]+$"
    } else {
        query
    };

    let item_db = match parse_item_db(input) {
        Ok(item_db) => item_db,
        Err(_) => return Err(()),
    };

    let ge_filename = "lib/ge.json";
    let mut output = s.l("$ Alch Profit $");
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
            let limit = commas(item.limit.unwrap_or(0) as f64, "d");

            if item.limit.unwrap_or(0) >= limit_flag as u64 {
                format!(
                    "{}: {}{} {}",
                    s.c1(&item.name),
                    s.c2(&commas(profit as f64, "d")),
                    s.c1("gp"),
                    s.p(&format!("L:{}", limit))
                )
            } else {
                "".to_string()
            }
        })
        .collect();

    let mut iterator = 0;
    sorted_items.retain(|x| {
        if x.is_empty() {
            return false;
        }

        iterator += 1;

        iterator <= 15
    });

    output = vec![output, not_found(sorted_items)].join(" ");

    let output_vec: Vec<String> = vec![output];

    Ok(output_vec)
}
