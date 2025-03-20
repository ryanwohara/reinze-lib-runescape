use crate::common::{skill_by_id, skill_id, skills};
use crate::stats::agility::{Agility, AgilityDetails, AgilityMultipliers};

pub trait Skill {
    fn defaults() -> Vec<Details>;
    fn details(&self) -> Details;
}

#[derive(Clone, PartialOrd, PartialEq)]
pub enum Details {
    Agility(AgilityDetails),
}

impl Details {
    // pub fn skill(&self) -> String {
    //     match self {
    //         Details::Agility(agility) => "Agility",
    //     }.to_string()
    // }

    pub fn name(&self) -> String {
        if let Some(result) = skills().get(self.skill_id() as usize) {
            return result.clone();
        };

        "".to_string()
    }

    pub fn skill_id(&self) -> u32 {
        skill_id(self.skill())
    }

    pub fn skill(&self) -> String {
        match self {
            Details::Agility(_) => "Agility",
        }
        .to_string()
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
}

pub fn details_by_skill_id(id: u32) -> Vec<Details> {
    match skill_by_id(id).as_str() {
        "Agility" => Agility::defaults(),
        _ => return vec![],
    }
}
