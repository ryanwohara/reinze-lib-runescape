use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use common::{c1, c2, p};
use regex::Regex;
use std::ops::Add;
use crate::stats::mining::MiningMultipliers::ProspectorsKit;

pub enum Mining {
    Clay,
    RuneEssence,
    CopperOre,
    TinOre,
    Limestone,
    BarroniteShards,
    BarroniteDeposit,
    IronOre,
    SilverOre,
    PureEssence,
    Coal,
    Sandstone1kg,
    Sandstone2kg,
    Sandstone5kg,
    Sandstone10kg,
    DenseEssenceBlock,
    GemRocks,
    GoldOre,
    CalcifiedRocks,
    Granite500g,
    Granite2kg,
    Granite5kg,
    MithrilOre,
    SoftClay,
    AdamantiteOre,
    RuniteOre,
    Amethyst,
}

impl Skill for Mining {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Clay,
            Self::RuneEssence,
            Self::CopperOre,
            Self::TinOre,
            Self::Limestone,
            Self::BarroniteShards,
            Self::BarroniteDeposit,
            Self::IronOre,
            Self::SilverOre,
            Self::PureEssence,
            Self::Coal,
            Self::Sandstone1kg,
            Self::Sandstone2kg,
            Self::Sandstone5kg,
            Self::Sandstone10kg,
            Self::DenseEssenceBlock,
            Self::GemRocks,
            Self::GoldOre,
            Self::CalcifiedRocks,
            Self::Granite500g,
            Self::Granite2kg,
            Self::Granite5kg,
            Self::MithrilOre,
            Self::SoftClay,
            Self::AdamantiteOre,
            Self::RuniteOre,
            Self::Amethyst,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![Self::IronOre, Self::Coal, Self::GemRocks, Self::GoldOre, Self::MithrilOre, Self::AdamantiteOre, Self::RuniteOre, Self::Amethyst]
            .iter()
            .map(|x| x.details())
            .collect()
    }

    fn details(&self) -> Details {
        let details = match (self) {
            Self::Clay => ("Clay", 1, 5.0),
            Self::RuneEssence => ("Rune Essence", 1, 5.0),
            Self::CopperOre => ("Copper", 1, 17.5),
            Self::TinOre => ("Tin", 1, 17.5),
            Self::Limestone => ("Limestone", 10, 26.5),
            Self::BarroniteShards => ("Barronite Shards", 14, 16.0),
            Self::BarroniteDeposit => ("Barronite Deposit", 14, 32.0),
            Self::IronOre => ("Iron", 15, 35.0),
            Self::SilverOre => ("Silver", 20, 40.0),
            Self::PureEssence => ("Pure Essence", 30, 5.0),
            Self::Coal => ("Coal", 30, 50.0),
            Self::Sandstone1kg => ("Sandstone 1kg", 35, 30.0),
            Self::Sandstone2kg => ("Sandstone 2kg", 35, 40.0),
            Self::Sandstone5kg => ("Sandstone 5kg", 35, 50.0),
            Self::Sandstone10kg => ("Sandstone 10kg", 35, 60.0),
            Self::DenseEssenceBlock => ("Dense Essence Block", 38, 12.0),
            Self::GemRocks => ("Gem Rocks", 40, 65.0),
            Self::GoldOre => ("Gold", 40, 65.0),
            Self::CalcifiedRocks => ("Calcified Rocks", 41, 33.0),
            Self::Granite500g => ("Granite 500g", 45, 50.0),
            Self::Granite2kg => ("Granite 2kg", 45, 60.0),
            Self::Granite5kg => ("Granite 5kg", 45, 75.0),
            Self::MithrilOre => ("Mithril", 55, 80.0),
            Self::SoftClay => ("Soft Clay", 70, 5.0),
            Self::AdamantiteOre => ("Adamantite", 70, 95.0),
            Self::RuniteOre => ("Runite", 85, 125.0),
            Self::Amethyst => ("Amethyst", 92, 240.0),
        };

        Details::Mining(MiningDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
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

impl Detail for Mining {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Mining(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Mining(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Mining(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct MiningDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
}

impl IntoString for MiningDetails {
    fn to_string(&self, xp_difference: f64) -> String {
        let mut vec = vec![format!(
            "{}: {}",
            c1(self.name.as_str()),
            c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp as f64).ceil()).as_str(),
                "d"
            )
                .as_str())
        )];

        let a = ProspectorsKit;
        let d = a.details();
        vec.push(p(format!(
            "{} {}",
            c1(format!("{}:", d.name.as_str()).as_str()),
            c2(common::commas_from_string(
                format!("{}", (xp_difference / (self.xp as f64 * d.value)).ceil()).as_str(),
                "d"
            )
                .as_str())
        )
            .as_str()));

        vec.join(" ")
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum MiningMultipliers {
    ProspectorsKit,
}

impl MiningMultipliers {
    pub fn details(&self) -> MiningMultiplierDetails {
        let details = match self {
            Self::ProspectorsKit => ("Outfit", 1.025),
        };

        MiningMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct MiningMultiplierDetails {
    pub name: String,
    pub value: f64,
}
