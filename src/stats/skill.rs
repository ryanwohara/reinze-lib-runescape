use crate::common::{skill_by_id, skill_id, skills};
use crate::stats::agility::{Agility, AgilityDetails, AgilityMultipliers};
use crate::stats::construction::ConstructionMultipliers;

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
}

pub trait IntoString {
    fn to_string(&self, xp_difference: f64) -> String;
}

impl Details {
    pub fn to_string(&self, xp_difference: f64) -> String {
        match self {
            Details::Agility(agility) => agility,
            Details::Construction(construction) => construction,
        }
        .to_string(xp_difference)
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
}

pub fn details_by_skill_id(id: u32, query: &str) -> Vec<Details> {
    if query.len() == 0 {
        return match skill_by_id(id).as_str() {
            "Agility" => Agility::defaults(),
            _ => vec![],
        };
    }

    let mut matches = match skill_by_id(id).as_str() {
        "Agility" => Agility::search(query),
        _ => vec![],
    };

    matches.iter().map(|x| x.details()).collect()
}
