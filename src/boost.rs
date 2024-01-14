use crate::common::skill;
use common::{c1, c2, capitalize, l, not_found};

pub fn boost(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Boost");

    if query.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, c1("No query provided"))]);
    }

    let skill = skill(query).to_ascii_lowercase();

    let boost = match ini::Ini::load_from_file("lib/boost.ini") {
        Ok(boost) => boost,
        Err(e) => {
            println!("Error loading boost.ini: {}", e);
            return Err(());
        }
    };
    let section = match boost.section(Some("skills")) {
        Some(section) => section,
        None => {
            println!("Error getting section: skills");
            return Err(());
        }
    };

    let mut found_params: Vec<String> = vec![];

    for (k, v) in section.iter() {
        if k.contains(&skill) {
            let split = v.split(" ~ ").collect::<Vec<&str>>();
            found_params.push(c1(&capitalize(k)));
            for s in split {
                found_params.push(c2(s));
            }
            break;
        }
    }

    let output = format!("{} {}", prefix, not_found(found_params));

    Ok(vec![output])
}
