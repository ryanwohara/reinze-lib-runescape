use common::{c2, convert_split_to_string, l, not_found, p};

pub fn patch(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Patch");
    let patch = Patch::from(query);

    let locations = patch.locations().into_iter().map(|location| c2(&location)).collect();

    let output = format!(
        "{} {}{}",
        prefix,
        format_patch(&patch.to_string()),
        not_found(locations)
    );

    Ok(vec![output])
}

fn format_patch(patch: &str) -> String {
    if patch.len() > 0 {
        format!("{} ", p(&patch))
    } else {
        "".to_string()
    }
}

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
            Self::None,
        ]
    }

    fn to_string(&self) -> String {
        match self {
            Self::Allotment => "Allotment",
            Self::Flower => "Flower",
            Self::Herb => "Herb",
            Self::Bush => "Bush",
            Self::Tree => "Tree",
            Self::Fruit => "Fruit",
            Self::Hops => "Hops",
            Self::Spirit => "Spirit",
            Self::Belladonna => "Belladonna",
            Self::Calquat => "Calquat",
            Self::Mushroom => "Mushroom",
            Self::Celastrus => "Celastrus",
            Self::Redwood => "Redwood",
            Self::Crystal => "Crystal",
            Self::Seaweed => "Seaweed",
            Self::Grape => "Grape",
            Self::Hespori => "Hespori",
            Self::Anima => "Anima",
            Self::Cactus => "Cactus",
            Self::Hardwood => "Hardwood",
            Self::None => "",
        }
        .to_string()
    }

    fn locations(&self) -> Vec<String> {
        let locations = match self {
            Self::Allotment => vec![
                "Falador South",
                "Port Phasmatys",
                "Harmony Island",
                "Catherby North",
                "Ardougne North",
                "Farming Guild",
                "Hosidius South-west",
                "Civitas illa Fortis West",
            ],
            Self::Flower => vec![
                "Falador South",
                "Port Phasmatys",
                "Catherby North",
                "Ardougne North",
                "Farming Guild",
                "Hosidius South-west",
                "Civitas illa Fortis West",
            ],
            Self::Herb => vec![
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
            Self::Bush => vec![
                "Champions' Guild",
                "Rimmington",
                "Ardougne South",
                "Etceteria South-west",
                "Farming Guild",
            ],
            Self::Tree => vec![
                "Lumbridge West",
                "Varrock Castle",
                "Falador Park",
                "Taverley",
                "Gnome Stronghold",
                "Farming Guild",
                "Auburnvale"
            ],
            Self::Fruit => vec![
                "Catherby East",
                "Tree Gnome Maze West",
                "Brimhaven North",
                "Gnome Stronghold",
                "Lletya",
                "Farming Guild",
            ],
            Self::Hops => vec![
                "Lumbridge North",
                "McGrubor's Woods North",
                "Yanille",
                "Entrana",
                "Aldarin",
            ],
            Self::Spirit => vec![
                "Etceteria South-east",
                "Port Sarim East",
                "Brimhaven East",
                "Farming Guild",
                "Hosidius South-west",
            ],
            Self::Belladonna => vec!["Draynor Village Manor"],
            Self::Calquat => vec!["Tai Bwo Wannai North"],
            Self::Mushroom => vec!["Canifis"],
            Self::Celastrus => vec!["Farming Guild"],
            Self::Redwood => vec!["Farming Guild"],
            Self::Crystal => vec!["Prifddinas North"],
            Self::Seaweed => vec!["Underwater (Fossil Island)"],
            Self::Grape => vec!["Hosidius Vinery"],
            Self::Hespori => vec!["Farming Guild"],
            Self::Anima => vec!["Farming Guild"],
            Self::Cactus => vec!["Al Kharid", "Farming Guild"],
            Self::Hardwood => vec!["Fossil Island (Mushroom Forest)", "Locus Oasis (Varlamore)"],
            Self::None => vec![],
        };

        convert_split_to_string(locations)
    }

    fn from(query: &str) -> Self {
        let patches = Self::all();

        for patch in patches {
            if patch
                .to_string()
                .to_lowercase()
                .contains(&query.to_lowercase())
            {
                return patch;
            }
        }

        Patch::None
    }
}
