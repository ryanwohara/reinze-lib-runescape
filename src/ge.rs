use super::items::{Ge, GeItemPrice, StrOrNum};
use crate::common::{eval_query, parse_item_db};
use common::{c1, c2, c3, c4, c5, commas, l, not_found, p};
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

    for item in item_db.iter() {
        // Hit the OSRS API with the item ID
        let url = format!(
            "http://services.runescape.com/m=itemdb_oldschool/api/catalogue/detail.json?item={}",
            item.id
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

        let count = item.total.unwrap_or_else(|| 0);

        if count == 0 {
            found_items.push(format!(
                "{}: {}{} {}",
                c1(&item.name),
                c2(&str_from_enum(ge_json.current.price)),
                c1("gp"),
                price_change(&ge_json.today, count)
            ));
        } else {
            let total = match eval_query(str_from_enum(ge_json.current.price).replace(" ", "")) {
                Ok(t) => t * count as f64,
                Err(_) => 0.0,
            };

            found_items.push(format!(
                "{}: {}{} {}",
                c1(&format!("{}x {}", commas(count as f64, "d"), item.name)),
                c2(&commas(total, "d")),
                c1("gp"),
                price_change(&ge_json.today, count)
            ));
        }
    }

    output = format!("{} {}", output, not_found(found_items));

    let output_vec = vec![output];

    Ok(output_vec)
}

fn price_change(today: &GeItemPrice, count: u64) -> String {
    let mut output = String::new();
    let str_price = str_from_enum(today.price).replace(" ", "");

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

    p(&output)
}

fn str_from_enum(price: StrOrNum) -> String {
    match price {
        StrOrNum::Str(s) => s.to_string(),
        StrOrNum::Num(n) => n.to_string(),
    }
}
