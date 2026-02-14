use common::source::Source;
use common::{commas, remove_trailing_zeroes};
use serde::{Deserialize, Serialize};

pub fn lookup(s: &Source) -> Result<Vec<String>, ()> {
    let prefix = s.l("Plant");

    if s.query.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, s.c1("No query provided"))]);
    }

    for plant in Plant::all() {
        let details = plant.details();

        if details
            .name
            .to_ascii_lowercase()
            .contains(&s.query.to_ascii_lowercase())
        {
            return Ok(vec![format!("{} {}", prefix, details.to_string(s),)]);
        }
    }

    Ok(vec![format!("{} {}", prefix, s.c1("No results found"))])
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
    Acorn,
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
    WhiteLily,
    Attas,
    Iasor,
    Kronos,
    CrystalTree,
    Hespori,
    Elkhorn,
    Pillar,
    Umbral,
    Camphor,
    Ironwood,
    Rosewood,
    Flax,
    Hemp,
    Cotton,
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
            Self::Acorn,
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
            Self::WhiteLily,
            Self::Attas,
            Self::Iasor,
            Self::Kronos,
            Self::CrystalTree,
            Self::Hespori,
            Self::Elkhorn,
            Self::Pillar,
            Self::Umbral,
            Self::Camphor,
            Self::Ironwood,
            Self::Rosewood,
            Self::Flax,
            Self::Hemp,
            Self::Cotton,
        ]
    }

    fn details(&self) -> PlantDetails {
        match self {
            Self::Potato => {
                PlantDetails::from("Potato", 1, 35.0, 8.0, 0.0, 0.0, "2 Buckets of Compost")
            }
            Self::Onion => {
                PlantDetails::from("Onion", 5, 35.0, 9.5, 0.0, 10.5, "1 Sack of Potatoes")
            }
            Self::Cabbage => {
                PlantDetails::from("Cabbage", 7, 35.0, 10.0, 0.0, 11.5, "1 Sack of Onions")
            }
            Self::Tomatoes => {
                PlantDetails::from("Tomatoes", 12, 35.0, 12.5, 0.0, 14.0, "2 Sacks of Cabbage")
            }
            Self::Sweetcorn => {
                PlantDetails::from("Sweetcorn", 20, 55.0, 17.0, 0.0, 19.0, "10 Jute Fibres")
            }
            Self::Strawberries => PlantDetails::from(
                "Strawberries",
                31,
                55.0,
                26.0,
                0.0,
                29.0,
                "1 Basket of Apples",
            ),
            Self::Watermelon => {
                PlantDetails::from("Watermelon", 47, 75.0, 48.5, 0.0, 54.5, "10 Curry Leaves")
            }
            Self::SnapeGrass => {
                PlantDetails::from("Snape Grass", 61, 75.0, 82.0, 0.0, 82.0, "5 Jangerberries")
            }
            Self::Marigold => PlantDetails::from("Marigold", 2, 17.5, 8.5, 0.0, 47.0, ""),
            Self::Rosemary => PlantDetails::from("Rosemary", 11, 17.5, 12.0, 0.0, 66.5, ""),
            Self::Nasturtium => PlantDetails::from("Nasturtium", 24, 17.5, 19.5, 0.0, 111.0, ""),
            Self::Woad => PlantDetails::from("Woad", 25, 17.5, 20.5, 0.0, 115.5, ""),
            Self::LimpwurtPlant => {
                PlantDetails::from("Limpwurt Plant", 26, 17.5, 21.5, 0.0, 120.0, "")
            }
            Self::RedberryBush => PlantDetails::from(
                "Redberry Bush",
                10,
                90.0,
                11.5,
                64.0,
                4.5,
                "4 Sacks of Cabbage",
            ),
            Self::CadavaberryBush => PlantDetails::from(
                "Cadavaberry Bush",
                22,
                110.0,
                18.0,
                102.5,
                7.0,
                "3 Baskets of Tomatoes",
            ),
            Self::DwellberryBush => PlantDetails::from(
                "Dwellberry Bush",
                36,
                130.0,
                31.5,
                177.5,
                12.0,
                "3 Baskets of Strawberries",
            ),
            Self::JangerberryBush => PlantDetails::from(
                "Jangerberry Bush",
                48,
                150.0,
                50.5,
                284.5,
                19.0,
                "6 Watermelons",
            ),
            Self::WhiteberryBush => PlantDetails::from(
                "Whiteberry Bush",
                59,
                150.0,
                78.0,
                437.5,
                29.0,
                "8 Mushrooms",
            ),
            Self::PoisonIvyBush => {
                PlantDetails::from("Poison Ivy Bush", 70, 150.0, 120.0, 674.0, 45.0, "")
            }
            Self::Acorn => {
                PlantDetails::from("Acorn", 15, 220.0, 14.0, 467.3, 0.0, "1 Basket of Tomatoes")
            }
            Self::OakTree => PlantDetails::from(
                "Oak Tree",
                15,
                220.0,
                14.0,
                467.3,
                0.0,
                "1 Basket of Tomatoes",
            ),
            Self::WillowTree => PlantDetails::from(
                "Willow Tree",
                30,
                220.0,
                25.0,
                1456.3,
                0.0,
                "1 Basket of Apples",
            ),
            Self::MapleTree => PlantDetails::from(
                "Maple Tree",
                45,
                300.0,
                45.0,
                3403.4,
                0.0,
                "1 Basket of Oranges",
            ),
            Self::YewTree => {
                PlantDetails::from("Yew Tree", 60, 380.0, 81.0, 7069.9, 0.0, "10 Cactus Spines")
            }
            Self::MagicTree => {
                PlantDetails::from("Magic Tree", 75, 460.0, 145.5, 13768.3, 0.0, "25 Coconuts")
            }
            Self::AppleTree => PlantDetails::from(
                "Apple Tree",
                27,
                960.0,
                22.0,
                1199.5,
                8.5,
                "9 Raw Sweetcorn",
            ),
            Self::BananaTree => PlantDetails::from(
                "Banana Tree",
                33,
                960.0,
                28.0,
                1750.5,
                10.5,
                "4 Baskets of Apples",
            ),
            Self::OrangeTree => PlantDetails::from(
                "Orange Tree",
                39,
                960.0,
                35.5,
                2470.2,
                13.5,
                "3 Baskets of Strawberries",
            ),
            Self::CurryTree => PlantDetails::from(
                "Curry Tree",
                42,
                960.0,
                40.0,
                2906.9,
                15.0,
                "5 Baskets of Bananas",
            ),
            Self::PineappleTree => PlantDetails::from(
                "Pineapple Tree",
                51,
                960.0,
                57.0,
                4605.7,
                21.5,
                "10 Watermelons",
            ),
            Self::PapayaTree => PlantDetails::from(
                "Papaya Tree",
                57,
                960.0,
                72.0,
                6146.4,
                27.0,
                "10 Pineapples",
            ),
            Self::PalmTree => {
                PlantDetails::from("Palm Tree", 68, 960.0, 110.5, 10150.1, 42.0, "15 Papayas")
            }
            Self::Barley => {
                PlantDetails::from("Barley", 3, 35.0, 8.5, 0.0, 9.5, "3 Buckets of Compost")
            }
            Self::HammerstoneHops => PlantDetails::from(
                "Hammerstone Hops",
                4,
                35.0,
                9.0,
                0.0,
                10.0,
                "1 Bunch of Marigolds",
            ),
            Self::AsgarnianHops => PlantDetails::from(
                "Asgarnian Hops",
                8,
                45.0,
                10.5,
                0.0,
                12.0,
                "1 Sack of Onions",
            ),
            Self::JutePlant => PlantDetails::from(
                "Jute Plant",
                13,
                45.0,
                13.0,
                0.0,
                14.5,
                "6 Handfuls of Barley Malt",
            ),
            Self::YanillianHops => PlantDetails::from(
                "Yanillian Hops",
                16,
                55.0,
                14.5,
                0.0,
                16.0,
                "1 Basket of Tomatoes",
            ),
            Self::KrandorianHops => PlantDetails::from(
                "Krandorian Hops",
                21,
                65.0,
                17.5,
                0.0,
                19.5,
                "3 Sacks of Cabbage",
            ),
            Self::WildbloodHops => {
                PlantDetails::from("Wildblood Hops", 28, 75.0, 23.0, 0.0, 26.0, "1 Nasturtium")
            }
            Self::Guam => PlantDetails::from("Guam", 9, 75.0, 11.0, 0.0, 12.5, ""),
            Self::Marrentill => PlantDetails::from("Marrentill", 14, 75.0, 13.5, 0.0, 15.0, ""),
            Self::Tarromin => PlantDetails::from("Tarromin", 19, 75.0, 16.0, 0.0, 18.0, ""),
            Self::Harralander => PlantDetails::from("Harralander", 26, 75.0, 21.5, 0.0, 24.0, ""),
            Self::Goutweed => PlantDetails::from("Goutweed", 29, 75.0, 105.0, 0.0, 45.0, ""),
            Self::RanarrWeed => PlantDetails::from("Ranarr Weed", 32, 75.0, 27.0, 0.0, 30.5, ""),
            Self::Toadflax => PlantDetails::from("Toadflax", 38, 75.0, 34.0, 0.0, 38.5, ""),
            Self::IritLeaf => PlantDetails::from("Irit Leaf", 44, 75.0, 43.0, 0.0, 48.5, ""),
            Self::Avantoe => PlantDetails::from("Avantoe", 50, 75.0, 54.5, 0.0, 61.5, ""),
            Self::Kwuarm => PlantDetails::from("Kwuarm", 56, 75.0, 69.0, 0.0, 78.0, ""),
            Self::Snapdragon => PlantDetails::from("Snapdragon", 62, 75.0, 87.5, 0.0, 98.5, ""),
            Self::Cadantine => PlantDetails::from("Cadantine", 67, 75.0, 106.5, 0.0, 120.0, ""),
            Self::Lantadyme => PlantDetails::from("Lantadyme", 73, 75.0, 134.5, 0.0, 151.5, ""),
            Self::DwarfWeed => PlantDetails::from("Dwarf Weed", 79, 75.0, 170.5, 0.0, 192.0, ""),
            Self::Torstol => PlantDetails::from("Torstol", 85, 75.0, 199.5, 0.0, 224.5, ""),
            Self::BittercapMushroom => {
                PlantDetails::from("Bittercap Mushroom", 53, 220.0, 61.5, 0.0, 57.7, "")
            }
            Self::Cactus => {
                PlantDetails::from("Cactus", 55, 520.0, 66.5, 374.0, 25.0, "6 Cadava berries")
            }
            Self::PotatoCactus => PlantDetails::from(
                "Potato Cactus",
                64,
                70.0,
                68.0,
                230.0,
                68.0,
                "8 Snape grass",
            ),
            Self::Belladonna => PlantDetails::from("Belladonna", 63, 320.0, 91.0, 0.0, 512.0, ""),
            Self::CalquatTree => PlantDetails::from(
                "Calquat Tree",
                72,
                1200.0,
                129.5,
                12096.0,
                48.5,
                "8 Poison ivy berries",
            ),
            Self::SpiritTree => PlantDetails::from(
                "Spirit Tree",
                83,
                3680.0,
                199.5,
                19301.8,
                0.0,
                "1 Ground Suqah tooth, 5 Monkey nuts, 1 Monkey bar",
            ),
            Self::DragonfruitTree => PlantDetails::from(
                "Dragonfruit Tree",
                81,
                960.0,
                140.0,
                17335.0,
                70.0,
                "15 Coconuts",
            ),
            Self::CelastrusTree => PlantDetails::from(
                "Celastrus Tree",
                85,
                800.0,
                200.0,
                14130.0,
                23.5,
                "8 Potato cactus",
            ),
            Self::RedwoodTree => PlantDetails::from(
                "Redwood Tree",
                90,
                6400.0,
                230.0,
                22450.0,
                0.0,
                "6 Dragonfruits",
            ),
            Self::TeakTree => PlantDetails::from(
                "Teak Tree",
                35,
                4480.0,
                35.0,
                7290.0,
                0.0,
                "15 Limpwurt roots",
            ),
            Self::MahoganyTree => PlantDetails::from(
                "Mahogany Tree",
                55,
                5120.0,
                68.0,
                15720.0,
                0.0,
                "25 Yanillian hops",
            ),
            Self::WhiteLily => PlantDetails::from("White Lily", 58, 20.0, 42.0, 0.0, 250.0, ""),
            Self::CrystalTree => {
                PlantDetails::from("Crystal Tree", 74, 480.0, 126.0, 13240.0, 0.0, "")
            }
            Self::Attas => PlantDetails::from("Attas Plant", 76, 5120.0, 100.0, 0.0, 0.0, ""),
            Self::Iasor => PlantDetails::from("Iasor Plant", 76, 5120.0, 100.0, 0.0, 0.0, ""),
            Self::Kronos => PlantDetails::from("Kronos Plant", 76, 5120.0, 100.0, 0.0, 0.0, ""),
            Self::Hespori => PlantDetails::from("Hespori", 65, 1920.0, 62.0, 0.0, 12600.0, ""),
            Self::Elkhorn => PlantDetails::from(
                "Elkhorn Coral",
                28,
                160.0,
                20.5,
                0.0,
                24.0,
                "5 Giant seaweed",
            ),
            Self::Pillar => {
                PlantDetails::from("Pillar Coral", 52, 160.0, 0.5, 0.0, 60.0, "5 Elkhorn coral")
            }
            Self::Umbral => PlantDetails::from(
                "Umbral Coral",
                77,
                160.0,
                136.0,
                0.0,
                159.0,
                "5 Pillar seaweed",
            ),
            Self::Camphor => PlantDetails::from(
                "Camphor Tree",
                66,
                5120.0,
                88.0,
                0.0,
                0.0,
                "10 White berries",
            ),
            Self::Ironwood => PlantDetails::from(
                "Ironwood Tree",
                80,
                5120.0,
                0.0,
                0.0,
                0.0,
                "10 Curry leaves",
            ),
            Self::Rosewood => {
                PlantDetails::from("Rosewood Tree", 92, 6400.0, 0.0, 0.0, 0.0, "8 Dragonfruit")
            }
            Self::Flax => PlantDetails::from("Flax", 18, 60.0, 16.0, 0.0, 17.5, "6 Grain"),
            Self::Hemp => PlantDetails::from("Hemp", 37, 80.0, 33.0, 0.0, 37.0, "6 Flax"),
            Self::Cotton => PlantDetails::from("Cotton", 71, 100.0, 72.0, 0.0, 0.0, "6 Hemp"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PlantDetails {
    name: String,
    level: u32,
    time: f64,
    planting_xp: f64,
    checking_xp: f64,
    harvesting_xp: f64,
    payment: String,
}

impl PlantDetails {
    fn from<T>(
        name: T,
        level: u32,
        time: f64,
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

    fn name(&self) -> String {
        self.name.replace("_", " ")
    }

    fn to_string(&self, s: &Source) -> String {
        vec![
            s.p(&self.name()),
            s.c1("Level:"),
            s.c2(&self.level.to_string()),
            s.c1("Time:"),
            s.c2(&self.time.to_string()),
            s.c1("Planting XP:"),
            s.c2(&zero_or_na(self.planting_xp)),
            s.c1("Checking XP:"),
            s.c2(&zero_or_na(self.checking_xp)),
            s.c1("Harvesting XP:"),
            s.c2(&zero_or_na(self.harvesting_xp)),
            s.c1("Payment:"),
            if self.payment.len() > 0 {
                s.c2(&self.payment.to_string())
            } else {
                s.c2("N/A")
            },
        ]
        .join(" ")
    }
}

fn round(num: f64) -> String {
    remove_trailing_zeroes(&commas(f64::round(num * 10.0) / 10.0, "f"))
}

fn zero_or_na(num: f64) -> String {
    if num > 0.0 {
        round(num)
    } else {
        "N/A".to_string()
    }
}
