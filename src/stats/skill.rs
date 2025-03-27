use crate::common::{skill_by_id, skill_id, skills};
use crate::stats::agility::{Agility, AgilityDetails, AgilityMultipliers};
use crate::stats::construction::{Construction, ConstructionDetails, ConstructionMultipliers};
use crate::stats::cooking::{Cooking, CookingDetails};
use crate::stats::crafting::{Crafting, CraftingDetails};
use crate::stats::farming::{Farming, FarmingDetails, FarmingMultipliers};
use crate::stats::firemaking::{Firemaking, FiremakingDetails, FiremakingMultipliers};
use crate::stats::fishing::{Fishing, FishingDetails, FishingMultipliers};
use crate::stats::fletching::{Fletching, FletchingDetails};
use crate::stats::herblore::{Herblore, HerbloreDetails};
use crate::stats::hunter::{Hunter, HunterDetails};

pub trait Skill {
    fn all() -> Vec<Self>
    where
        Self: Sized;
    fn defaults() -> Vec<Details>;
    fn details(&self) -> Details;
    fn search<T>(query: T) -> Vec<Self>
    where
        T: ToString,
        Self: Sized;
}

#[derive(Clone, PartialOrd, PartialEq)]
pub enum Details {
    Agility(AgilityDetails),
    Construction(ConstructionDetails),
    Cooking(CookingDetails),
    Crafting(CraftingDetails),
    Farming(FarmingDetails),
    Firemaking(FiremakingDetails),
    Fishing(FishingDetails),
    Fletching(FletchingDetails),
    Herblore(HerbloreDetails),
    Hunter(HunterDetails),
}

pub trait IntoString {
    fn to_string(&self, xp_difference: f64) -> String;
}

impl Details {
    pub fn to_string(&self, xp_difference: f64) -> String {
        match self {
            Details::Agility(agility) => agility.to_string(xp_difference),
            Details::Construction(construction) => construction.to_string(xp_difference),
            Details::Cooking(cooking) => cooking.to_string(xp_difference),
            Details::Crafting(crafting) => crafting.to_string(xp_difference),
            Details::Farming(farming) => farming.to_string(xp_difference),
            Details::Firemaking(firemaking) => firemaking.to_string(xp_difference),
            Details::Fishing(fishing) => fishing.to_string(xp_difference),
            Details::Fletching(fletching) => fletching.to_string(xp_difference),
            Details::Herblore(herblore) => herblore.to_string(xp_difference),
            Details::Hunter(hunter) => hunter.to_string(xp_difference),
        }
    }

    pub fn name(&self) -> String {
        if let Some(result) = skills().get(self.skill_id() as usize) {
            return result.clone();
        };

        "".to_owned()
    }

    pub fn skill_id(&self) -> u32 {
        skill_id(self.skill())
    }

    pub fn skill(&self) -> String {
        match self {
            Details::Agility(_) => "Agility",
            Details::Construction(_) => "Construction",
            Details::Cooking(_) => "Cooking",
            Details::Crafting(_) => "Crafting",
            Details::Farming(_) => "Farming",
            Details::Firemaking(_) => "Firemaking",
            Details::Fishing(_) => "Fishing",
            Details::Fletching(_) => "Fletching",
            Details::Herblore(_) => "Herblore",
            Details::Hunter(_) => "Hunter",
        }
        .to_owned()
    }

    pub fn multipliers<T>(details: T) -> Vec<Multipliers>
    where
        T: Detail,
    {
        details.multipliers()
    }
}

pub trait Detail {
    fn multipliers(&self) -> Vec<Multipliers>;
    fn name(&self) -> String;
    fn level(&self) -> u32;
    fn xp(&self) -> f64;
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Multipliers {
    Agility(AgilityMultipliers),
    Construction(ConstructionMultipliers),
    Farming(FarmingMultipliers),
    Firemaking(FiremakingMultipliers),
    Fishing(FishingMultipliers),
}

pub fn details_by_skill_id(id: u32, query: &str) -> Vec<Details> {
    if query.len() == 0 {
        return match skill_by_id(id).as_str() {
            "Agility" => Agility::defaults(),
            "Construction" => Construction::defaults(),
            "Cooking" => Cooking::defaults(),
            "Crafting" => Crafting::defaults(),
            "Farming" => Farming::defaults(),
            "Firemaking" => Firemaking::defaults(),
            "Fishing" => Fishing::defaults(),
            "Fletching" => Fletching::defaults(),
            "Herblore" => Herblore::defaults(),
            "Hunter" => Hunter::defaults(),
            _ => vec![],
        };
    }

    match skill_by_id(id).as_str() {
        "Agility" => Agility::search(query).iter().map(|x| x.details()).collect(),
        "Construction" => Construction::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Cooking" => Cooking::search(query).iter().map(|x| x.details()).collect(),
        "Crafting" => Crafting::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Farming" => Farming::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Firemaking" => Firemaking::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Fishing" => Fishing::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Fletching" => Fletching::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Herblore" => Herblore::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Hunter" => Hunter::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        _ => vec![],
    }
}
