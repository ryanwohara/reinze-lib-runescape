use crate::common::parse_item_db;
use common::{c1, c2, commas, l, not_found, p};

pub fn lookup(query: &str) -> Result<Vec<String>, ()> {
    // Scan lib/item_db.json for up to 10 items that match the query
    let item_db = match parse_item_db(query) {
        Ok(item_db) => item_db,
        Err(_) => return Err(()),
    };

    let mut output = l("Price");
    let mut found_items: Vec<String> = vec![];

    for item in item_db.iter() {
        let total = item.total.unwrap_or_else(|| 0);

        if total < 2 {
            found_items.push(format!(
                "{}: {}{} {}",
                c1(&item.name),
                match item.highalch {
                    Some(value) => c2(&commas(value as f64, "d")),
                    None => c2("0"),
                },
                c1("gp"),
                match item.lowalch {
                    Some(value) => p(&format!("{}{}", commas(value as f64, "d"), c1("gp"))),
                    None => p(&format!("0{}", c1("gp"))),
                }
            ));
        } else {
            found_items.push(format!(
                "{}: {}{} {}",
                c1(&item.name),
                match item.highalch {
                    Some(value) => c2(&commas((total * value) as f64, "d")),
                    None => c2("0"),
                },
                c1("gp"),
                match item.lowalch {
                    Some(value) => p(&format!(
                        "{}{}",
                        commas((total * value) as f64, "d"),
                        c1("gp")
                    )),
                    None => p(&format!("0{}", c1("gp"))),
                }
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
