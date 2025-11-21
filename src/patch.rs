use common::{c2, l, not_found, p};
use std::fmt;
use std::str::FromStr;

pub fn patch(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Patch");
    let patch: Patch = query.parse()?;

    let locations = patch
        .locations()
        .iter()
        .map(|location| c2(&location))
        .collect();

    let output = format!(
        "{} {}{}",
        prefix,
        format_patch(&patch.to_string()),
        not_found(locations)
    );

    Ok(vec![output])
}

fn format_patch(patch: &str) -> String {
    if !patch.is_empty() {
        format!("{} ", p(&patch))
    } else {
        "".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Patch {
    Allotment,
    Flower,
    Herb,
    Bush,
    Tree,
    Fruit,
    Hops,
    Spirit,
    Belladonna,
    Calquat,
    Mushroom,
    Celastrus,
    Redwood,
    Crystal,
    Seaweed,
    Grape,
    Hespori,
    Anima,
    Cactus,
    Hardwood,
    Coral,
    None,
}

impl Patch {
    fn all() -> Vec<Self> {
        vec![
            Self::Allotment,
            Self::Flower,
            Self::Herb,
            Self::Bush,
            Self::Tree,
            Self::Fruit,
            Self::Hops,
            Self::Spirit,
            Self::Belladonna,
            Self::Calquat,
            Self::Mushroom,
            Self::Celastrus,
            Self::Redwood,
            Self::Crystal,
            Self::Seaweed,
            Self::Grape,
            Self::Hespori,
            Self::Anima,
            Self::Cactus,
            Self::Hardwood,
            Self::Coral,
            Self::None,
        ]
    }

    fn locations(&self) -> &'static [&'static str] {
        match self {
            Self::Allotment => &[
                "Falador South",
                "Port Phasmatys",
                "Harmony Island",
                "Catherby North",
                "Ardougne North",
                "Farming Guild",
                "Hosidius South-west",
                "Civitas illa Fortis West",
            ],
            Self::Flower => &[
                "Falador South",
                "Port Phasmatys",
                "Catherby North",
                "Ardougne North",
                "Farming Guild",
                "Hosidius South-west",
                "Civitas illa Fortis West",
            ],
            Self::Herb => &[
                "Falador South",
                "Troll Stronghold Rooftop",
                "Port Phasmatys",
                "Catherby North",
                "Ardougne North",
                "Farming Guild",
                "Weiss",
                "Harmony Island",
                "Varlamore",
                "Hosidius South-west",
                "Civitas illa Fortis West",
            ],
            Self::Bush => &[
                "Champions' Guild",
                "Rimmington",
                "Ardougne South",
                "Etceteria South-west",
                "Farming Guild",
            ],
            Self::Tree => &[
                "Lumbridge West",
                "Varrock Castle",
                "Falador Park",
                "Taverley",
                "Gnome Stronghold",
                "Farming Guild",
                "Auburnvale",
            ],
            Self::Fruit => &[
                "Catherby East",
                "Tree Gnome Maze West",
                "Brimhaven North",
                "Gnome Stronghold",
                "Lletya",
                "Farming Guild",
            ],
            Self::Hops => &[
                "Lumbridge North",
                "McGrubor's Woods North",
                "Yanille",
                "Entrana",
                "Aldarin",
            ],
            Self::Spirit => &[
                "Etceteria South-east",
                "Port Sarim East",
                "Brimhaven East",
                "Farming Guild",
                "Hosidius South-west",
            ],
            Self::Belladonna => &["Draynor Village Manor"],
            Self::Calquat => &["Tai Bwo Wannai North", "The Great Conch"],
            Self::Mushroom => &["Canifis"],
            Self::Celastrus => &["Farming Guild"],
            Self::Redwood => &["Farming Guild"],
            Self::Crystal => &["Prifddinas North"],
            Self::Seaweed => &["Underwater (Fossil Island)"],
            Self::Grape => &["Hosidius Vinery"],
            Self::Hespori => &["Farming Guild"],
            Self::Anima => &["Farming Guild"],
            Self::Cactus => &["Al Kharid", "Farming Guild"],
            Self::Hardwood => &["Fossil Island (Mushroom Forest)", "Locus Oasis (Varlamore)"],
            Self::Coral => &["Coral Nurseries (East. Great Conch)"],
            Self::None => &[],
        }
    }
}

impl FromStr for Patch {
    type Err = ();

    fn from_str(query: &str) -> Result<Self, Self::Err> {
        Patch::all()
            .into_iter()
            .find(|patch| {
                patch
                    .to_string()
                    .to_lowercase()
                    .contains(&query.to_lowercase())
            })
            .ok_or(())
    }
}

impl fmt::Display for Patch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Patch::Allotment => "Allotment",
            Patch::Flower => "Flower",
            Patch::Herb => "Herb",
            Patch::Bush => "Bush",
            Patch::Tree => "Tree",
            Patch::Fruit => "Fruit",
            Patch::Hops => "Hops",
            Patch::Spirit => "Spirit",
            Patch::Belladonna => "Belladonna",
            Patch::Calquat => "Calquat",
            Patch::Mushroom => "Mushroom",
            Patch::Celastrus => "Celastrus",
            Patch::Redwood => "Redwood",
            Patch::Crystal => "Crystal",
            Patch::Seaweed => "Seaweed",
            Patch::Grape => "Grape",
            Patch::Hespori => "Hespori",
            Patch::Anima => "Anima",
            Patch::Cactus => "Cactus",
            Patch::Hardwood => "Hardwood",
            Patch::Coral => "Coral",
            Patch::None => "",
        };
        write!(f, "{}", s)
    }
}
