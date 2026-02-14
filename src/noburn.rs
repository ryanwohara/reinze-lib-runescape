use common::source::Source;

pub fn noburn(s: &Source) -> Result<Vec<String>, ()> {
    let query = &s.query;

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
        .map(|fish| fish.to_string(s))
        .collect();

    Ok(vec![
        format!("{} {}", s.l("NoBurn"), output.join(&s.c1(" | "))),
        s.p(
            "Fire | Range | Hosidius 5% | Hosidius 10% | (Gauntlets | Gauntlets + Hosidius 5% | Gauntlet + Hosidius 10%)",
        ),
    ])
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

    fn to_string(&self, s: &Source) -> String {
        format!(
            "{} {} {} {} {} {}",
            s.c1(self.name),
            s.c2(&if_not_available(self.fire, s)),
            s.c2(&if_not_available(self.range, s)),
            s.c2(&if_not_available(self.hosidius5, s)),
            s.c2(&if_not_available(self.hosidius10, s)),
            s.p(&format!(
                "{} {} {}",
                &if_not_available(self.gauntlet, s),
                &if_not_available(self.gauntlet_hosidius5, s),
                &if_not_available(self.gauntlet_hosidius10, s)
            )),
        )
    }
}

fn if_not_available(int: u32, s: &Source) -> String {
    if int == 0 {
        return s.c2("N/A");
    }

    int.to_string()
}

fn fish_finder(fish: &Fish, query: &str) -> bool {
    (query.len() > 0 && fish.name.to_lowercase().contains(&query.to_lowercase()))
        || query.len() == 0
}
