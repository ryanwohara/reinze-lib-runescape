use crate::common::parse_item_db;
use common::source::Source;
use common::{commas, not_found};

pub fn lookup(s: &Source) -> Result<Vec<String>, ()> {
    let query = &s.query;

    // Scan lib/item_db.json for up to 10 items that match the query
    let item_db = match parse_item_db(query) {
        Ok(item_db) => item_db,
        Err(_) => return Err(()),
    };

    let mut output = s.l("Alch");
    let mut found_items: Vec<String> = vec![];

    for item in item_db.iter() {
        let total = item.total.unwrap_or_else(|| 0);

        if total < 2 {
            found_items.push(format!(
                "{}: {}{} {}",
                s.c1(&item.name),
                match item.highalch {
                    Some(value) => s.c2(&commas(value as f64, "d")),
                    None => s.c2("0"),
                },
                s.c1("gp"),
                match item.lowalch {
                    Some(value) => s.p(&format!("{}{}", commas(value as f64, "d"), s.c1("gp"))),
                    None => s.p(&format!("0{}", s.c1("gp"))),
                }
            ));
        } else {
            found_items.push(format!(
                "{}: {}{} {}",
                s.c1(&item.name),
                match item.highalch {
                    Some(value) => s.c2(&commas((total * value) as f64, "d")),
                    None => s.c2("0"),
                },
                s.c1("gp"),
                match item.lowalch {
                    Some(value) => s.p(&format!(
                        "{}{}",
                        commas((total * value) as f64, "d"),
                        s.c1("gp")
                    )),
                    None => s.p(&format!("0{}", s.c1("gp"))),
                }
            ));
        }

        if found_items.len() >= 10 {
            break;
        }
    }

    output = vec![output, not_found(found_items)].join(" ");

    Ok(vec![output])
}
