use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use common::{c1, c2};
use regex::Regex;
use std::ops::Add;

pub enum Crafting {
    BallOfWool,
    Pot,
    UnfiredPot,
    OpalRing,
    LeatherGloves,
    Opal,
    BeerGlass,
    MoltenGlass,
    EmptyCandleLantern,
    BirdHouse,
    GoldRing,
    GoldNecklace,
    PieDish,
    UnfiredPieDish,
    LeatherBoots,
    GoldBracelet,
    Bowl,
    UnfiredBowl,
    GoldAmuletU,
    Cowl,
    BowString,
    CrossbowString,
    LeatherVambraces,
    EmptyOilLamp,
    Jade,
    JadeRing,
    LeatherBody,
    OakBirdHouse,
    RedTopaz,
    OpalNecklace,
    TopazRing,
    HolySymbol,
    UnholySymbol,
    LeatherChaps,
    EmptyPlantPot,
    UnfiredPlantPot,
    MagicString,
    SapphireRing,
    Sapphire,
    EmptySack,
    OpalBracelet,
    SapphireNecklace,
    Tiara,
    SapphireBracelet,
    SapphireAmuletU,
    PotLid,
    UnfiredPotLid,
    WillowBirdHouse,
    JadeNecklace,
    DriftNet,
    EmeraldRing,
    OpalAmuletU,
    Emerald,
    HardleatherBody,
    EmeraldNecklace,
    JadeBracelet,
    Rope,
    EmeraldBracelet,
    EmeraldAmuletU,
    SpikyVambraces,
    TopazNecklace,
    Vial,
    JadeAmuletU,
    RubyRing,
    Ruby,
    TeakBirdHouse,
    BroodooShield,
    Basket,
    Coif,
    TopazBracelet,
    RubyNecklace,
    HardLeatherShield,
    GoldTiara,
    Fishbowl,
    RubyBracelet,
    DiamondRing,
    Diamond,
    SnakeskinBoots,
    MapleBirdHouse,
    TopazAmuletU,
    UnpoweredOrb,
    SnakeskinVambraces,
    SnakeskinBandana,
    LanternLens,
    MahoganyBirdHouse,
    RubyAmuletU,
    SnakeskinChaps,
    SnakeskinBody,
    WaterBattlestaff,
    DragonstoneRing,
    Dragonstone,
    DiamondNecklace,
    SnakeskinShield,
    GreenDhideVamb,
    DiamondBracelet,
    EarthBattlestaff,
    YewBirdHouse,
    GreenDhideChaps,
    GreenDhideShield,
    FireBattlestaff,
    GreenDhideBody,
    BlueDhideVamb,
    AirBattlestaff,
    OnyxRing,
    Onyx,
    BlueDhideChaps,
    BlueDhideShield,
    DiamondAmuletU,
    BlueDhideBody,
    DragonstoneNecklace,
    RedDhideVamb,
    DragonstoneBracelet,
    MagicBirdHouse,
    RedDhideChaps,
    RedDhideShield,
    RedDhideBody,
    BlackDhideVamb,
    DragonstoneAmuletU,
    OnyxNecklace,
    BlackDhideChaps,
    AmethystBoltTips,
    BlackDhideShield,
    OnyxBracelet,
    BlackDhideBody,
    AmethystArrowtips,
    AmethystJavelinHeads,
    LightOrb,
    AmethystDartTip,
    ZenyteRing,
    Zenyte,
    RedwoodBirdHouse,
    OnyxAmuletU,
    ZenyteNecklace,
    ZenyteBracelet,
    ZenyteAmuletU,
}

impl Detail for Crafting {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Crafting(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Crafting(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Crafting(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Crafting {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::BallOfWool,
            Self::Pot,
            Self::UnfiredPot,
            Self::OpalRing,
            Self::LeatherGloves,
            Self::Opal,
            Self::BeerGlass,
            Self::MoltenGlass,
            Self::EmptyCandleLantern,
            Self::BirdHouse,
            Self::GoldRing,
            Self::GoldNecklace,
            Self::PieDish,
            Self::UnfiredPieDish,
            Self::LeatherBoots,
            Self::GoldBracelet,
            Self::Bowl,
            Self::UnfiredBowl,
            Self::GoldAmuletU,
            Self::Cowl,
            Self::BowString,
            Self::CrossbowString,
            Self::LeatherVambraces,
            Self::EmptyOilLamp,
            Self::Jade,
            Self::JadeRing,
            Self::LeatherBody,
            Self::OakBirdHouse,
            Self::RedTopaz,
            Self::OpalNecklace,
            Self::TopazRing,
            Self::HolySymbol,
            Self::UnholySymbol,
            Self::LeatherChaps,
            Self::EmptyPlantPot,
            Self::UnfiredPlantPot,
            Self::MagicString,
            Self::SapphireRing,
            Self::Sapphire,
            Self::EmptySack,
            Self::OpalBracelet,
            Self::SapphireNecklace,
            Self::Tiara,
            Self::SapphireBracelet,
            Self::SapphireAmuletU,
            Self::PotLid,
            Self::UnfiredPotLid,
            Self::WillowBirdHouse,
            Self::JadeNecklace,
            Self::DriftNet,
            Self::EmeraldRing,
            Self::OpalAmuletU,
            Self::Emerald,
            Self::HardleatherBody,
            Self::EmeraldNecklace,
            Self::JadeBracelet,
            Self::Rope,
            Self::EmeraldBracelet,
            Self::EmeraldAmuletU,
            Self::SpikyVambraces,
            Self::TopazNecklace,
            Self::Vial,
            Self::JadeAmuletU,
            Self::RubyRing,
            Self::Ruby,
            Self::TeakBirdHouse,
            Self::BroodooShield,
            Self::Basket,
            Self::Coif,
            Self::TopazBracelet,
            Self::RubyNecklace,
            Self::HardLeatherShield,
            Self::GoldTiara,
            Self::Fishbowl,
            Self::RubyBracelet,
            Self::DiamondRing,
            Self::Diamond,
            Self::SnakeskinBoots,
            Self::MapleBirdHouse,
            Self::TopazAmuletU,
            Self::UnpoweredOrb,
            Self::SnakeskinVambraces,
            Self::SnakeskinBandana,
            Self::LanternLens,
            Self::MahoganyBirdHouse,
            Self::RubyAmuletU,
            Self::SnakeskinChaps,
            Self::SnakeskinBody,
            Self::WaterBattlestaff,
            Self::DragonstoneRing,
            Self::Dragonstone,
            Self::DiamondNecklace,
            Self::SnakeskinShield,
            Self::GreenDhideVamb,
            Self::DiamondBracelet,
            Self::EarthBattlestaff,
            Self::YewBirdHouse,
            Self::GreenDhideChaps,
            Self::GreenDhideShield,
            Self::FireBattlestaff,
            Self::GreenDhideBody,
            Self::BlueDhideVamb,
            Self::AirBattlestaff,
            Self::OnyxRing,
            Self::Onyx,
            Self::BlueDhideChaps,
            Self::BlueDhideShield,
            Self::DiamondAmuletU,
            Self::BlueDhideBody,
            Self::DragonstoneNecklace,
            Self::RedDhideVamb,
            Self::DragonstoneBracelet,
            Self::MagicBirdHouse,
            Self::RedDhideChaps,
            Self::RedDhideShield,
            Self::RedDhideBody,
            Self::BlackDhideVamb,
            Self::DragonstoneAmuletU,
            Self::OnyxNecklace,
            Self::BlackDhideChaps,
            Self::AmethystBoltTips,
            Self::BlackDhideShield,
            Self::OnyxBracelet,
            Self::BlackDhideBody,
            Self::AmethystArrowtips,
            Self::AmethystJavelinHeads,
            Self::LightOrb,
            Self::AmethystDartTip,
            Self::ZenyteRing,
            Self::Zenyte,
            Self::RedwoodBirdHouse,
            Self::OnyxAmuletU,
            Self::ZenyteNecklace,
            Self::ZenyteBracelet,
            Self::ZenyteAmuletU,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::LeatherBody,
            Self::WaterBattlestaff,
            Self::EarthBattlestaff,
            Self::FireBattlestaff,
            Self::AirBattlestaff,
            Self::GreenDhideBody,
            Self::BlueDhideBody,
            Self::RedDhideBody,
            Self::BlackDhideBody,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::BallOfWool => ("Ball Of Wool", 1, 2.5),
            Self::Pot => ("Pot", 1, 6.3),
            Self::UnfiredPot => ("Unfired Pot", 1, 6.3),
            Self::OpalRing => ("Opal Ring", 1, 10.0),
            Self::LeatherGloves => ("Leather Gloves", 1, 13.8),
            Self::Opal => ("Opal", 1, 15.0),
            Self::BeerGlass => ("Beer Glass", 1, 17.5),
            Self::MoltenGlass => ("Molten Glass", 1, 20.0),
            Self::EmptyCandleLantern => ("Empty Candle Lantern", 4, 19.0),
            Self::BirdHouse => ("Bird House", 5, 15.0),
            Self::GoldRing => ("Gold Ring", 5, 15.0),
            Self::GoldNecklace => ("Gold Necklace", 6, 20.0),
            Self::PieDish => ("Pie Dish", 7, 10.0),
            Self::UnfiredPieDish => ("Unfired Pie Dish", 7, 15.0),
            Self::LeatherBoots => ("Leather Boots", 7, 16.3),
            Self::GoldBracelet => ("Gold Bracelet 11069", 7, 25.0),
            Self::Bowl => ("Bowl", 8, 15.0),
            Self::UnfiredBowl => ("Unfired Bowl", 8, 18.0),
            Self::GoldAmuletU => ("Gold Amulet U", 8, 30.0),
            Self::Cowl => ("Leather Cowl", 9, 18.5),
            Self::BowString => ("Bow String", 10, 15.0),
            Self::CrossbowString => ("Crossbow String", 10, 15.0),
            Self::LeatherVambraces => ("Leather Vambraces", 11, 22.0),
            Self::EmptyOilLamp => ("Empty Oil Lamp", 12, 25.0),
            Self::Jade => ("Jade", 13, 20.0),
            Self::JadeRing => ("Jade Ring", 13, 32.0),
            Self::LeatherBody => ("Leather Body", 14, 25.0),
            Self::OakBirdHouse => ("Oak Bird House", 15, 20.0),
            Self::RedTopaz => ("Red Topaz", 16, 25.0),
            Self::OpalNecklace => ("Opal Necklace", 16, 35.0),
            Self::TopazRing => ("Topaz Ring", 16, 35.0),
            Self::HolySymbol => ("Holy Symbol", 16, 50.0),
            Self::UnholySymbol => ("Unholy Symbol", 17, 50.0),
            Self::LeatherChaps => ("Leather Chaps", 18, 27.0),
            Self::EmptyPlantPot => ("Empty Plant Pot", 19, 17.5),
            Self::UnfiredPlantPot => ("Unfired Plant Pot", 19, 20.0),
            Self::MagicString => ("Magic String", 19, 30.0),
            Self::SapphireRing => ("Sapphire Ring", 20, 40.0),
            Self::Sapphire => ("Sapphire", 20, 50.0),
            Self::EmptySack => ("Empty Sack", 21, 38.0),
            Self::OpalBracelet => ("Opal Bracelet", 22, 45.0),
            Self::SapphireNecklace => ("Sapphire Necklace", 22, 55.0),
            Self::Tiara => ("Tiara", 23, 52.5),
            Self::SapphireBracelet => ("Sapphire Bracelet 11072", 23, 60.0),
            Self::SapphireAmuletU => ("Sapphire Amulet U", 24, 65.0),
            Self::PotLid => ("Pot Lid", 25, 20.0),
            Self::UnfiredPotLid => ("Unfired Pot Lid", 25, 20.0),
            Self::WillowBirdHouse => ("Willow Bird House", 25, 25.0),
            Self::JadeNecklace => ("Jade Necklace", 25, 54.0),
            Self::DriftNet => ("Drift Net", 26, 55.0),
            Self::EmeraldRing => ("Emerald Ring", 27, 55.0),
            Self::OpalAmuletU => ("Opal Amulet U", 27, 55.0),
            Self::Emerald => ("Emerald", 27, 67.5),
            Self::HardleatherBody => ("Hardleather Body", 28, 35.0),
            Self::EmeraldNecklace => ("Emerald Necklace", 29, 60.0),
            Self::JadeBracelet => ("Jade Bracelet", 29, 60.0),
            Self::Rope => ("Rope", 30, 25.0),
            Self::EmeraldBracelet => ("Emerald Bracelet", 30, 65.0),
            Self::EmeraldAmuletU => ("Emerald Amulet U", 31, 70.0),
            Self::SpikyVambraces => ("Spiky Vambraces", 32, 6.0),
            Self::TopazNecklace => ("Topaz Necklace", 32, 70.0),
            Self::Vial => ("Vial", 33, 35.0),
            Self::JadeAmuletU => ("Jade Amulet U", 34, 70.0),
            Self::RubyRing => ("Ruby Ring", 34, 70.0),
            Self::Ruby => ("Ruby", 34, 85.0),
            Self::TeakBirdHouse => ("Teak Bird House", 35, 30.0),
            Self::BroodooShield => ("Broodoo Shield", 35, 100.0),
            Self::Basket => ("Basket", 36, 56.0),
            Self::Coif => ("Coif", 38, 37.0),
            Self::TopazBracelet => ("Topaz Bracelet", 38, 75.0),
            Self::RubyNecklace => ("Ruby Necklace", 40, 75.0),
            Self::HardLeatherShield => ("Hard Leather Shield", 41, 70.0),
            Self::GoldTiara => ("Gold Tiara", 42, 35.0),
            Self::Fishbowl => ("Fishbowl", 42, 42.5),
            Self::RubyBracelet => ("Ruby Bracelet", 42, 80.0),
            Self::DiamondRing => ("Diamond Ring", 43, 85.0),
            Self::Diamond => ("Diamond", 43, 107.5),
            Self::SnakeskinBoots => ("Snakeskin Boots", 45, 30.0),
            Self::MapleBirdHouse => ("Maple Bird House", 45, 35.0),
            Self::TopazAmuletU => ("Topaz Amulet U", 45, 80.0),
            Self::UnpoweredOrb => ("Unpowered Orb", 46, 52.5),
            Self::SnakeskinVambraces => ("Snakeskin Vambraces", 47, 35.0),
            Self::SnakeskinBandana => ("Snakeskin Bandana", 48, 45.0),
            Self::LanternLens => ("Lantern Lens", 49, 55.0),
            Self::MahoganyBirdHouse => ("Mahogany Bird House", 50, 40.0),
            Self::RubyAmuletU => ("Ruby Amulet U", 50, 85.0),
            Self::SnakeskinChaps => ("Snakeskin Chaps", 51, 50.0),
            Self::SnakeskinBody => ("Snakeskin Body", 53, 55.0),
            Self::WaterBattlestaff => ("Water Battlestaff", 54, 100.0),
            Self::DragonstoneRing => ("Dragonstone Ring", 55, 100.0),
            Self::Dragonstone => ("Dragonstone", 55, 137.5),
            Self::DiamondNecklace => ("Diamond Necklace", 56, 90.0),
            Self::SnakeskinShield => ("Snakeskin Shield", 56, 100.0),
            Self::GreenDhideVamb => ("Green Dhide Vambraces", 57, 62.0),
            Self::DiamondBracelet => ("Diamond Bracelet", 58, 95.0),
            Self::EarthBattlestaff => ("Earth Battlestaff", 58, 112.5),
            Self::YewBirdHouse => ("Yew Bird House", 60, 45.0),
            Self::GreenDhideChaps => ("Green Dhide Chaps", 60, 124.0),
            Self::GreenDhideShield => ("Green Dhide Shield", 62, 124.0),
            Self::FireBattlestaff => ("Fire Battlestaff", 62, 125.0),
            Self::GreenDhideBody => ("Green Dhide Body", 63, 186.0),
            Self::BlueDhideVamb => ("Blue Dhide Vambraces", 66, 70.0),
            Self::AirBattlestaff => ("Air Battlestaff", 66, 137.5),
            Self::OnyxRing => ("Onyx Ring", 67, 115.0),
            Self::Onyx => ("Onyx", 67, 167.5),
            Self::BlueDhideChaps => ("Blue Dhide Chaps", 68, 140.0),
            Self::BlueDhideShield => ("Blue Dhide Shield", 69, 140.0),
            Self::DiamondAmuletU => ("Diamond Amulet U", 70, 100.0),
            Self::BlueDhideBody => ("Blue Dhide Body", 71, 210.0),
            Self::DragonstoneNecklace => ("Dragon Necklace", 72, 105.0),
            Self::RedDhideVamb => ("Red Dhide Vambraces", 73, 78.0),
            Self::DragonstoneBracelet => ("Dragonstone Bracelet", 74, 110.0),
            Self::MagicBirdHouse => ("Magic Bird House", 75, 50.0),
            Self::RedDhideChaps => ("Red Dhide Chaps", 75, 156.0),
            Self::RedDhideShield => ("Red Dhide Shield", 76, 156.0),
            Self::RedDhideBody => ("Red Dhide Body", 77, 234.0),
            Self::BlackDhideVamb => ("Black Dhide Vambraces", 79, 86.0),
            Self::DragonstoneAmuletU => ("Dragonstone Amulet U", 80, 150.0),
            Self::OnyxNecklace => ("Onyx Necklace", 82, 120.0),
            Self::BlackDhideChaps => ("Black Dhide Chaps", 82, 172.0),
            Self::AmethystBoltTips => ("Amethyst Bolt Tips", 83, 4.0),
            Self::BlackDhideShield => ("Black Dhide Shield", 83, 172.0),
            Self::OnyxBracelet => ("Onyx Bracelet", 84, 125.0),
            Self::BlackDhideBody => ("Black Dhide Body", 84, 258.0),
            Self::AmethystArrowtips => ("Amethyst Arrowtips", 85, 4.0),
            Self::AmethystJavelinHeads => ("Amethyst Javelin Heads", 87, 12.0),
            Self::LightOrb => ("Light Orb", 87, 70.0),
            Self::AmethystDartTip => ("Amethyst Dart Tip", 89, 7.5),
            Self::ZenyteRing => ("Zenyte Ring", 89, 150.0),
            Self::Zenyte => ("Zenyte", 89, 200.0),
            Self::RedwoodBirdHouse => ("Redwood Bird House", 90, 55.0),
            Self::OnyxAmuletU => ("Onyx Amulet U", 90, 165.0),
            Self::ZenyteNecklace => ("Zenyte Necklace", 92, 165.0),
            Self::ZenyteBracelet => ("Zenyte Bracelet 19532", 95, 180.0),
            Self::ZenyteAmuletU => ("Zenyte Amulet U", 98, 200.0),
        };

        Details::Crafting(CraftingDetails {
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

#[derive(Clone, PartialOrd, PartialEq)]
pub struct CraftingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
}

impl IntoString for CraftingDetails {
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
