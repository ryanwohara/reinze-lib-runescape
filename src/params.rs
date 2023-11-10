extern crate ini;

use common::c1;
use common::c2;
use common::capitalize;
use common::l;
use ini::Ini;

pub fn params(query: &str) -> Result<Vec<String>, ()> {
    //skill: &str, param: &str
    let params: (&str, &str) = match query.split_once(" ") {
        Some(params) => params,
        None => return Ok(vec!["Invalid number of arguments".to_string()]),
    };

    let skill = params.0;
    let param = params.1;

    if params.0.is_empty() || params.1.is_empty() {
        return Ok(vec!["Invalid number of arguments".to_string()]);
    }

    let database = match Ini::load_from_file("lib/Database.ini") {
        Ok(database) => database,
        Err(e) => {
            println!("Error loading Database.ini: {}", e);
            return Err(());
        }
    };

    let section = match database.section(Some(capitalize(skill))) {
        Some(section) => section,
        None => {
            println!("Error getting section: {}", skill);
            return Err(());
        }
    };

    let underscored = param.replace(" ", "_");

    let prefix = l(&capitalize(skill)).to_string();

    let mut found_params: Vec<String> = vec![];

    for (k, v) in section.iter() {
        if k.to_ascii_lowercase()
            .contains(&underscored.to_ascii_lowercase())
        {
            found_params.push(format!(
                "{}: {}",
                c1(&k.replace("_", " ")),
                c2(&format!("{}xp", v.to_string()))
            ));
        }

        if found_params.len() >= 10 {
            break;
        }
    }

    if found_params.len() == 0 {
        return Err(());
    }

    let output = format!("{} {}", prefix, found_params.join(&c1(" | ")));

    let output_vec: Vec<String> = vec![output];

    Ok(output_vec)
}
