use super::items::{Ge, GeItemPrice, StrOrNum};
use crate::common::parse_item_db;
use common::{c1, c2, c3, c4, c5, l, not_found, p};
use serde_json;

// Scan lib/item_db.json for up to 10 items that match the query
// and hit the OSRS Grand Exchange API to get the price of each item
pub fn ge(query: &str) -> Result<Vec<String>, ()> {
    let item_db = match parse_item_db(query) {
        Ok(item_db) => item_db,
        Err(_) => return Err(()),
    };

    let mut output = l("Grand Exchange");
    let mut found_items: Vec<String> = vec![];

    for item_str in item_db.iter() {
        let result = match serde_json::from_str::<Ge>(item_str) {
            Ok(json) => json,
            Err(e) => {
                println!("Error deserializing item mapping in GE: {e}");
                return Err(());
            }
        };

        // Hit the OSRS API with the item ID
        let url = format!(
            "http://services.runescape.com/m=itemdb_oldschool/api/catalogue/detail.json?item={}",
            result.item.id
        );

        let rt = tokio::runtime::Runtime::new().unwrap();

        let response = match rt.block_on(reqwest::get(&url)) {
            Ok(response) => response,
            Err(e) => {
                println!("Error getting response from OSRS API: {}", e);
                return Err(());
            }
        };

        let j: String = match rt.block_on(response.text()) {
            Ok(j) => j,
            Err(e) => {
                println!("Error parsing response from OSRS API into JSON: {}", e);
                return Err(());
            }
        };

        let json: Ge = match serde_json::from_str(&j) {
            Ok(json) => json,
            Err(e) => {
                println!("Error parsing response from OSRS API into JSON: {}", e);
                return Err(());
            }
        };

        let ge_json = &json.item;

        found_items.push(format!(
            "{}: {}{} {}",
            c1(&result.item.name),
            c2(&str_from_enum(ge_json.current.price)),
            c1("gp"),
            price_change(&ge_json.today)
        ));
    }

    output = format!("{} {}", output, not_found(found_items));

    let output_vec = vec![output];

    Ok(output_vec)
}

fn price_change(today: &GeItemPrice) -> String {
    let mut output = String::new();

    if today.trend == "neutral" {
        output = format!("{}{}", c5(&str_from_enum(today.price)), c5("▬"));
    } else if today.trend == "positive" {
        output = format!("{}{}", c4(&str_from_enum(today.price)), c4("▲"));
    } else if today.trend == "negative" {
        output = format!(
            "{}{}",
            c3(&str_from_enum(today.price).replace(" ", "")),
            c3("▼")
        );
    }

    p(&output)
}

fn str_from_enum(price: StrOrNum) -> String {
    match price {
        StrOrNum::Str(s) => s.to_string(),
        StrOrNum::Num(n) => n.to_string(),
    }
}
