use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use common::{c1, c2, p};
use regex::Regex;
use std::ops::Add;

pub enum Magic {
    WindStrike,
    Confuse,
    EnchantOpalBolt,
    WaterStrike,
    ArceuusLibraryTeleport,
    EnchantSapphireBolt,
    EnchantSapphireJewellery,
    EarthStrike,
    Weaken,
    FireStrike,
    EnchantJadeBolt,
    BonesToBananas,
    BasicReanimation,
    WindBolt,
    DraynorManorTeleport,
    Curse,
    Bind,
    LowLevelAlchemy,
    WaterBolt,
    EnchantPearlBolt,
    VarrockTeleport,
    EnchantEmeraldBolt,
    EnchantEmeraldJewellery,
    MindAltarTeleport,
    EarthBolt,
    EnchantTopazBolt,
    LumbridgeTeleport,
    TelekineticGrab,
    RespawnTeleport,
    FireBolt,
    GhostlyGrasp,
    FaladorTeleport,
    ResurrectLesserThrall,
    CrumbleUndead,
    SalveGraveyardTeleport,
    TeleportToHouse,
    WindBlast,
    AdeptReanimation,
    SuperheatItem,
    InferiorDemonbane,
    CamelotTeleport,
    WaterBlast,
    ShadowVeil,
    FenkenstrainsCastleTeleport,
    KourendCastleTeleport,
    EnchantRubyBolt,
    EnchantRubyJewellery,
    IbanBlast,
    MagicDart,
    SmokeRush,
    DarkLure,
    Snare,
    ArdougneTeleport,
    ShadowRush,
    EarthBlast,
    CivitasIllaFortisTeleport,
    PaddewwaTeleport,
    HighLevelAlchemy,
    BloodRush,
    SkeletalGrasp,
    ChargeWaterOrb,
    EnchantDiamondBolt,
    EnchantDiamondJewellery,
    ResurrectSuperiorThrall,
    IceRush,
    WatchtowerTeleport,
    FireBlast,
    MarkOfDarkness,
    ClawsOfGuthix,
    FlamesOfZamorak,
    SaradominStrike,
    BonesToPeaches,
    ChargeEarthOrb,
    SenntistenTeleport,
    TrollheimTeleport,
    WestArdougneTeleport,
    SmokeBurst,
    SuperiorDemonbane,
    WindWave,
    ChargeFireOrb,
    ShadowBurst,
    TeleportApeAtoll,
    LesserCorruption,
    WaterWave,
    BakePie,
    Geomancy,
    HarmonyIslandTeleport,
    CurePlant,
    MonsterExamine,
    ChargeAirOrb,
    KharyrllTeleport,
    VileVigour,
    Vulnerability,
    NpcContact,
    BloodBurst,
    CureOther,
    Humidify,
    EnchantDragonstoneBolt,
    EnchantDragonstoneJewellery,
    MoonclanTeleport,
    EarthWave,
    IceBurst,
    TeleGroupMoonclan,
    Degrime,
    CureMe,
    OuraniaTeleport,
    HunterKit,
    CemeteryTeleport,
    WaterbirthTeleport,
    LassarTeleport,
    ExpertReanimation,
    TeleGroupWaterbirth,
    Enfeeble,
    WardOfArceuus,
    SmokeBlitz,
    CureGroup,
    TeleotherLumbridge,
    FireWave,
    BarbarianTeleport,
    StatSpy,
    ShadowBlitz,
    SpinFlax,
    TeleGroupBarbarian,
    ResurrectGreaterThrall,
    SuperglassMake,
    KhazardTeleport,
    TanLeather,
    DareeyakTeleport,
    ResurrectCrops,
    UndeadGrasp,
    TeleGroupKhazard,
    Dream,
    Entangle,
    BloodBlitz,
    StringJewellery,
    DeathCharge,
    Stun,
    Charge,
    WindSurge,
    StatRestorePotShare,
    DarkDemonbane,
    IceBlitz,
    MagicImbue,
    TeleotherFalador,
    FertileSoil,
    BarrowsTeleport,
    CarrallangerTeleport,
    BoostPotionShare,
    DemonicOffering,
    TeleportToTarget,
    WaterSurge,
    TeleBlock,
    FishingGuildTeleport,
    GreaterCorruption,
    SmokeBarrage,
    PlankMake,
    TeleGroupFishingGuild,
    CatherbyTeleport,
    EnchantOnyxBolt,
    EnchantOnyxJewellery,
    ShadowBarrage,
    TeleGroupCatherby,
    IcePlateauTeleport,
    RechargeDragonstone,
    EarthSurge,
    TeleGroupIcePlateau,
    AnnakarlTeleport,
    ApeAtollTeleport,
    TeleotherCamelot,
    MasterReanimation,
    EnergyTransfer,
    BloodBarrage,
    HealOther,
    SinisterOffering,
    VengeanceOther,
    EnchantZenyteJewellery,
    IceBarrage,
    Vengeance,
    FireSurge,
    HealGroup,
    GhorrockTeleport,
    SpellbookSwap,
}

impl Detail for Magic {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Magic(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Magic(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Magic(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Magic {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::WindStrike,
            Self::Confuse,
            Self::EnchantOpalBolt,
            Self::WaterStrike,
            Self::ArceuusLibraryTeleport,
            Self::EnchantSapphireBolt,
            Self::EnchantSapphireJewellery,
            Self::EarthStrike,
            Self::Weaken,
            Self::FireStrike,
            Self::EnchantJadeBolt,
            Self::BonesToBananas,
            Self::BasicReanimation,
            Self::WindBolt,
            Self::DraynorManorTeleport,
            Self::Curse,
            Self::Bind,
            Self::LowLevelAlchemy,
            Self::WaterBolt,
            Self::EnchantPearlBolt,
            Self::VarrockTeleport,
            Self::EnchantEmeraldBolt,
            Self::EnchantEmeraldJewellery,
            Self::MindAltarTeleport,
            Self::EarthBolt,
            Self::EnchantTopazBolt,
            Self::LumbridgeTeleport,
            Self::TelekineticGrab,
            Self::RespawnTeleport,
            Self::FireBolt,
            Self::GhostlyGrasp,
            Self::FaladorTeleport,
            Self::ResurrectLesserThrall,
            Self::CrumbleUndead,
            Self::SalveGraveyardTeleport,
            Self::TeleportToHouse,
            Self::WindBlast,
            Self::AdeptReanimation,
            Self::SuperheatItem,
            Self::InferiorDemonbane,
            Self::CamelotTeleport,
            Self::WaterBlast,
            Self::ShadowVeil,
            Self::FenkenstrainsCastleTeleport,
            Self::KourendCastleTeleport,
            Self::EnchantRubyBolt,
            Self::EnchantRubyJewellery,
            Self::IbanBlast,
            Self::MagicDart,
            Self::SmokeRush,
            Self::DarkLure,
            Self::Snare,
            Self::ArdougneTeleport,
            Self::ShadowRush,
            Self::EarthBlast,
            Self::CivitasIllaFortisTeleport,
            Self::PaddewwaTeleport,
            Self::HighLevelAlchemy,
            Self::BloodRush,
            Self::SkeletalGrasp,
            Self::ChargeWaterOrb,
            Self::EnchantDiamondBolt,
            Self::EnchantDiamondJewellery,
            Self::ResurrectSuperiorThrall,
            Self::IceRush,
            Self::WatchtowerTeleport,
            Self::FireBlast,
            Self::MarkOfDarkness,
            Self::ClawsOfGuthix,
            Self::FlamesOfZamorak,
            Self::SaradominStrike,
            Self::BonesToPeaches,
            Self::ChargeEarthOrb,
            Self::SenntistenTeleport,
            Self::TrollheimTeleport,
            Self::WestArdougneTeleport,
            Self::SmokeBurst,
            Self::SuperiorDemonbane,
            Self::WindWave,
            Self::ChargeFireOrb,
            Self::ShadowBurst,
            Self::TeleportApeAtoll,
            Self::LesserCorruption,
            Self::WaterWave,
            Self::BakePie,
            Self::Geomancy,
            Self::HarmonyIslandTeleport,
            Self::CurePlant,
            Self::MonsterExamine,
            Self::ChargeAirOrb,
            Self::KharyrllTeleport,
            Self::VileVigour,
            Self::Vulnerability,
            Self::NpcContact,
            Self::BloodBurst,
            Self::CureOther,
            Self::Humidify,
            Self::EnchantDragonstoneBolt,
            Self::EnchantDragonstoneJewellery,
            Self::MoonclanTeleport,
            Self::EarthWave,
            Self::IceBurst,
            Self::TeleGroupMoonclan,
            Self::Degrime,
            Self::CureMe,
            Self::OuraniaTeleport,
            Self::HunterKit,
            Self::CemeteryTeleport,
            Self::WaterbirthTeleport,
            Self::LassarTeleport,
            Self::ExpertReanimation,
            Self::TeleGroupWaterbirth,
            Self::Enfeeble,
            Self::WardOfArceuus,
            Self::SmokeBlitz,
            Self::CureGroup,
            Self::TeleotherLumbridge,
            Self::FireWave,
            Self::BarbarianTeleport,
            Self::StatSpy,
            Self::ShadowBlitz,
            Self::SpinFlax,
            Self::TeleGroupBarbarian,
            Self::ResurrectGreaterThrall,
            Self::SuperglassMake,
            Self::KhazardTeleport,
            Self::TanLeather,
            Self::DareeyakTeleport,
            Self::ResurrectCrops,
            Self::UndeadGrasp,
            Self::TeleGroupKhazard,
            Self::Dream,
            Self::Entangle,
            Self::BloodBlitz,
            Self::StringJewellery,
            Self::DeathCharge,
            Self::Stun,
            Self::Charge,
            Self::WindSurge,
            Self::StatRestorePotShare,
            Self::DarkDemonbane,
            Self::IceBlitz,
            Self::MagicImbue,
            Self::TeleotherFalador,
            Self::FertileSoil,
            Self::BarrowsTeleport,
            Self::CarrallangerTeleport,
            Self::BoostPotionShare,
            Self::DemonicOffering,
            Self::TeleportToTarget,
            Self::WaterSurge,
            Self::TeleBlock,
            Self::FishingGuildTeleport,
            Self::GreaterCorruption,
            Self::SmokeBarrage,
            Self::PlankMake,
            Self::TeleGroupFishingGuild,
            Self::CatherbyTeleport,
            Self::EnchantOnyxBolt,
            Self::EnchantOnyxJewellery,
            Self::ShadowBarrage,
            Self::TeleGroupCatherby,
            Self::IcePlateauTeleport,
            Self::RechargeDragonstone,
            Self::EarthSurge,
            Self::TeleGroupIcePlateau,
            Self::AnnakarlTeleport,
            Self::ApeAtollTeleport,
            Self::TeleotherCamelot,
            Self::MasterReanimation,
            Self::EnergyTransfer,
            Self::BloodBarrage,
            Self::HealOther,
            Self::SinisterOffering,
            Self::VengeanceOther,
            Self::EnchantZenyteJewellery,
            Self::IceBarrage,
            Self::Vengeance,
            Self::FireSurge,
            Self::HealGroup,
            Self::GhorrockTeleport,
            Self::SpellbookSwap,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::VarrockTeleport,
            Self::CamelotTeleport,
            Self::ArdougneTeleport,
            Self::HighLevelAlchemy,
            Self::MagicImbue,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::WindStrike => ("Wind Strike", 1, 5.5, false),
            Self::Confuse => ("Confuse", 3, 13.0, false),
            Self::EnchantOpalBolt => ("Enchant Opal Bolt", 4, 9.0, true),
            Self::WaterStrike => ("Water Strike", 5, 7.5, false),
            Self::ArceuusLibraryTeleport => ("Arceuus Library Teleport", 6, 10.0, true),
            Self::EnchantSapphireBolt => ("Enchant Sapphire Bolt", 7, 17.5, true),
            Self::EnchantSapphireJewellery => ("Enchant Sapphire Jewellery", 7, 17.5, false),
            Self::EarthStrike => ("Earth Strike", 9, 9.5, false),
            Self::Weaken => ("Weaken", 11, 21.0, false),
            Self::FireStrike => ("Fire Strike", 13, 11.5, false),
            Self::EnchantJadeBolt => ("Enchant Jade Bolt", 14, 19.0, true),
            Self::BonesToBananas => ("Bones To Bananas", 15, 25.0, false),
            Self::BasicReanimation => ("Basic Reanimation", 16, 32.0, true),
            Self::WindBolt => ("Wind Bolt", 17, 13.5, false),
            Self::DraynorManorTeleport => ("Draynor Manor Teleport", 17, 16.0, true),
            Self::Curse => ("Curse", 19, 29.0, false),
            Self::Bind => ("Bind", 20, 30.0, false),
            Self::LowLevelAlchemy => ("Low Level Alchemy", 21, 31.0, false),
            Self::WaterBolt => ("Water Bolt", 23, 16.5, false),
            Self::EnchantPearlBolt => ("Enchant Pearl Bolt", 24, 29.0, true),
            Self::VarrockTeleport => ("Varrock Teleport", 25, 35.0, false),
            Self::EnchantEmeraldBolt => ("Enchant Emerald Bolt", 27, 37.0, true),
            Self::EnchantEmeraldJewellery => ("Enchant Emerald Jewellery", 27, 37.0, false),
            Self::MindAltarTeleport => ("Mind Altar Teleport", 28, 22.0, true),
            Self::EarthBolt => ("Earth Bolt", 29, 19.5, false),
            Self::EnchantTopazBolt => ("Enchant Topaz Bolt", 29, 33.0, true),
            Self::LumbridgeTeleport => ("Lumbridge Teleport", 31, 41.0, false),
            Self::TelekineticGrab => ("Telekinetic Grab", 33, 43.0, false),
            Self::RespawnTeleport => ("Respawn Teleport", 34, 27.0, true),
            Self::FireBolt => ("Fire Bolt", 35, 22.5, false),
            Self::GhostlyGrasp => ("Ghostly Grasp", 35, 22.5, true),
            Self::FaladorTeleport => ("Falador Teleport", 37, 48.0, false),
            Self::ResurrectLesserThrall => ("Resurrect Lesser Thrall", 38, 55.0, true),
            Self::CrumbleUndead => ("Crumble Undead", 39, 24.5, false),
            Self::SalveGraveyardTeleport => ("Salve Graveyard Teleport", 40, 30.0, true),
            Self::TeleportToHouse => ("Teleport To House", 40, 30.0, true),
            Self::WindBlast => ("Wind Blast", 41, 25.5, false),
            Self::AdeptReanimation => ("Adept Reanimation", 41, 80.0, true),
            Self::SuperheatItem => ("Superheat Item", 43, 53.0, false),
            Self::InferiorDemonbane => ("Inferior Demonbane", 44, 27.0, true),
            Self::CamelotTeleport => ("Camelot Teleport", 45, 55.5, true),
            Self::WaterBlast => ("Water Blast", 47, 28.5, false),
            Self::ShadowVeil => ("Shadow Veil", 47, 58.0, true),
            Self::FenkenstrainsCastleTeleport => ("Fenkenstrain's Castle Teleport", 48, 50.0, true),
            Self::KourendCastleTeleport => ("Kourend Castle Teleport", 48, 58.0, true),
            Self::EnchantRubyBolt => ("Enchant Ruby Bolt", 49, 59.0, true),
            Self::EnchantRubyJewellery => ("Enchant Ruby Jewellery", 49, 59.0, false),
            Self::IbanBlast => ("Iban Blast", 50, 30.0, true),
            Self::MagicDart => ("Magic Dart", 50, 30.0, true),
            Self::SmokeRush => ("Smoke Rush", 50, 30.0, true),
            Self::DarkLure => ("Dark Lure", 50, 60.0, true),
            Self::Snare => ("Snare", 50, 60.0, false),
            Self::ArdougneTeleport => ("Ardougne Teleport", 51, 61.0, true),
            Self::ShadowRush => ("Shadow Rush", 52, 31.0, true),
            Self::EarthBlast => ("Earth Blast", 53, 31.5, false),
            Self::CivitasIllaFortisTeleport => ("Civitas illa Fortis Teleport", 54, 64.0, true),
            Self::PaddewwaTeleport => ("Paddewwa Teleport", 54, 64.0, true),
            Self::HighLevelAlchemy => ("High Level Alchemy", 55, 65.0, false),
            Self::BloodRush => ("Blood Rush", 56, 33.0, true),
            Self::SkeletalGrasp => ("Skeletal Grasp", 56, 33.0, true),
            Self::ChargeWaterOrb => ("Charge Water Orb", 56, 66.0, true),
            Self::EnchantDiamondBolt => ("Enchant Diamond Bolt", 57, 67.0, true),
            Self::EnchantDiamondJewellery => ("Enchant Diamond Jewellery", 57, 67.0, false),
            Self::ResurrectSuperiorThrall => ("Resurrect Superior Thrall", 57, 70.0, true),
            Self::IceRush => ("Ice Rush", 58, 34.0, true),
            Self::WatchtowerTeleport => ("Watchtower Teleport", 58, 68.0, true),
            Self::FireBlast => ("Fire Blast", 59, 34.5, false),
            Self::MarkOfDarkness => ("Mark of Darkness", 59, 70.0, true),
            Self::ClawsOfGuthix => ("Claws Of Guthix", 60, 35.0, true),
            Self::FlamesOfZamorak => ("Flames Of Zamorak", 60, 35.0, true),
            Self::SaradominStrike => ("Saradomin Strike", 60, 35.0, true),
            Self::BonesToPeaches => ("Bones To Peaches", 60, 35.5, true),
            Self::ChargeEarthOrb => ("Charge Earth Orb", 60, 70.0, true),
            Self::SenntistenTeleport => ("Senntisten Teleport", 60, 70.0, true),
            Self::TrollheimTeleport => ("Trollheim Teleport", 61, 68.0, true),
            Self::WestArdougneTeleport => ("West Ardougne Teleport", 61, 68.0, true),
            Self::SmokeBurst => ("Smoke Burst", 62, 36.0, true),
            Self::SuperiorDemonbane => ("Superior Demonbane", 62, 36.0, true),
            Self::WindWave => ("Wind Wave", 62, 36.0, true),
            Self::ChargeFireOrb => ("Charge Fire Orb", 63, 73.0, true),
            Self::ShadowBurst => ("Shadow Burst", 64, 37.0, true),
            Self::TeleportApeAtoll => ("Teleport Ape Atoll", 64, 74.0, true),
            Self::LesserCorruption => ("Lesser Corruption", 64, 75.0, true),
            Self::WaterWave => ("Water Wave", 65, 37.5, true),
            Self::BakePie => ("Bake Pie", 65, 60.0, true),
            Self::Geomancy => ("Geomancy", 65, 60.0, true),
            Self::HarmonyIslandTeleport => ("Harmony Island Teleport", 65, 74.0, true),
            Self::CurePlant => ("Cure Plant", 66, 60.0, true),
            Self::MonsterExamine => ("Monster Examine", 66, 61.0, true),
            Self::ChargeAirOrb => ("Charge Air Orb", 66, 76.0, true),
            Self::KharyrllTeleport => ("Kharyrll Teleport", 66, 76.0, true),
            Self::VileVigour => ("Vile Vigour", 66, 76.0, true),
            Self::Vulnerability => ("Vulnerability", 66, 76.0, true),
            Self::NpcContact => ("Npc Contact", 67, 63.0, true),
            Self::BloodBurst => ("Blood Burst", 68, 39.0, true),
            Self::CureOther => ("Cure Other", 68, 65.0, true),
            Self::Humidify => ("Humidify", 68, 65.0, true),
            Self::EnchantDragonstoneBolt => ("Enchant Dragonstone Bolt", 68, 78.0, true),
            Self::EnchantDragonstoneJewellery => ("Enchant Dragonstone Jewellery", 68, 78.0, true),
            Self::MoonclanTeleport => ("Moonclan Teleport", 69, 66.0, true),
            Self::EarthWave => ("Earth Wave", 70, 40.0, true),
            Self::IceBurst => ("Ice Burst", 70, 40.0, true),
            Self::TeleGroupMoonclan => ("Tele Group Moonclan", 70, 67.0, true),
            Self::Degrime => ("Degrime", 70, 83.0, true),
            Self::CureMe => ("Cure Me", 71, 69.0, true),
            Self::OuraniaTeleport => ("Ourania Teleport", 71, 69.0, true),
            Self::HunterKit => ("Hunter Kit", 71, 70.0, true),
            Self::CemeteryTeleport => ("Cemetery Teleport", 71, 82.0, true),
            Self::WaterbirthTeleport => ("Waterbirth Teleport", 72, 71.0, true),
            Self::LassarTeleport => ("Lassar Teleport", 72, 82.0, true),
            Self::ExpertReanimation => ("Expert Reanimation", 72, 138.0, true),
            Self::TeleGroupWaterbirth => ("Tele Group Waterbirth", 73, 72.0, true),
            Self::Enfeeble => ("Enfeeble", 73, 83.0, true),
            Self::WardOfArceuus => ("Ward of Arceuus", 73, 83.0, true),
            Self::SmokeBlitz => ("Smoke Blitz", 74, 42.0, true),
            Self::CureGroup => ("Cure Group", 74, 74.0, true),
            Self::TeleotherLumbridge => ("Teleother Lumbridge", 74, 84.0, true),
            Self::FireWave => ("Fire Wave", 75, 42.5, true),
            Self::BarbarianTeleport => ("Barbarian Teleport", 75, 76.0, true),
            Self::StatSpy => ("Stat Spy", 75, 76.0, true),
            Self::ShadowBlitz => ("Shadow Blitz", 76, 43.0, true),
            Self::SpinFlax => ("Spin Flax", 76, 75.0, true),
            Self::TeleGroupBarbarian => ("Tele Group Barbarian", 76, 77.0, true),
            Self::ResurrectGreaterThrall => ("Resurrect Greater Thrall", 76, 88.0, true),
            Self::SuperglassMake => ("Superglass Make", 77, 78.0, true),
            Self::KhazardTeleport => ("Khazard Teleport", 78, 80.0, true),
            Self::TanLeather => ("Tan Leather", 78, 81.0, true),
            Self::DareeyakTeleport => ("Dareeyak Teleport", 78, 88.0, true),
            Self::ResurrectCrops => ("Resurrect Crops", 78, 90.0, true),
            Self::UndeadGrasp => ("Undead Grasp", 79, 46.5, true),
            Self::TeleGroupKhazard => ("Tele Group Khazard", 79, 81.0, true),
            Self::Dream => ("Dream", 79, 82.0, true),
            Self::Entangle => ("Entangle", 79, 89.0, true),
            Self::BloodBlitz => ("Blood Blitz", 80, 45.0, true),
            Self::StringJewellery => ("String Jewellery", 80, 83.0, true),
            Self::DeathCharge => ("Death Charge", 80, 90.0, true),
            Self::Stun => ("Stun", 80, 90.0, true),
            Self::Charge => ("Charge", 80, 180.0, true),
            Self::WindSurge => ("Wind Surge", 81, 44.5, true),
            Self::StatRestorePotShare => ("Stat Restore Pot Share", 81, 84.0, true),
            Self::DarkDemonbane => ("Dark Demonbane", 82, 43.5, true),
            Self::IceBlitz => ("Ice Blitz", 82, 46.0, true),
            Self::MagicImbue => ("Magic Imbue", 82, 86.0, true),
            Self::TeleotherFalador => ("Teleother Falador", 82, 92.0, true),
            Self::FertileSoil => ("Fertile Soil", 83, 87.0, true),
            Self::BarrowsTeleport => ("Barrows Teleport", 83, 90.0, true),
            Self::CarrallangerTeleport => ("Carrallanger Teleport", 84, 82.0, true),
            Self::BoostPotionShare => ("Boost Potion Share", 84, 88.0, true),
            Self::DemonicOffering => ("Demonic Offering", 84, 175.0, true),
            Self::TeleportToTarget => ("Teleport To Target", 85, 45.0, true),
            Self::WaterSurge => ("Water Surge", 85, 46.5, true),
            Self::TeleBlock => ("Tele Block", 85, 80.0, false),
            Self::FishingGuildTeleport => ("Fishing Guild Teleport", 85, 89.0, true),
            Self::GreaterCorruption => ("Greater Corruption", 85, 95.0, true),
            Self::SmokeBarrage => ("Smoke Barrage", 86, 48.0, true),
            Self::PlankMake => ("Plank Make", 86, 90.0, true),
            Self::TeleGroupFishingGuild => ("Tele Group Fishing Guild", 86, 90.0, true),
            Self::CatherbyTeleport => ("Catherby Teleport", 87, 92.0, true),
            Self::EnchantOnyxBolt => ("Enchant Onyx Bolt", 87, 97.0, true),
            Self::EnchantOnyxJewellery => ("Enchant Onyx Jewellery", 87, 97.0, true),
            Self::ShadowBarrage => ("Shadow Barrage", 88, 48.0, true),
            Self::TeleGroupCatherby => ("Tele Group Catherby", 88, 93.0, true),
            Self::IcePlateauTeleport => ("Ice Plateau Teleport", 89, 96.0, true),
            Self::RechargeDragonstone => ("Recharge Dragonstone", 89, 97.5, true),
            Self::EarthSurge => ("Earth Surge", 90, 48.5, true),
            Self::TeleGroupIcePlateau => ("Tele Group Ice Plateau", 90, 99.0, true),
            Self::AnnakarlTeleport => ("Annakarl Teleport", 90, 100.0, true),
            Self::ApeAtollTeleport => ("Ape Atoll Teleport", 90, 100.0, true),
            Self::TeleotherCamelot => ("Teleother Camelot", 90, 100.0, true),
            Self::MasterReanimation => ("Master Reanimation", 90, 170.0, true),
            Self::EnergyTransfer => ("Energy Transfer", 91, 100.0, true),
            Self::BloodBarrage => ("Blood Barrage", 92, 51.0, true),
            Self::HealOther => ("Heal Other", 92, 101.0, true),
            Self::SinisterOffering => ("Sinister Offering", 92, 180.0, true),
            Self::VengeanceOther => ("Vengeance Other", 93, 108.0, true),
            Self::EnchantZenyteJewellery => ("Enchant Zenyte Jewellery", 93, 110.0, true),
            Self::IceBarrage => ("Ice Barrage", 94, 52.0, true),
            Self::Vengeance => ("Vengeance", 94, 112.0, true),
            Self::FireSurge => ("Fire Surge", 95, 50.5, true),
            Self::HealGroup => ("Heal Group", 95, 124.0, true),
            Self::GhorrockTeleport => ("Ghorrock Teleport", 96, 106.0, true),
            Self::SpellbookSwap => ("Spellbook Swap", 96, 130.0, true),
        };

        Details::Magic(MagicDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            members: details.3,
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
pub struct MagicDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub members: bool,
}

impl IntoString for MagicDetails {
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
