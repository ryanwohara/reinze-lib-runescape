use anyhow::Result;
use common::source::Source;
use common::{commas, remove_trailing_zeroes};
use serde::{Deserialize, Serialize};

/// Names that are not a crop in their own right but are what a player would
/// type. `Acorn` is the seed an oak grows from, so it resolves to the oak
/// rather than carrying a duplicate set of numbers.
const ALIASES: [(&str, &str); 1] = [("acorn", "oak")];

fn resolve(query: &str) -> String {
    let query = query.trim().to_ascii_lowercase();

    ALIASES
        .iter()
        .find(|(alias, _)| *alias == query)
        .map_or(query.clone(), |(_, name)| name.to_string())
}

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let prefix = s.l("Plant");

    if s.query.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, s.c1("No query provided"))]);
    }

    let query = resolve(&s.query);

    for plant in Plant::all() {
        let details = plant.details();

        if details.name.to_ascii_lowercase().contains(&query) {
            return Ok(vec![format!("{} {}", prefix, details.to_string(s),)]);
        }
    }

    Ok(vec![format!("{} {}", prefix, s.c1("No results found"))])
}

enum Plant {
    Potato,
    Marigold,
    Barley,
    HammerstoneHops,
    Onion,
    Cabbage,
    AsgarnianHops,
    Guam,
    RedberryBush,
    Rosemary,
    Tomatoes,
    JutePlant,
    Marrentill,
    OakTree,
    YanillianHops,
    KeldaHops,
    Flax,
    Tarromin,
    Sweetcorn,
    KrandorianHops,
    CadavaberryBush,
    Seaweed,
    Nasturtium,
    Woad,
    YellowOrchids,
    Harralander,
    LimpwurtPlant,
    AppleTree,
    Golpar,
    Elkhorn,
    WildbloodHops,
    Goutweed,
    WillowTree,
    Strawberries,
    RanarrWeed,
    BananaTree,
    Golovanova,
    TeakTree,
    DwellberryBush,
    GrapeVine,
    Hemp,
    Toadflax,
    Buchu,
    OrangeTree,
    CurryTree,
    IritLeaf,
    MapleTree,
    Watermelon,
    JangerberryBush,
    Avantoe,
    PineappleTree,
    Pillar,
    BittercapMushroom,
    Bologano,
    Cactus,
    MahoganyTree,
    Noxifer,
    Kwuarm,
    PapayaTree,
    WhiteLily,
    WhiteberryBush,
    YewTree,
    SnapeGrass,
    Snapdragon,
    Belladonna,
    PotatoCactus,
    Hespori,
    Huasca,
    Camphor,
    Cadantine,
    PalmTree,
    PoisonIvyBush,
    Cotton,
    CalquatTree,
    Lantadyme,
    CrystalTree,
    Logavano,
    MagicTree,
    Attas,
    Iasor,
    Kronos,
    Umbral,
    DwarfWeed,
    Ironwood,
    DragonfruitTree,
    SpiritTree,
    CelastrusTree,
    Torstol,
    RedwoodTree,
    Rosewood,
}

impl Plant {
    fn all() -> Vec<Self> {
        vec![
            Self::Potato,
            Self::Marigold,
            Self::Barley,
            Self::HammerstoneHops,
            Self::Onion,
            Self::Cabbage,
            Self::AsgarnianHops,
            Self::Guam,
            Self::RedberryBush,
            Self::Rosemary,
            Self::Tomatoes,
            Self::JutePlant,
            Self::Marrentill,
            Self::OakTree,
            Self::YanillianHops,
            Self::KeldaHops,
            Self::Flax,
            Self::Tarromin,
            Self::Sweetcorn,
            Self::KrandorianHops,
            Self::CadavaberryBush,
            Self::Seaweed,
            Self::Nasturtium,
            Self::Woad,
            Self::YellowOrchids,
            Self::Harralander,
            Self::LimpwurtPlant,
            Self::AppleTree,
            Self::Golpar,
            Self::Elkhorn,
            Self::WildbloodHops,
            Self::Goutweed,
            Self::WillowTree,
            Self::Strawberries,
            Self::RanarrWeed,
            Self::BananaTree,
            Self::Golovanova,
            Self::TeakTree,
            Self::DwellberryBush,
            Self::GrapeVine,
            Self::Hemp,
            Self::Toadflax,
            Self::Buchu,
            Self::OrangeTree,
            Self::CurryTree,
            Self::IritLeaf,
            Self::MapleTree,
            Self::Watermelon,
            Self::JangerberryBush,
            Self::Avantoe,
            Self::PineappleTree,
            Self::Pillar,
            Self::BittercapMushroom,
            Self::Bologano,
            Self::Cactus,
            Self::MahoganyTree,
            Self::Noxifer,
            Self::Kwuarm,
            Self::PapayaTree,
            Self::WhiteLily,
            Self::WhiteberryBush,
            Self::YewTree,
            Self::SnapeGrass,
            Self::Snapdragon,
            Self::Belladonna,
            Self::PotatoCactus,
            Self::Hespori,
            Self::Huasca,
            Self::Camphor,
            Self::Cadantine,
            Self::PalmTree,
            Self::PoisonIvyBush,
            Self::Cotton,
            Self::CalquatTree,
            Self::Lantadyme,
            Self::CrystalTree,
            Self::Logavano,
            Self::MagicTree,
            Self::Attas,
            Self::Iasor,
            Self::Kronos,
            Self::Umbral,
            Self::DwarfWeed,
            Self::Ironwood,
            Self::DragonfruitTree,
            Self::SpiritTree,
            Self::CelastrusTree,
            Self::Torstol,
            Self::RedwoodTree,
            Self::Rosewood,
        ]
    }

    fn details(&self) -> PlantDetails {
        match self {
            Self::Potato => {
                PlantDetails::from("Potato", 1, 40.0, 8.0, 0.0, 9.0, "2 Buckets of Compost")
            }
            Self::Marigold => PlantDetails::from("Marigold", 2, 20.0, 8.5, 0.0, 47.0, ""),
            Self::Barley => {
                PlantDetails::from("Barley", 3, 40.0, 8.5, 0.0, 9.5, "3 Buckets of Compost")
            }
            Self::HammerstoneHops => PlantDetails::from(
                "Hammerstone Hops",
                4,
                40.0,
                9.0,
                0.0,
                10.0,
                "1 Bunch of Marigolds",
            ),
            Self::Onion => {
                PlantDetails::from("Onion", 5, 40.0, 9.5, 0.0, 10.5, "1 Sack of Potatoes")
            }
            Self::Cabbage => {
                PlantDetails::from("Cabbage", 7, 40.0, 10.0, 0.0, 11.5, "1 Sack of Onions")
            }
            Self::AsgarnianHops => PlantDetails::from(
                "Asgarnian Hops",
                8,
                50.0,
                10.9,
                0.0,
                12.0,
                "1 Sack of Onions",
            ),
            Self::Guam => PlantDetails::from("Guam", 9, 80.0, 11.0, 0.0, 12.5, ""),
            Self::RedberryBush => PlantDetails::from(
                "Redberry Bush",
                10,
                100.0,
                11.5,
                64.0,
                4.5,
                "4 Sacks of Cabbage",
            ),
            Self::Rosemary => PlantDetails::from("Rosemary", 11, 20.0, 12.0, 0.0, 66.5, ""),
            Self::Tomatoes => {
                PlantDetails::from("Tomatoes", 12, 40.0, 12.5, 0.0, 14.0, "2 Sacks of Cabbage")
            }
            Self::JutePlant => PlantDetails::from(
                "Jute Plant",
                13,
                50.0,
                13.0,
                0.0,
                14.5,
                "6 Handfuls of Barley Malt",
            ),
            Self::Marrentill => PlantDetails::from("Marrentill", 14, 80.0, 13.5, 0.0, 15.0, ""),
            Self::OakTree => PlantDetails::from(
                "Oak Tree",
                15,
                160.0,
                14.0,
                467.3,
                0.0,
                "1 Basket of Tomatoes",
            ),
            Self::YanillianHops => PlantDetails::from(
                "Yanillian Hops",
                16,
                60.0,
                14.5,
                0.0,
                16.0,
                "1 Basket of Tomatoes",
            ),
            Self::KeldaHops => PlantDetails::from("Kelda Hops", 17, 20.0, 9.0, 0.0, 10.0, ""),
            Self::Flax => PlantDetails::from("Flax", 18, 60.0, 16.0, 0.0, 17.5, "6 Grain"),
            Self::Tarromin => PlantDetails::from("Tarromin", 19, 80.0, 16.0, 0.0, 18.0, ""),
            Self::Sweetcorn => {
                PlantDetails::from("Sweetcorn", 20, 60.0, 17.0, 0.0, 19.0, "10 Jute Fibres")
            }
            Self::KrandorianHops => PlantDetails::from(
                "Krandorian Hops",
                21,
                70.0,
                17.5,
                0.0,
                19.5,
                "3 Sacks of Cabbage",
            ),
            Self::CadavaberryBush => PlantDetails::from(
                "Cadavaberry Bush",
                22,
                120.0,
                18.0,
                102.5,
                7.0,
                "3 Baskets of Tomatoes",
            ),
            Self::Seaweed => {
                PlantDetails::from("Seaweed", 23, 40.0, 19.0, 0.0, 21.0, "200 Numulites")
            }
            Self::Nasturtium => PlantDetails::from("Nasturtium", 24, 20.0, 19.5, 0.0, 111.0, ""),
            Self::Woad => PlantDetails::from("Woad", 25, 20.0, 20.5, 0.0, 115.5, ""),
            Self::YellowOrchids => PlantDetails::from("Yellow Orchids", 25, 0.0, 0.0, 0.0, 0.0, ""),
            Self::Harralander => PlantDetails::from("Harralander", 26, 80.0, 21.5, 0.0, 24.0, ""),
            Self::LimpwurtPlant => {
                PlantDetails::from("Limpwurt Plant", 26, 20.0, 21.5, 0.0, 120.0, "")
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
            Self::Golpar => PlantDetails::from("Golpar", 27, 0.5, 4.0, 0.0, 10.0, ""),
            Self::Elkhorn => PlantDetails::from(
                "Elkhorn Coral",
                28,
                160.0,
                20.5,
                0.0,
                24.0,
                "5 Giant seaweed",
            ),
            Self::WildbloodHops => {
                PlantDetails::from("Wildblood Hops", 28, 80.0, 23.0, 0.0, 26.0, "1 Nasturtium")
            }
            Self::Goutweed => PlantDetails::from("Goutweed", 29, 80.0, 105.0, 0.0, 45.0, ""),
            Self::WillowTree => PlantDetails::from(
                "Willow Tree",
                30,
                240.0,
                25.0,
                1456.5,
                0.0,
                "1 Basket of Apples",
            ),
            Self::Strawberries => PlantDetails::from(
                "Strawberries",
                31,
                60.0,
                26.0,
                0.0,
                29.0,
                "1 Basket of Apples",
            ),
            Self::RanarrWeed => PlantDetails::from("Ranarr Weed", 32, 80.0, 26.5, 0.0, 30.5, ""),
            Self::BananaTree => PlantDetails::from(
                "Banana Tree",
                33,
                960.0,
                28.0,
                1750.5,
                10.5,
                "4 Baskets of Apples",
            ),
            Self::Golovanova => PlantDetails::from("Golovanova", 34, 3.0, 0.0, 0.0, 6.0, ""),
            Self::TeakTree => PlantDetails::from(
                "Teak Tree",
                35,
                4480.0,
                35.0,
                7290.0,
                0.0,
                "15 Limpwurt roots",
            ),
            Self::DwellberryBush => PlantDetails::from(
                "Dwellberry Bush",
                36,
                140.0,
                31.5,
                177.5,
                12.0,
                "3 Baskets of Strawberries",
            ),
            Self::GrapeVine => PlantDetails::from("Grape Vine", 36, 35.0, 31.5, 625.0, 40.0, ""),
            Self::Hemp => PlantDetails::from("Hemp", 37, 80.0, 33.0, 0.0, 37.0, "6 Flax"),
            Self::Toadflax => PlantDetails::from("Toadflax", 38, 80.0, 34.0, 0.0, 38.5, ""),
            Self::Buchu => PlantDetails::from("Buchu", 39, 0.5, 6.0, 0.0, 15.0, ""),
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
            Self::IritLeaf => PlantDetails::from("Irit Leaf", 44, 80.0, 43.0, 0.0, 48.5, ""),
            Self::MapleTree => PlantDetails::from(
                "Maple Tree",
                45,
                320.0,
                45.0,
                3403.4,
                0.0,
                "1 Basket of Oranges",
            ),
            Self::Watermelon => {
                PlantDetails::from("Watermelon", 47, 80.0, 48.5, 0.0, 54.5, "10 Curry Leaves")
            }
            Self::JangerberryBush => PlantDetails::from(
                "Jangerberry Bush",
                48,
                160.0,
                50.5,
                284.5,
                19.0,
                "6 Watermelons",
            ),
            Self::Avantoe => PlantDetails::from("Avantoe", 50, 80.0, 54.5, 0.0, 61.5, ""),
            Self::PineappleTree => PlantDetails::from(
                "Pineapple Tree",
                51,
                960.0,
                57.0,
                4605.0,
                21.5,
                "10 Watermelons",
            ),
            Self::Pillar => PlantDetails::from(
                "Pillar Coral",
                52,
                160.0,
                52.0,
                0.0,
                60.0,
                "5 Elkhorn coral",
            ),
            Self::BittercapMushroom => {
                PlantDetails::from("Bittercap Mushroom", 53, 240.0, 61.5, 0.0, 57.7, "")
            }
            Self::Bologano => PlantDetails::from("Bologano", 54, 3.0, 0.0, 0.0, 14.0, ""),
            Self::Cactus => {
                PlantDetails::from("Cactus", 55, 560.0, 66.5, 374.0, 25.0, "6 Cadava berries")
            }
            Self::MahoganyTree => PlantDetails::from(
                "Mahogany Tree",
                55,
                5120.0,
                63.0,
                15720.0,
                0.0,
                "25 Yanillian hops",
            ),
            Self::Noxifer => PlantDetails::from("Noxifer", 55, 0.5, 12.0, 0.0, 30.0, ""),
            Self::Kwuarm => PlantDetails::from("Kwuarm", 56, 80.0, 69.0, 0.0, 78.0, ""),
            Self::PapayaTree => PlantDetails::from(
                "Papaya Tree",
                57,
                960.0,
                72.0,
                6146.6,
                27.0,
                "10 Pineapples",
            ),
            Self::WhiteLily => PlantDetails::from("White Lily", 58, 20.0, 42.0, 0.0, 250.0, ""),
            Self::WhiteberryBush => PlantDetails::from(
                "Whiteberry Bush",
                59,
                160.0,
                78.0,
                437.5,
                29.0,
                "8 Mushrooms",
            ),
            Self::YewTree => {
                PlantDetails::from("Yew Tree", 60, 365.0, 81.0, 7069.9, 0.0, "10 Cactus Spines")
            }
            Self::SnapeGrass => {
                PlantDetails::from("Snape Grass", 61, 70.0, 82.0, 0.0, 82.0, "5 Jangerberries")
            }
            Self::Snapdragon => PlantDetails::from("Snapdragon", 62, 80.0, 87.5, 0.0, 98.5, ""),
            Self::Belladonna => PlantDetails::from("Belladonna", 63, 320.0, 91.0, 0.0, 512.0, ""),
            Self::PotatoCactus => PlantDetails::from(
                "Potato Cactus",
                64,
                70.0,
                68.0,
                230.0,
                68.0,
                "8 Snape grass",
            ),
            Self::Hespori => PlantDetails::from("Hespori", 65, 1920.0, 62.0, 0.0, 12600.0, ""),
            Self::Huasca => PlantDetails::from("Huasca", 65, 80.0, 86.5, 0.0, 110.0, ""),
            Self::Camphor => PlantDetails::from(
                "Camphor Tree",
                66,
                5120.0,
                88.0,
                17840.0,
                0.0,
                "10 White berries",
            ),
            Self::Cadantine => PlantDetails::from("Cadantine", 67, 80.0, 106.5, 0.0, 120.0, ""),
            Self::PalmTree => {
                PlantDetails::from("Palm Tree", 68, 960.0, 110.5, 10150.1, 41.5, "15 Papayas")
            }
            Self::PoisonIvyBush => {
                PlantDetails::from("Poison Ivy Bush", 70, 160.0, 120.0, 675.0, 45.0, "")
            }
            Self::Cotton => PlantDetails::from("Cotton", 71, 100.0, 72.0, 0.0, 82.0, "6 Hemp"),
            Self::CalquatTree => PlantDetails::from(
                "Calquat Tree",
                72,
                1280.0,
                129.5,
                12096.0,
                48.5,
                "8 Poison ivy berries",
            ),
            Self::Lantadyme => PlantDetails::from("Lantadyme", 73, 80.0, 134.5, 0.0, 151.5, ""),
            Self::CrystalTree => {
                PlantDetails::from("Crystal Tree", 74, 480.0, 126.0, 13240.0, 0.0, "")
            }
            Self::Logavano => PlantDetails::from("Logavano", 74, 3.0, 0.0, 0.0, 23.0, ""),
            Self::MagicTree => {
                PlantDetails::from("Magic Tree", 75, 480.0, 145.5, 13768.3, 0.0, "25 Coconuts")
            }
            Self::Attas => PlantDetails::from("Attas Plant", 76, 5120.0, 100.0, 0.0, 0.0, ""),
            Self::Iasor => PlantDetails::from("Iasor Plant", 76, 5120.0, 100.0, 0.0, 0.0, ""),
            Self::Kronos => PlantDetails::from("Kronos Plant", 76, 5120.0, 100.0, 0.0, 0.0, ""),
            Self::Umbral => PlantDetails::from(
                "Umbral Coral",
                77,
                160.0,
                136.0,
                0.0,
                159.0,
                "5 Pillar seaweed",
            ),
            Self::DwarfWeed => PlantDetails::from("Dwarf Weed", 79, 80.0, 170.5, 0.0, 192.0, ""),
            Self::Ironwood => PlantDetails::from(
                "Ironwood Tree",
                80,
                5120.0,
                145.0,
                20380.0,
                0.0,
                "10 Curry leaves",
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
            Self::SpiritTree => PlantDetails::from(
                "Spirit Tree",
                83,
                3840.0,
                199.5,
                19301.0,
                0.0,
                "1 Ground Suqah tooth, 5 Monkey nuts, 1 Monkey bar",
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
            Self::Torstol => PlantDetails::from("Torstol", 85, 80.0, 199.5, 0.0, 224.5, ""),
            Self::RedwoodTree => PlantDetails::from(
                "Redwood Tree",
                90,
                6400.0,
                230.0,
                22450.0,
                0.0,
                "6 Dragonfruits",
            ),
            Self::Rosewood => PlantDetails::from(
                "Rosewood Tree",
                92,
                5760.0,
                252.0,
                23100.0,
                0.0,
                "8 Dragonfruit",
            ),
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
            s.c2(&growth_time(self.time)),
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

/// Growth time carries its unit, since a bare number reads as anything. Under
/// a minute is shown in seconds - the Chambers of Xeric herbs grow in 30 - and
/// a crop the wiki lists no time for says so rather than claiming zero.
fn growth_time(minutes: f64) -> String {
    if minutes <= 0.0 {
        return "N/A".to_string();
    }

    if minutes < 1.0 {
        return format!("{} sec", round(minutes * 60.0));
    }

    format!("{} min", round(minutes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find(name: &str) -> PlantDetails {
        Plant::all()
            .into_iter()
            .map(|plant| plant.details())
            .find(|details| details.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("{} is not in the table", name))
    }

    #[test]
    fn every_plant_is_named_distinct_and_levelled() {
        let names: Vec<String> = Plant::all()
            .into_iter()
            .map(|plant| plant.details().name)
            .collect();

        for plant in Plant::all() {
            let details = plant.details();
            assert!(!details.name.is_empty(), "a plant has no name");
            assert!(details.level >= 1, "{} has no farming level", details.name);
        }

        let mut unique = names.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(names.len(), unique.len(), "duplicate plant in the table");
    }

    #[test]
    fn the_table_is_ordered_by_farming_level() {
        let levels: Vec<u32> = Plant::all()
            .into_iter()
            .map(|plant| plant.details().level)
            .collect();

        for pair in levels.windows(2) {
            assert!(
                pair[0] <= pair[1],
                "table is out of level order: {:?}",
                pair
            );
        }
    }

    /// The crops that prompted this table being regenerated from the wiki.
    #[test]
    fn the_crops_the_table_used_to_be_missing_are_present() {
        assert_eq!(find("Seaweed").level, 23);
        assert_eq!(find("Grape Vine").level, 36);
        assert_eq!(find("Huasca").level, 65);
    }

    #[test]
    fn seaweed_carries_its_numulite_payment() {
        let seaweed = find("Seaweed");

        assert_eq!(seaweed.payment, "200 Numulites");
        assert_eq!(seaweed.planting_xp, 19.0);
        assert_eq!(seaweed.harvesting_xp, 21.0);
    }

    #[test]
    fn acorn_resolves_to_the_oak_rather_than_duplicating_it() {
        // Acorn is the oak's seed, so it is an alias, not its own row.
        assert_eq!(resolve("acorn"), "oak");
        assert_eq!(resolve("ACORN"), "oak");
        assert!(
            Plant::all()
                .into_iter()
                .all(|plant| plant.details().name != "Acorn"),
            "Acorn should not be a row of its own"
        );
    }

    #[test]
    fn a_query_that_is_not_an_alias_is_passed_through_lowercased() {
        assert_eq!(resolve("  Ranarr Weed "), "ranarr weed");
    }

    #[test]
    fn growth_time_carries_a_unit() {
        assert_eq!(growth_time(80.0), "80 min");
        assert_eq!(growth_time(17.5), "17.5 min");
    }

    #[test]
    fn a_crop_growing_in_under_a_minute_is_shown_in_seconds() {
        // The Chambers of Xeric herbs grow in 30 seconds.
        assert_eq!(growth_time(0.5), "30 sec");
    }

    #[test]
    fn a_crop_with_no_published_time_says_so() {
        // Yellow orchids have a Farming level and nothing else on the wiki.
        assert_eq!(growth_time(0.0), "N/A");
        assert_eq!(find("Yellow Orchids").level, 25);
    }

    /// Values the wiki publishes that the hand-maintained table had wrong, so a
    /// regeneration that silently reverts them fails here.
    #[test]
    fn corrected_values_match_the_wiki() {
        assert_eq!(find("Oak Tree").time, 160.0);
        assert_eq!(find("Potato").harvesting_xp, 9.0);
        assert_eq!(find("Cotton").harvesting_xp, 82.0);
        assert_eq!(find("Ironwood Tree").planting_xp, 145.0);
        assert_eq!(find("Pillar Coral").planting_xp, 52.0);
        assert_eq!(find("Guam").time, 80.0);
    }
}
