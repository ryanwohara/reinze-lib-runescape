use crate::common::c1;
use crate::common::c2;
use crate::common::l;

pub fn patch(query: &str) -> Result<Vec<String>, ()> {
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

    let prefix = l("Patch");

    let mut found_params: Vec<String> = vec![];

    for (k, v) in section.iter() {
        if k.to_ascii_lowercase()
            .contains(&underscored.to_ascii_lowercase())
        {
            found_params.push(format!("{}: {}", c1(&k.replace("_", " ")), c2(v)));
        }

        if found_params.len() >= 10 {
            break;
        }
    }

    if found_params.len() == 0 {
        return Ok(vec![format!("{}: {}", prefix, c1("No results found"))]);
    }

    let output = format!("{} {}", prefix, found_params.join(&c1(" | ")));

    let output_vec = vec![output];

    Ok(output_vec)
}
