use common::{c1, c2, l, p};

pub fn noburn(query: &str) -> Result<Vec<String>, ()> {
    let fish = vec![
        Fish::new("Tuna", 65, 0),
        Fish::new("Lobster", 74, 68),
        Fish::new("Swordfish", 86, 81),
        Fish::new("Monkfish", 90, 90),
        Fish::new("Shark", 100, 94),
        Fish::new("Anglerfish", 0, 98),
    ];
    let output: Vec<String> = fish.into_iter().map(|fish| fish.to_string()).collect();
    return Ok(vec![format!("{} {}", l("NoBurn"), output.join(&c1(" | ")))]);
}

struct Fish {
    name: &'static str,
    normal: u32,
    gauntlet: u32,
}

impl Fish {
    fn new(name: &'static str, normal: u32, gauntlet: u32) -> Self {
        Self {
            name,
            normal,
            gauntlet,
        }
    }
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            c1(self.name),
            c2(&self.normal.to_string()),
            p(&self.gauntlet.to_string())
        )
    }
}
