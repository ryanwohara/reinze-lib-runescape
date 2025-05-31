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

    let mut found_params: Vec<PlantDetails> = vec![];

    for (k, v) in section.iter() {
        if k.to_ascii_lowercase()
            .contains(&underscored.to_ascii_lowercase())
        {
            let split = v.split(",").collect::<Vec<&str>>();
            let plant = PlantDetails::from(
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

enum Plant {
    Potato,
    Onion,
    Cabbage,
    Tomatoes,
    Sweetcorn,
    Strawberries,
    Watermelon,
    SnapeGrass,
    Marigold,
    Rosemary,
    Nasturtium,
    Woad,
    LimpwurtPlant,
    RedberryBush,
    CadavaberryBush,
    DwellberryBush,
    JangerberryBush,
    WhiteberryBush,
    PoisonIvyBush,
    OakTree,
    WillowTree,
    MapleTree,
    YewTree,
    MagicTree,
    AppleTree,
    BananaTree,
    OrangeTree,
    CurryTree,
    PineappleTree,
    PapayaTree,
    PalmTree,
    Barley,
    HammerstoneHops,
    AsgarnianHops,
    JutePlant,
    YanillianHops,
    KrandorianHops,
    WildbloodHops,
    Guam,
    Marrentill,
    Tarromin,
    Harralander,
    Goutweed,
    RanarrWeed,
    Toadflax,
    IritLeaf,
    Avantoe,
    Kwuarm,
    Snapdragon,
    Cadantine,
    Lantadyme,
    DwarfWeed,
    Torstol,
    BittercapMushroom,
    Cactus,
    PotatoCactus,
    Belladonna,
    CalquatTree,
    SpiritTree,
    DragonfruitTree,
    CelastrusTree,
    RedwoodTree,
    TeakTree,
    MahoganyTree,
    CrystalTree,
    Hespori,
}

impl Plant {
    fn all() -> Vec<Self> {
        vec![
            Self::Potato,
            Self::Onion,
            Self::Cabbage,
            Self::Tomatoes,
            Self::Sweetcorn,
            Self::Strawberries,
            Self::Watermelon,
            Self::SnapeGrass,
            Self::Marigold,
            Self::Rosemary,
            Self::Nasturtium,
            Self::Woad,
            Self::LimpwurtPlant,
            Self::RedberryBush,
            Self::CadavaberryBush,
            Self::DwellberryBush,
            Self::JangerberryBush,
            Self::WhiteberryBush,
            Self::PoisonIvyBush,
            Self::OakTree,
            Self::WillowTree,
            Self::MapleTree,
            Self::YewTree,
            Self::MagicTree,
            Self::AppleTree,
            Self::BananaTree,
            Self::OrangeTree,
            Self::CurryTree,
            Self::PineappleTree,
            Self::PapayaTree,
            Self::PalmTree,
            Self::Barley,
            Self::HammerstoneHops,
            Self::AsgarnianHops,
            Self::JutePlant,
            Self::YanillianHops,
            Self::KrandorianHops,
            Self::WildbloodHops,
            Self::Guam,
            Self::Marrentill,
            Self::Tarromin,
            Self::Harralander,
            Self::Goutweed,
            Self::RanarrWeed,
            Self::Toadflax,
            Self::IritLeaf,
            Self::Avantoe,
            Self::Kwuarm,
            Self::Snapdragon,
            Self::Cadantine,
            Self::Lantadyme,
            Self::DwarfWeed,
            Self::Torstol,
            Self::BittercapMushroom,
            Self::Cactus,
            Self::PotatoCactus,
            Self::Belladonna,
            Self::CalquatTree,
            Self::SpiritTree,
            Self::DragonfruitTree,
            Self::CelastrusTree,
            Self::RedwoodTree,
            Self::TeakTree,
            Self::MahoganyTree,
            Self::CrystalTree,
            Self::Hespori,
        ]
    }

    fn details(&self) -> PlantDetails {
        match self {
            Self::Potato => {
                PlantDetails::from("Potato", 1, 35, 8.0, 0.0, 0.0, "2 Buckets of Compost")
            }
            Self::Onion => PlantDetails::from("Onion", 5, 35, 9.5, 0.0, 10.5, "1 Sack of Potatoes"),
            Plant::Cabbage => { PlantDetails::from("Cabbage", 7, 35, 10, -, 11.5, "1 Sack of Onions") }
            Plant::Tomatoes => { PlantDetails::from("Tomatoes", 12, 35, 12.5, -, 14, "2 Sacks of Cabbage") }
            Plant::Sweetcorn => { PlantDetails::from("Sweetcorn", 20, 55, 17, -, 19, "10 Jute Fibres") }
            Plant::Strawberries => { PlantDetails::from("Strawberries", 31, 55, 26, -, 29, "1 Basket of Apples") }
            Plant::Watermelon => { PlantDetails::from("Watermelon", 47, 75, 48.5, -, 54.5, "10 Curry Leaves") }
            Plant::SnapeGrass => { PlantDetails::from("Snape_Grass", 61, 75, 82, -, 82, "5 Jangerberries") }
            Plant::Marigold => { PlantDetails::from("Marigold", 2, 17.5, 8.5, -, 47) }
            Plant::Rosemary => { PlantDetails::from("Rosemary", 11, 17.5, 12, -, 66.5) }
            Plant::Nasturtium => { PlantDetails::from("Nasturtium", 24, 17.5, 19.5, -, 111) }
            Plant::Woad => { PlantDetails::from("Woad", 25, 17.5, 20.5, -, 115.5) }
            Plant::LimpwurtPlant => { PlantDetails::from("Limpwurt_Plant", 26, 17.5, 21.5, -, 120) }
            Plant::RedberryBush => { PlantDetails::from("Redberry_Bush", 10, 90, 11.5, 64, 4.5, "4 Sacks of Cabbage") }
            Plant::CadavaberryBush => { PlantDetails::from("Cadavaberry_Bush", 22, 110, 18.0, 102.5, 7, "3 Baskets of Tomatoes") }
            Plant::DwellberryBush => { PlantDetails::from("Dwellberry_Bush", 36, 130, 31.5, 177.5, 12, "3 Baskets of Strawberries") }
            Plant::JangerberryBush => { PlantDetails::from("Jangerberry_Bush", 48, 150, 50.5, 284.5, 19, "6 Watermelons") }
            Plant::WhiteberryBush => { PlantDetails::from("Whiteberry_Bush", 59, 150, 78, 437.5, 29, "8 Mushrooms") }
            Plant::PoisonIvyBush => { PlantDetails::from("Poison_Ivy_Bush", 70, 150, 120, 674, 45) }
            Plant::OakTree => { PlantDetails::from("Oak_Tree", 15, 220, 14, 467.3, -, "1 Basket of Tomatoes") }
            Plant::WillowTree => { PlantDetails::from("Willow_Tree", 30, 220, 25, 1456.3, -, "1 Basket of Apples") }
            Plant::MapleTree => { PlantDetails::from("Maple_Tree", 45, 300, 45, 3403.4, -, "1 Basket of Oranges") }
            Plant::YewTree => { PlantDetails::from("Yew_Tree", 60, 380, 81, 7069.9, -, "10 Cactus Spines") }
            Plant::MagicTree => { PlantDetails::from("Magic_Tree", 75, 460, 145.5, 13768.3, -, "25 Coconuts") }
            Plant::AppleTree => { PlantDetails::from("Apple_Tree", 27, 960, 22, 1199.5, 8.5, "9 Raw Sweetcorn") }
            Plant::BananaTree => { PlantDetails::from("Banana_Tree", 33, 960, 28, 1750.5, 10.5, "4 Baskets of Apples") }
            Plant::OrangeTree => { PlantDetails::from("Orange_Tree", 39, 960, 35.5, 2470.2, 13.5, "3 Baskets of Strawberries") }
            Plant::CurryTree => { PlantDetails::from("Curry_Tree", 42, 960, 40, 2906.9, 15, "5 Baskets of Bananas") }
            Plant::PineappleTree => { PlantDetails::from("Pineapple_Tree", 51, 960, 57, 4605.7, 21.5, "10 Watermelons") }
            Plant::PapayaTree => { PlantDetails::from("Papaya_Tree", 57, 960, 72, 6146.4, 27, "10 Pineapples") }
            Plant::PalmTree => { PlantDetails::from("Palm_Tree", 68, 960, 110.5, 10150.1, 42, "15 Papayas") }
            Plant::Barley => { PlantDetails::from("Barley", 3, 35, 8.5, -, 9.5, "3 Buckets of Compost") }
            Plant::HammerstoneHops => { PlantDetails::from("Hammerstone_Hops", 4, 35, 9, -, 10, "1 Bunch of Marigolds") }
            Plant::AsgarnianHops => { PlantDetails::from("Asgarnian_Hops", 8, 45, 10.5, -, 12, "1 Sack of Onions") }
            Plant::JutePlant => { PlantDetails::from("Jute_Plant", 13, 45, 13, -, 14.5, "6 Handfuls of Barley Malt") }
            Plant::YanillianHops => { PlantDetails::from("Yanillian_Hops", 16, 55, 14.5, -, 16, "1 Basket of Tomatoes") }
            Plant::KrandorianHops => { PlantDetails::from("Krandorian_Hops", 21, 65, 17.5, -, 19.5, "3 Sacks of Cabbage") }
            Plant::WildbloodHops => { PlantDetails::from("Wildblood_Hops", 28, 75, 23, -, 26, "1 Nasturtium") }
            Plant::Guam => { PlantDetails::from("Guam", 9, 75, 11, -, 12.5) }
            Plant::Marrentill => { PlantDetails::from("Marrentill", 14, 75, 13.5, -, 15) }
            Plant::Tarromin => { PlantDetails::from("Tarromin", 19, 75, 16, -, 18) }
            Plant::Harralander => { PlantDetails::from("Harralander", 26, 75, 21.5, -, 24) }
            Plant::Goutweed => { PlantDetails::from("Goutweed", 29, 75, 105, -, 45) }
            Plant::RanarrWeed => { PlantDetails::from("Ranarr_Weed", 32, 75, 27, -, 30.5) }
            Plant::Toadflax => { PlantDetails::from("Toadflax", 38, 75, 34, -, 38.5) }
            Plant::IritLeaf => { PlantDetails::from("Irit_Leaf", 44, 75, 43, -, 48.5) }
            Plant::Avantoe => { PlantDetails::from("Avantoe", 50, 75, 54.5, -, 61.5) }
            Plant::Kwuarm => { PlantDetails::from("Kwuarm", 56, 75, 69, -, 78) }
            Plant::Snapdragon => { PlantDetails::from("Snapdragon", 62, 75, 87.5, -, 98.5) }
            Plant::Cadantine => { PlantDetails::from("Cadantine", 67, 75, 106.5, -, 120) }
            Plant::Lantadyme => { PlantDetails::from("Lantadyme", 73, 75, 134.5, -, 151.5) }
            Plant::DwarfWeed => { PlantDetails::from("Dwarf_Weed", 79, 75, 170.5, -, 192) }
            Plant::Torstol => { PlantDetails::from("Torstol", 85, 75, 199.5, -, 224.5) }
            Plant::BittercapMushroom => { PlantDetails::from("Bittercap_Mushroom", 53, 220, 61.5, -, 57.7) }
            Plant::Cactus => { PlantDetails::from("Cactus", 55, 520, 66.5, 374, 25, "6 Cadava berries") }
            Plant::PotatoCactus => { PlantDetails::from("Potato_Cactus", 64, 70, 68, 230, 68, "8 Snape grass") }
            Plant::Belladonna => { PlantDetails::from("Belladonna", 63, 320, 91, -, 512) }
            Plant::CalquatTree => { PlantDetails::from("Calquat_Tree", 72, 1200, 129.5, 12096, 48.5, "8 Poison ivy berries") }
            Plant::SpiritTree => { PlantDetails::from("Spirit_Tree", 83, 3680, 199.5, 19301.8, -, "1 Ground Suqah tooth, 5 Monkey nuts, 1 Monkey bar") }
            Plant::DragonfruitTree => { PlantDetails::from("Dragonfruit_Tree", 81, 960, 140, 17335, 70, "15 Coconuts") }
            Plant::CelastrusTree => { PlantDetails::from("Celastrus_Tree", 85, 800, 200, 14130, 23.5, "8 Potato cactus") }
            Plant::RedwoodTree => { PlantDetails::from("Redwood_Tree", 90, 6400, 230, 22450, -, "6 Dragonfruits") }
            Plant::TeakTree => { PlantDetails::from("Teak_Tree", 35, 4480, 35, 7290, -, "15 Limpwurt roots") }
            Plant::MahoganyTree => { PlantDetails::from("Mahogany_Tree", 55, 5120, 68, 15720, -, "25 Yanillian hops") }
            Plant::CrystalTree => { PlantDetails::from("Crystal_Tree", 74, 480, 126, 13240, -, -) }
            Plant::Hespori => { PlantDetails::from("Hespori", 65, 1920, 62, -, 12600, -, -) }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PlantDetails {
    name: String,
    level: u32,
    time: u32,
    planting_xp: f64,
    checking_xp: f64,
    harvesting_xp: f64,
    payment: String,
}

impl PlantDetails {
    fn from<T>(
        name: T,
        level: u32,
        time: u32,
        planting_xp: f64,
        checking_xp: f64,
        harvesting_xp: f64,
        payment: T,
    ) -> Self
    where
        T: ToString,
    {
        Self {
            name: name.to_string(),
            level,
            time,
            planting_xp,
            checking_xp,
            harvesting_xp,
            payment: payment.to_string(),
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
