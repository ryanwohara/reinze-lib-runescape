use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use common::{c1, c2};
use regex::Regex;
use std::ops::Add;

pub enum Sailing {
    SmallShipwreck,
    FishermansShipwreck,
    BarracudaShipwreck,
    LargeShipwreck,
    PirateShipwreck,
    MercenaryShipwreck,
    FremennikShipwreck,
    MerchantShipwreck,
    TemporUnranked,
    TemporSwordfish,
    TemporShark,
    TemporMerlin,
    JubblyUnranked,
    JubblySwordfish,
    JubblyShark,
    JubblyMerlin,
    GwenithUnranked,
    GwenithSwordfish,
    GwenithShark,
    GwenithMerlin,
}

impl Skill for Sailing {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::SmallShipwreck,
            Self::FishermansShipwreck,
            Self::BarracudaShipwreck,
            Self::LargeShipwreck,
            Self::PirateShipwreck,
            Self::MercenaryShipwreck,
            Self::FremennikShipwreck,
            Self::MerchantShipwreck,
            Self::TemporUnranked,
            Self::TemporSwordfish,
            Self::TemporShark,
            Self::TemporMerlin,
            Self::JubblyUnranked,
            Self::JubblySwordfish,
            Self::JubblyShark,
            Self::JubblyMerlin,
            Self::GwenithUnranked,
            Self::GwenithSwordfish,
            Self::GwenithShark,
            Self::GwenithMerlin,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::FishermansShipwreck,
            Self::BarracudaShipwreck,
            Self::LargeShipwreck,
            Self::PirateShipwreck,
            Self::MercenaryShipwreck,
            Self::FremennikShipwreck,
            Self::MerchantShipwreck,
            Self::TemporMerlin,
            Self::JubblyMerlin,
            Self::GwenithMerlin,
        ]
            .iter()
            .map(|x| x.details())
            .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::SmallShipwreck => ("Small Shipwreck", 15, 5.5),
            Self::FishermansShipwreck => ("Fishermans Shipwreck", 26, 9.0),
            Self::BarracudaShipwreck => ("Barracuda Shipwreck", 35, 15.5),
            Self::LargeShipwreck => ("Large Shipwreck", 53, 24.0),
            Self::PirateShipwreck => ("Pirate Shipwreck", 64, 31.5),
            Self::MercenaryShipwreck => ("Mercenary Shipwreck", 73, 63.5),
            Self::FremennikShipwreck => ("Fremennik Shipwreck", 80, 75.0),
            Self::MerchantShipwreck => ("Merchant Shipwreck", 87, 95.0),
            Self::TemporUnranked => ("Tempor Unranked", 30, 300.0),
            Self::TemporSwordfish => ("Tempor Swordfish", 30, 595.0),
            Self::TemporShark => ("Tempor Shark", 30, 1025.0),
            Self::TemporMerlin => ("Tempor Merlin", 30, 1790.0),
            Self::JubblyUnranked => ("Jubbly Unranked", 55, 1592.0),
            Self::JubblySwordfish => ("Jubbly Swordfish", 55, 2392.0),
            Self::JubblyShark => ("Jubbly Shark", 55, 4270.0),
            Self::JubblyMerlin => ("Jubbly Merlin", 55, 8204.0),
            Self::GwenithUnranked => ("Gwenith Unranked", 72, 3100.0),
            Self::GwenithSwordfish => ("Gwenith Swordfish", 72, 4100.0),
            Self::GwenithShark => ("Gwenith Shark", 72, 9315.0),
            Self::GwenithMerlin => ("Gwenith Merlin", 72, 19410.0),
        };

        Details::Sailing(SailingDetails {
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

impl Detail for Sailing {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Sailing(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Sailing(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Sailing(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct SailingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
}

impl IntoString for SailingDetails {
    fn to_string(&self, xp_difference: f64) -> String {
        format!(
            "{}: {}",
            c1(self.name.as_str()),
            c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp).ceil()).as_str(),
                "d"
            )
                .as_str())
        )
    }
}
