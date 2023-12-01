use common::{c1, c2, commas, l, not_found, p, remove_trailing_zeroes};
use serde::{Deserialize, Serialize};

pub fn lookup(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Plant");

    if query.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, c1("No query provided"))]);
    }

    let underscored = query.replace(" ", "_");

    let plants = match ini::Ini::load_from_file("lib/plants.ini") {
        Ok(plants) => plants,
        Err(e) => {
            println!("Error loading plants.ini: {}", e);
            return Err(());
        }
    };

    let section = match plants.section(Some("plants")) {
        Some(section) => section,
        None => {
            println!("Error getting section: plants");
            return Err(());
        }
    };

    let mut found_params: Vec<Plant> = vec![];

    for (k, v) in section.iter() {
        if k.to_ascii_lowercase()
            .contains(&underscored.to_ascii_lowercase())
        {
            let split = v.split("~").collect::<Vec<&str>>();
            let plant = Plant::new(
                k.replace("_", " "),
                split.get(0).unwrap_or(&"0").parse::<u32>().unwrap_or(0),
                split.get(1).unwrap_or(&"0").parse::<u32>().unwrap_or(0),
                split.get(2).unwrap_or(&"0").parse::<f64>().unwrap_or(0.0),
                split.get(3).unwrap_or(&"0").parse::<f64>().unwrap_or(0.0),
                split.get(4).unwrap_or(&"0").parse::<f64>().unwrap_or(0.0),
                split.get(5).unwrap_or(&"err").to_string(),
            );
            found_params.push(plant);
            break;
        }
    }

    if found_params.len() == 0 {
        return Ok(vec![format!("{}: {}", prefix, c1("No results found"))]);
    }

    let output = format!(
        "{} {}",
        prefix,
        not_found(
            found_params
                .into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
        )
    );

    Ok(vec![output])
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Plant {
    name: String,
    level: u32,
    time: u32,
    planting_xp: f64,
    checking_xp: f64,
    harvesting_xp: f64,
    payment: String,
}

impl Plant {
    fn new(
        name: String,
        level: u32,
        time: u32,
        planting_xp: f64,
        checking_xp: f64,
        harvesting_xp: f64,
        payment: String,
    ) -> Self {
        Self {
            name,
            level,
            time,
            planting_xp,
            checking_xp,
            harvesting_xp,
            payment,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {}",
            p(&self.name),
            c1("Req Lvl:"),
            c2(&self.level.to_string()),
            c1("Time:"),
            c2(&self.time.to_string()),
            c1("Planting XP:"),
            c2(&round(self.planting_xp)),
            c1("Checking XP:"),
            c2(&round(self.checking_xp)),
            c1("Harvesting XP:"),
            c2(&round(self.harvesting_xp)),
            c1("Payment:"),
            c2(&self.payment.to_string())
        )
    }
}

fn round(num: f64) -> String {
    remove_trailing_zeroes(&commas(f64::round(num * 10.0) / 10.0, "f"))
}
