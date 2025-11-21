use common::{c1, c2, l, p};

pub fn noburn(query: &str) -> Result<Vec<String>, ()> {
    let fish: Vec<Fish> = vec![
        Fish::new("Tuna", 63, 0, 59, 0, 0, 0, 0),
        Fish::new("Lobster", 74, 74, 70, 0, 64, 61, 0),
        Fish::new("Swordfish", 86, 81, 76, 0, 81, 76, 0),
        Fish::new("Monkfish", 92, 90, 86, 82, 87, 82, 0),
        Fish::new("Shark", 0, 0, 0, 99, 94, 89, 84),
        Fish::new("Anglerfish", 0, 0, 0, 0, 98, 93, 88),
    ];
    let output: Vec<String> = fish
        .into_iter()
        .filter(|fish| fish_finder(fish, query))
        .map(|fish| fish.to_string())
        .collect();
    return Ok(vec![
        format!("{} {}", l("NoBurn"), output.join(&c1(" | "))),
        p(
            "Fire | Range | Hosidius 5% | Hosidius 10% | (Gauntlets | Gauntlets + Hosidius 5% | Gauntlet + Hosidius 10%)",
        ),
    ]);
}

struct Fish {
    name: &'static str,
    fire: u32,
    range: u32,
    hosidius5: u32,
    hosidius10: u32,
    gauntlet: u32,
    gauntlet_hosidius5: u32,
    gauntlet_hosidius10: u32,
}

impl Fish {
    fn new(
        name: &'static str,
        fire: u32,
        range: u32,
        hosidius5: u32,
        hosidius10: u32,
        gauntlet: u32,
        gauntlet_hosidius5: u32,
        gauntlet_hosidius10: u32,
    ) -> Self {
        Self {
            name,
            fire,
            range,
            hosidius5,
            hosidius10,
            gauntlet,
            gauntlet_hosidius5,
            gauntlet_hosidius10,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            c1(self.name),
            c2(&if_not_available(self.fire)),
            c2(&if_not_available(self.range)),
            c2(&if_not_available(self.hosidius5)),
            c2(&if_not_available(self.hosidius10)),
            p(&format!(
                "{} {} {}",
                &if_not_available(self.gauntlet),
                &if_not_available(self.gauntlet_hosidius5),
                &if_not_available(self.gauntlet_hosidius10)
            )),
        )
    }
}

fn if_not_available(int: u32) -> String {
    if int == 0 {
        return c2("N/A");
    }

    int.to_string()
}

fn fish_finder(fish: &Fish, query: &str) -> bool {
    (query.len() > 0 && fish.name.to_lowercase().contains(&query.to_lowercase()))
        || query.len() == 0
}
