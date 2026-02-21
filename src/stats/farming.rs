use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use regex::Regex;
use std::ops::Add;

pub enum Farming {
    Potatoes,
    WinterSqirk,
    DockLeaf,
    FernBigPlant,
    Plant,
    ShortPlant,
    SpringSqirk,
    AutumnSqirk,
    SummerSqirk,
    Bush,
    LargeLeafBush,
    SmallFern,
    Thistle,
    FernSmallPlant,
    HugePlant,
    Reeds,
    TallPlant,
    Onions,
    Cabbages,
    GuamLeaf,
    Tomatoes,
    Marrentill,
    OakTree,
    Tarromin,
    Sweetcorn,
    GiantSeaweed,
    Harralander,
    LimpwurtPlant,
    AppleTree,
    Goutweed,
    WillowTree,
    Strawberries,
    RanarrWeed,
    BananaTree,
    TeakTree,
    Toadflax,
    OrangeTree,
    CurryTree,
    IritLeaf,
    MapleTree,
    Watermelons,
    Avantoe,
    PineapplePlant,
    MahoganyTree,
    Kwuarm,
    PapayaTree,
    WhiteLily,
    YewTree,
    SnapeGrass,
    Snapdragon,
    Huasca,
    Hespori,
    Cadantine,
    PalmTree,
    CalquatTree,
    Lantadyme,
    CrystalTree,
    MagicTree,
    DwarfWeed,
    DragonfruitTree,
    SpiritTree,
    Torstol,
    CelastrusTree,
    RedwoodTree,
    CamphorTree,
    IronwoodTree,
    RosewoodTree,
}

impl Detail for Farming {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Farming(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Farming(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Farming(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Farming {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Potatoes,
            Self::WinterSqirk,
            Self::DockLeaf,
            Self::FernBigPlant,
            Self::Plant,
            Self::ShortPlant,
            Self::SpringSqirk,
            Self::AutumnSqirk,
            Self::SummerSqirk,
            Self::Bush,
            Self::LargeLeafBush,
            Self::SmallFern,
            Self::Thistle,
            Self::FernSmallPlant,
            Self::HugePlant,
            Self::Reeds,
            Self::TallPlant,
            Self::Onions,
            Self::Cabbages,
            Self::GuamLeaf,
            Self::Tomatoes,
            Self::Marrentill,
            Self::OakTree,
            Self::Tarromin,
            Self::Sweetcorn,
            Self::GiantSeaweed,
            Self::Harralander,
            Self::LimpwurtPlant,
            Self::AppleTree,
            Self::Goutweed,
            Self::WillowTree,
            Self::Strawberries,
            Self::RanarrWeed,
            Self::BananaTree,
            Self::TeakTree,
            Self::Toadflax,
            Self::OrangeTree,
            Self::CurryTree,
            Self::IritLeaf,
            Self::MapleTree,
            Self::Watermelons,
            Self::Avantoe,
            Self::PineapplePlant,
            Self::MahoganyTree,
            Self::Kwuarm,
            Self::PapayaTree,
            Self::WhiteLily,
            Self::YewTree,
            Self::SnapeGrass,
            Self::Snapdragon,
            Self::Huasca,
            Self::Hespori,
            Self::Cadantine,
            Self::PalmTree,
            Self::CalquatTree,
            Self::Lantadyme,
            Self::CrystalTree,
            Self::MagicTree,
            Self::DwarfWeed,
            Self::DragonfruitTree,
            Self::SpiritTree,
            Self::Torstol,
            Self::CelastrusTree,
            Self::RedwoodTree,
            Self::CamphorTree,
            Self::IronwoodTree,
            Self::RosewoodTree,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::Watermelons,
            Self::SnapeGrass,
            Self::CalquatTree,
            Self::PapayaTree,
            Self::PalmTree,
            Self::DragonfruitTree,
            Self::TeakTree,
            Self::MahoganyTree,
            Self::YewTree,
            Self::MagicTree,
            Self::CrystalTree,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::Potatoes => ("Potatoes", 1, 8.0),
            Self::WinterSqirk => ("Winter Sq'irk", 1, 30.0),
            Self::DockLeaf => ("Dock Leaf", 1, 31.0),
            Self::FernBigPlant => ("Fern (big plant)", 1, 31.0),
            Self::Plant => ("Plant", 1, 31.0),
            Self::ShortPlant => ("Short Plant", 1, 31.0),
            Self::SpringSqirk => ("Spring Sq'irk", 1, 40.0),
            Self::AutumnSqirk => ("Autumn Sq'irk", 1, 50.0),
            Self::SummerSqirk => ("Summer Sq'irk", 1, 60.0),
            Self::Bush => ("Bush", 1, 70.0),
            Self::LargeLeafBush => ("Large Leaf Bush", 1, 70.0),
            Self::SmallFern => ("Small Fern", 1, 70.0),
            Self::Thistle => ("Thistle", 1, 70.0),
            Self::FernSmallPlant => ("Fern (small plant)", 1, 100.0),
            Self::HugePlant => ("Huge Plant", 1, 100.0),
            Self::Reeds => ("Reeds", 1, 100.0),
            Self::TallPlant => ("Tall Plant", 1, 100.0),
            Self::Onions => ("Onions", 5, 10.0),
            Self::Cabbages => ("Cabbages", 7, 10.0),
            Self::GuamLeaf => ("Guam Leaf", 9, 11.0),
            Self::Tomatoes => ("Tomatoes", 12, 12.5),
            Self::Marrentill => ("Marrentill", 14, 13.5),
            Self::OakTree => ("Oak Tree", 15, 481.3),
            Self::Tarromin => ("Tarromin", 19, 16.0),
            Self::Sweetcorn => ("Sweetcorn", 20, 17.0),
            Self::GiantSeaweed => ("Giant seaweed", 23, 21.0),
            Self::Harralander => ("Harralander", 26, 21.5),
            Self::LimpwurtPlant => ("Limpwurt Plant", 26, 40.0),
            Self::AppleTree => ("Apple Tree", 27, 1221.5),
            Self::Goutweed => ("Goutweed", 29, 105.0),
            Self::WillowTree => ("Willow Tree", 30, 1481.5),
            Self::Strawberries => ("Strawberries", 31, 26.0),
            Self::RanarrWeed => ("Ranarr Weed", 32, 27.0),
            Self::BananaTree => ("Banana Tree", 33, 1778.5),
            Self::TeakTree => ("Teak Tree", 35, 7315.0),
            Self::Toadflax => ("Toadflax", 38, 34.0),
            Self::OrangeTree => ("Orange Tree", 39, 2505.7),
            Self::CurryTree => ("Curry Tree", 42, 2946.9),
            Self::IritLeaf => ("Irit Leaf", 44, 43.0),
            Self::MapleTree => ("Maple Tree", 45, 3448.4),
            Self::Watermelons => ("Watermelons", 47, 49.0),
            Self::Avantoe => ("Avantoe", 50, 54.5),
            Self::PineapplePlant => ("Pineapple Plant", 51, 4662.7),
            Self::MahoganyTree => ("Mahogany Tree", 55, 15783.0),
            Self::Kwuarm => ("Kwuarm", 56, 69.0),
            Self::PapayaTree => ("Papaya Tree", 57, 6218.4),
            Self::WhiteLily => ("White lily", 58, 292.0),
            Self::YewTree => ("Yew Tree", 60, 7150.9),
            Self::SnapeGrass => ("Snape grass", 61, 82.0),
            Self::Snapdragon => ("Snapdragon", 62, 87.5),
            Self::Huasca => ("Huasca", 65, 86.5),
            Self::Hespori => ("Hespori", 65, 12662.0),
            Self::CamphorTree => ("Camphor Tree", 66, 0.0),
            Self::Cadantine => ("Cadantine", 67, 106.5),
            Self::PalmTree => ("Palm Tree", 68, 10260.6),
            Self::CalquatTree => ("Calquat Tree", 72, 12225.5),
            Self::Lantadyme => ("Lantadyme", 73, 134.5),
            Self::CrystalTree => ("Crystal Tree", 74, 13366.0),
            Self::MagicTree => ("Magic Tree", 75, 13913.8),
            Self::DwarfWeed => ("Dwarf Weed", 79, 170.5),
            Self::IronwoodTree => ("Ironwood Tree", 80, 0.0),
            Self::DragonfruitTree => ("Dragonfruit Tree", 81, 17895.0),
            Self::SpiritTree => ("Spirit Tree", 83, 19501.3),
            Self::Torstol => ("Torstol", 85, 199.5),
            Self::CelastrusTree => ("Celastrus Tree", 85, 14334.0),
            Self::RedwoodTree => ("Redwood Tree", 90, 22680.0),
            Self::RosewoodTree => ("Rosewood Tree", 92, 0.0),
        };

        Details::Farming(FarmingDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: vec![Multipliers::Farming(FarmingMultipliers::FarmersOutfit)],
        })
    }

    fn search<T>(query: T) -> Vec<Self>
    where
        T: ToString,
        Self: Sized,
    {
        let mut all = Self::all();

        let q = query.to_string().to_lowercase();

        if let Ok(pattern) = Regex::new(q.as_str()) {
            let mut index = 0;
            all.retain(|activity| {
                if pattern
                    .captures(activity.name().to_lowercase().as_str())
                    .iter()
                    .count()
                    > 0
                    && index < 10
                {
                    index = index.add(1);

                    return true;
                }

                return false;
            });
        } else {
            return vec![];
        }

        all
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct FarmingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for FarmingDetails {
    fn to_string(&self, s: &crate::stats::skill::Source, xp_difference: f64) -> String {
        let mut vec = vec![format!(
            "{}: {}",
            s.c1(self.name.as_str()),
            s.c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp as f64).ceil()).as_str(),
                "d"
            )
            .as_str())
        )];

        self.multipliers.iter().for_each(|x| {
            let a = match x {
                Multipliers::Farming(x) => x,
                _ => return,
            };
            let d = a.details();
            vec.push(s.p(format!(
                "{} {}",
                s.c1(format!("{}:", d.name.as_str()).as_str()),
                s.c2(common::commas_from_string(
                    format!("{}", (xp_difference / (self.xp as f64 * d.value)).ceil()).as_str(),
                    "d"
                )
                .as_str())
            )
            .as_str()));
        });

        vec.join(" ")
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum FarmingMultipliers {
    FarmersOutfit,
}

impl FarmingMultipliers {
    pub fn details(&self) -> FarmingMultiplierDetails {
        let details = match self {
            Self::FarmersOutfit => ("Outfit", 1.025),
        };

        FarmingMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct FarmingMultiplierDetails {
    pub name: String,
    pub value: f64,
}
