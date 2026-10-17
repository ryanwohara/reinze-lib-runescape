use crate::common::{skill_by_id, skill_id, skills};
use crate::npc::data::{Npc, NpcMetadata};
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
use crate::stats::magic::{Magic, MagicDetails};
use crate::stats::mining::{Mining, MiningDetails, MiningMultipliers};
use crate::stats::prayer::{Prayer, PrayerDetails, PrayerMultipliers};
use crate::stats::runecraft::{Runecraft, RunecraftDetails, RunecraftMultipliers};
use crate::stats::sailing::{Sailing, SailingDetails};
use crate::stats::smithing::{Smithing, SmithingDetails, SmithingMultipliers};
use crate::stats::thieving::{Thieving, ThievingDetails};
use crate::stats::woodcutting::{Woodcutting, WoodcuttingDetails, WoodcuttingMultipliers};
pub use common::source::Source;

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
#[allow(dead_code)]
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
    Magic(MagicDetails),
    Mining(MiningDetails),
    Prayer(PrayerDetails),
    Runecraft(RunecraftDetails),
    Sailing(SailingDetails),
    Smithing(SmithingDetails),
    Thieving(ThievingDetails),
    Woodcutting(WoodcuttingDetails),
    Npc(NpcMetadata),
    Hitpoints(NpcMetadata),
    Slayer(NpcMetadata),
}

pub trait IntoString {
    fn to_string(&self, s: &crate::stats::skill::Source, xp_difference: f64) -> String;
}

impl Details {
    pub fn to_string(&self, s: &Source, xp_difference: f64) -> String {
        match self {
            Details::Agility(agility) => agility.to_string(s, xp_difference),
            Details::Construction(construction) => construction.to_string(s, xp_difference),
            Details::Cooking(cooking) => cooking.to_string(s, xp_difference),
            Details::Crafting(crafting) => crafting.to_string(s, xp_difference),
            Details::Farming(farming) => farming.to_string(s, xp_difference),
            Details::Firemaking(firemaking) => firemaking.to_string(s, xp_difference),
            Details::Fishing(fishing) => fishing.to_string(s, xp_difference),
            Details::Fletching(fletching) => fletching.to_string(s, xp_difference),
            Details::Herblore(herblore) => herblore.to_string(s, xp_difference),
            Details::Hunter(hunter) => hunter.to_string(s, xp_difference),
            Details::Magic(magic) => magic.to_string(s, xp_difference),
            Details::Mining(mining) => mining.to_string(s, xp_difference),
            Details::Prayer(prayer) => prayer.to_string(s, xp_difference),
            Details::Runecraft(runecraft) => runecraft.to_string(s, xp_difference),
            Details::Sailing(sailing) => sailing.to_string(s, xp_difference),
            Details::Smithing(smithing) => smithing.to_string(s, xp_difference),
            Details::Thieving(thieving) => thieving.to_string(s, xp_difference),
            Details::Woodcutting(woodcutting) => woodcutting.to_string(s, xp_difference),
            Details::Npc(npc) => IntoString::to_string(npc, s, xp_difference),
            Details::Hitpoints(npc) => npc.hp(s, xp_difference),
            Details::Slayer(npc) => npc.slay(s, xp_difference),
        }
    }

    /// The level needed to perform the action. Combat and Hitpoints have no
    /// requirement to fight a monster; Slayer has the task's Slayer level.
    pub fn level(&self) -> u32 {
        match self {
            Details::Agility(x) => x.level,
            Details::Construction(x) => x.level,
            Details::Cooking(x) => x.level,
            Details::Crafting(x) => x.level,
            Details::Farming(x) => x.level,
            Details::Firemaking(x) => x.level,
            Details::Fishing(x) => x.level,
            Details::Fletching(x) => x.level,
            Details::Herblore(x) => x.level,
            Details::Hunter(x) => x.level,
            Details::Magic(x) => x.level,
            Details::Mining(x) => x.level,
            Details::Prayer(x) => x.level,
            Details::Runecraft(x) => x.level,
            Details::Sailing(x) => x.level,
            Details::Smithing(x) => x.level,
            Details::Thieving(x) => x.level,
            Details::Woodcutting(x) => x.level,
            Details::Npc(_) | Details::Hitpoints(_) => 1,
            Details::Slayer(npc) => npc.slayer_req,
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> String {
        if let Some(result) = skills().get(self.skill_id() as usize) {
            return result.clone();
        };

        "".to_owned()
    }

    #[allow(dead_code)]
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
            Details::Magic(_) => "Magic",
            Details::Mining(_) => "Mining",
            Details::Prayer(_) => "Prayer",
            Details::Runecraft(_) => "Runecraft",
            Details::Sailing(_) => "Sailing",
            Details::Smithing(_) => "Smithing",
            Details::Thieving(_) => "Thieving",
            Details::Woodcutting(_) => "Woodcutting",
            Details::Npc(_) => "NPC",
            Details::Hitpoints(_) => "Hitpoints",
            Details::Slayer(_) => "Slayer",
        }
        .to_string()
    }

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn level(&self) -> u32;
    #[allow(dead_code)]
    fn xp(&self) -> f64;
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Multipliers {
    Agility(AgilityMultipliers),
    Construction(ConstructionMultipliers),
    Farming(FarmingMultipliers),
    Firemaking(FiremakingMultipliers),
    Fishing(FishingMultipliers),
    #[allow(dead_code)]
    Mining(MiningMultipliers),
    Prayer(PrayerMultipliers),
    Runecraft(RunecraftMultipliers),
    Smithing(SmithingMultipliers),
    Woodcutting(WoodcuttingMultipliers),
}

/// With no query, the skill's default actions are trimmed to the ones `level`
/// can already do. An explicit query is shown as-is.
pub fn details_by_skill_id(id: u32, query: &str, level: u32) -> Vec<Details> {
    if query.len() == 0 {
        let mut defaults = match skill_by_id(id).as_str() {
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
            "Magic" => Magic::defaults(),
            "Mining" => Mining::defaults(),
            "Prayer" => Prayer::defaults(),
            "Runecraft" => Runecraft::defaults(),
            "Sailing" => Sailing::defaults(),
            "Smithing" => Smithing::defaults(),
            "Thieving" => Thieving::defaults(),
            "Woodcutting" => Woodcutting::defaults(),
            // Combat skills are (ultimately) stored in
            // data.rs for easy updates with cmd-xp.py.
            "Attack" | "Strength" | "Defence" | "Ranged" => Npc::defaults(),
            "Hitpoints" => Npc::hitpoints(),
            "Slayer" => Npc::slayer(),
            _ => vec![],
        };
        defaults.retain(|detail| detail.level() <= level);
        return defaults;
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
        "Farming" => Farming::search(query).iter().map(|x| x.details()).collect(),
        "Firemaking" => Firemaking::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Fishing" => Fishing::search(query).iter().map(|x| x.details()).collect(),
        "Fletching" => Fletching::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Herblore" => Herblore::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Hunter" => Hunter::search(query).iter().map(|x| x.details()).collect(),
        "Magic" => Magic::search(query).iter().map(|x| x.details()).collect(),
        "Mining" => Mining::search(query).iter().map(|x| x.details()).collect(),
        "Prayer" => Prayer::search(query).iter().map(|x| x.details()).collect(),
        "Runecraft" => Runecraft::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Sailing" => Sailing::search(query).iter().map(|x| x.details()).collect(),
        "Smithing" => Smithing::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Thieving" => Thieving::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Woodcutting" => Woodcutting::search(query)
            .iter()
            .map(|x| x.details())
            .collect(),
        "Attack" | "Strength" | "Defence" | "Ranged" => {
            Npc::search(query).iter().map(|x| x.details()).collect()
        }
        "Hitpoints" => Npc::search(query)
            .iter()
            .map(|x| x.hitpoints_details())
            .collect(),
        "Slayer" => Npc::search(query)
            .iter()
            .map(|x| x.slayer_details())
            .collect(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn levels(skill: &str, query: &str, level: u32) -> Vec<u32> {
        details_by_skill_id(skill_id(skill.to_string()), query, level)
            .iter()
            .map(|detail| detail.level())
            .collect()
    }

    #[test]
    fn defaults_only_include_actions_the_level_can_do() {
        let all = levels("Agility", "", 99);
        let some = levels("Agility", "", 60);
        assert!(some.iter().all(|level| *level <= 60));
        assert!(some.len() < all.len());
    }

    #[test]
    fn defaults_are_empty_below_every_requirement() {
        assert!(levels("Agility", "", 1).is_empty());
    }

    #[test]
    fn a_query_ignores_the_level() {
        assert!(levels("Agility", "ardougne", 1).contains(&90));
    }
}
