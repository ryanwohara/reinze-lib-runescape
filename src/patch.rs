use common::{c1, c2, l, not_found};

pub fn patch(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Patch");

    if query.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, c1("No query provided"))]);
    }

    let underscored = query.replace(" ", "_");

    let patch = match ini::Ini::load_from_file("lib/patch.ini") {
        Ok(patch) => patch,
        Err(e) => {
            println!("Error loading patch.ini: {}", e);
            return Err(());
        }
    };

    let section = match patch.section(Some("patches")) {
        Some(section) => section,
        None => {
            println!("Error getting section: patch");
            return Err(());
        }
    };

    let mut found_params: Vec<String> = vec![];

    for (k, v) in section.iter() {
        if k.to_ascii_lowercase()
            .contains(&underscored.to_ascii_lowercase())
        {
            found_params.push(format!("{}: {}", c1(&k.replace("_", " ")), c2(v)));
            break;
        }
    }

    let output = format!("{} {}", prefix, not_found(found_params));

    Ok(vec![output])
}
