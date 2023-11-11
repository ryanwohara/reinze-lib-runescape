extern crate ini;

use common::{c1, c2, capitalize, l};
use ini::Ini;

pub fn params(query: &str) -> Result<Vec<String>, ()> {
    let (skill, param) = match query.split_once(" ") {
        Some((skill, param)) if !skill.is_empty() && !param.is_empty() => (skill, param),
        _ => return Ok(vec!["Invalid number of arguments".to_string()]),
    };

    let database = Ini::load_from_file("lib/Database.ini").map_err(|e| {
        println!("Error loading Database.ini: {}", e);
        ()
    })?;

    let section = database.section(Some(capitalize(skill))).ok_or_else(|| {
        println!("Error getting section: {}", skill);
        ()
    })?;

    let underscored = param.replace(" ", "_");
    let prefix = l(&capitalize(skill)).to_string();

    let found_params: Vec<String> = section
        .iter()
        .filter(|(k, _)| {
            k.to_ascii_lowercase()
                .contains(&underscored.to_ascii_lowercase())
        })
        .take(10)
        .map(|(k, v)| format!("{}: {}", c1(&k.replace("_", " ")), c2(&format!("{}xp", v))))
        .collect();

    if found_params.is_empty() {
        return Err(());
    }

    Ok(vec![format!(
        "{} {}",
        prefix,
        found_params.join(&c1(" | "))
    )])
}
