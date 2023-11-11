extern crate ini;

use common::{c1, c2, capitalize, l, not_found};
use ini::Ini;

pub fn params(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Params").to_string();

    let (skill, param) = match query.split_once(" ") {
        Some((skill, param)) if !skill.is_empty() && !param.is_empty() => {
            (common::skill(skill), param)
        }
        _ => {
            return Ok(vec![format!(
                "{} {}",
                prefix,
                c2("Invalid number of arguments")
            )])
        }
    };

    if skill.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, c2("Invalid skill"))]);
    }

    let database = Ini::load_from_file("lib/Database.ini").map_err(|e| {
        println!("Error loading Database.ini: {}", e);
        ()
    })?;

    let prefix = l(&capitalize(&skill)).to_string();

    let section = match database.section(Some(capitalize(&skill))) {
        Some(section) => section,
        _ => return Ok(vec![format!("{} {}", prefix, c1("No results found"))]),
    };

    let underscored = param.replace(" ", "_");

    let found_params: Vec<String> = section
        .iter()
        .filter(|(k, _)| {
            k.to_ascii_lowercase()
                .contains(&underscored.to_ascii_lowercase())
        })
        .take(10)
        .map(|(k, v)| format!("{} {}", c1(&k.replace("_", " ")), c2(&format!("{}xp", v))))
        .collect();
    Ok(vec![format!("{} {}", prefix, not_found(found_params))])
}
