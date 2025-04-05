use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use common::{c1, c2, p};
use regex::Regex;
use std::ops::Add;

pub enum Smithing {
    BronzeBar,
    BronzeAxe,
    BronzeDagger,
    BronzeMace,
    BronzeBoltsUnf,
    BronzeMedHelm,
    BronzeDartTip,
    BronzeNails,
    BronzeSword,
    BronzeWire,
    BronzeArrowtips,
    BronzeHasta,
    BronzeScimitar,
    BronzeSpear,
    BronzeJavelinHeads,
    BronzeLimbs,
    BronzeLongsword,
    BronzeKnife,
    BronzeFullHelm,
    BronzeSqShield,
    BronzeWarhammer,
    BronzeBattleaxe,
    BronzeChainbody,
    BronzeKiteshield,
    BronzeClaws,
    BarroniteDeposits,
    Bronze2hSword,
    IronBar,
    IronDagger,
    IronAxe,
    BronzePlatelegs,
    BronzePlateskirt,
    IronMace,
    IronSpit,
    IronBoltsUnf,
    IronMedHelm,
    BronzePlatebody,
    IronDartTip,
    IronNails,
    IronSword,
    SilverBar,
    IronArrowtips,
    IronHasta,
    IronScimitar,
    IronSpear,
    IronJavelinHeads,
    IronLongsword,
    IronKnife,
    IronFullHelm,
    IronLimbs,
    IronSqShield,
    IronWarhammer,
    IronBattleaxe,
    OilLanternFrame,
    IronChainbody,
    IronKiteshield,
    IronClaws,
    Iron2hSword,
    SteelBar,
    SteelDagger,
    SteelAxe,
    IronPlatelegs,
    IronPlateskirt,
    SteelMace,
    SteelBoltsUnf,
    SteelMedHelm,
    IronPlatebody,
    SteelDartTip,
    SteelNails,
    SteelSword,
    Cannonball,
    SteelArrowtips,
    SteelHasta,
    SteelScimitar,
    SteelSpear,
    SteelJavelinHeads,
    SteelLimbs,
    SteelStuds,
    SteelLongsword,
    SteelKnife,
    SteelFullHelm,
    SteelSqShield,
    SteelWarhammer,
    GoldBar,
    SteelBattleaxe,
    SteelChainbody,
    SteelKiteshield,
    SteelClaws,
    Steel2hSword,
    SteelPlatelegs,
    SteelPlateskirt,
    SteelPlatebody,
    BullseyeLanternUnf,
    MithrilBar,
    MithrilDagger,
    MithrilAxe,
    MithrilMace,
    MithrilBoltsUnf,
    MithrilMedHelm,
    MithrilDartTip,
    MithrilNails,
    MithrilSword,
    MithrilArrowtips,
    MithrilHasta,
    MithrilScimitar,
    MithrilSpear,
    MithrilJavelinHeads,
    MithrilLimbs,
    MithrilLongsword,
    MithrilKnife,
    MithrilFullHelm,
    MithrilSqShield,
    MithGrappleTip,
    MithrilWarhammer,
    DragonSqShield,
    MithrilBattleaxe,
    MithrilChainbody,
    MithrilKiteshield,
    MithrilClaws,
    Mithril2hSword,
    MithrilPlatelegs,
    MithrilPlateskirt,
    MithrilPlatebody,
    AdamantiteBar,
    AdamantDagger,
    AdamantAxe,
    AdamantMace,
    AdamantBoltsUnf,
    AdamantMedHelm,
    AdamantiteNails,
    AdamantDartTip,
    AdamantSword,
    AdamantArrowtips,
    AdamantHasta,
    AdamantScimitar,
    AdamantSpear,
    AdamantiteLimbs,
    AdamantJavelinHeads,
    AdamantLongsword,
    AdamantKnife,
    AdamantFullHelm,
    AdamantSqShield,
    AdamantWarhammer,
    AdamantBattleaxe,
    AdamantChainbody,
    AdamantKiteshield,
    AdamantClaws,
    Adamant2hSword,
    RuniteBar,
    RuneDagger,
    RuneAxe,
    AdamantPlatelegs,
    AdamantPlateskirt,
    RuneMace,
    RuneMedHelm,
    RuniteBoltsUnf,
    AdamantPlatebody,
    RuneDartTip,
    RuneNails,
    RuneSword,
    RuneArrowtips,
    RuneHasta,
    RuneScimitar,
    RuneSpear,
    DragonfireShield,
    RuneJavelinHeads,
    RuniteLimbs,
    RuneLongsword,
    RuneKnife,
    RuneFullHelm,
    RuneSqShield,
    RuneWarhammer,
    RuneBattleaxe,
    RuneChainbody,
    RuneKiteshield,
    RuneClaws,
    Rune2hSword,
    RunePlatelegs,
    RunePlateskirt,
    RunePlatebody,
}

impl Skill for Smithing {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::BronzeBar,
            Self::BronzeAxe,
            Self::BronzeDagger,
            Self::BronzeMace,
            Self::BronzeBoltsUnf,
            Self::BronzeMedHelm,
            Self::BronzeDartTip,
            Self::BronzeNails,
            Self::BronzeSword,
            Self::BronzeWire,
            Self::BronzeArrowtips,
            Self::BronzeHasta,
            Self::BronzeScimitar,
            Self::BronzeSpear,
            Self::BronzeJavelinHeads,
            Self::BronzeLimbs,
            Self::BronzeLongsword,
            Self::BronzeKnife,
            Self::BronzeFullHelm,
            Self::BronzeSqShield,
            Self::BronzeWarhammer,
            Self::BronzeBattleaxe,
            Self::BronzeChainbody,
            Self::BronzeKiteshield,
            Self::BronzeClaws,
            Self::BarroniteDeposits,
            Self::Bronze2hSword,
            Self::IronBar,
            Self::IronDagger,
            Self::IronAxe,
            Self::BronzePlatelegs,
            Self::BronzePlateskirt,
            Self::IronMace,
            Self::IronSpit,
            Self::IronBoltsUnf,
            Self::IronMedHelm,
            Self::BronzePlatebody,
            Self::IronDartTip,
            Self::IronNails,
            Self::IronSword,
            Self::SilverBar,
            Self::IronArrowtips,
            Self::IronHasta,
            Self::IronScimitar,
            Self::IronSpear,
            Self::IronJavelinHeads,
            Self::IronLongsword,
            Self::IronKnife,
            Self::IronFullHelm,
            Self::IronLimbs,
            Self::IronSqShield,
            Self::IronWarhammer,
            Self::IronBattleaxe,
            Self::OilLanternFrame,
            Self::IronChainbody,
            Self::IronKiteshield,
            Self::IronClaws,
            Self::Iron2hSword,
            Self::SteelBar,
            Self::SteelDagger,
            Self::SteelAxe,
            Self::IronPlatelegs,
            Self::IronPlateskirt,
            Self::SteelMace,
            Self::SteelBoltsUnf,
            Self::SteelMedHelm,
            Self::IronPlatebody,
            Self::SteelDartTip,
            Self::SteelNails,
            Self::SteelSword,
            Self::Cannonball,
            Self::SteelArrowtips,
            Self::SteelHasta,
            Self::SteelScimitar,
            Self::SteelSpear,
            Self::SteelJavelinHeads,
            Self::SteelLimbs,
            Self::SteelStuds,
            Self::SteelLongsword,
            Self::SteelKnife,
            Self::SteelFullHelm,
            Self::SteelSqShield,
            Self::SteelWarhammer,
            Self::GoldBar,
            Self::SteelBattleaxe,
            Self::SteelChainbody,
            Self::SteelKiteshield,
            Self::SteelClaws,
            Self::Steel2hSword,
            Self::SteelPlatelegs,
            Self::SteelPlateskirt,
            Self::SteelPlatebody,
            Self::BullseyeLanternUnf,
            Self::MithrilBar,
            Self::MithrilDagger,
            Self::MithrilAxe,
            Self::MithrilMace,
            Self::MithrilBoltsUnf,
            Self::MithrilMedHelm,
            Self::MithrilDartTip,
            Self::MithrilNails,
            Self::MithrilSword,
            Self::MithrilArrowtips,
            Self::MithrilHasta,
            Self::MithrilScimitar,
            Self::MithrilSpear,
            Self::MithrilJavelinHeads,
            Self::MithrilLimbs,
            Self::MithrilLongsword,
            Self::MithrilKnife,
            Self::MithrilFullHelm,
            Self::MithrilSqShield,
            Self::MithGrappleTip,
            Self::MithrilWarhammer,
            Self::DragonSqShield,
            Self::MithrilBattleaxe,
            Self::MithrilChainbody,
            Self::MithrilKiteshield,
            Self::MithrilClaws,
            Self::Mithril2hSword,
            Self::MithrilPlatelegs,
            Self::MithrilPlateskirt,
            Self::MithrilPlatebody,
            Self::AdamantiteBar,
            Self::AdamantDagger,
            Self::AdamantAxe,
            Self::AdamantMace,
            Self::AdamantBoltsUnf,
            Self::AdamantMedHelm,
            Self::AdamantiteNails,
            Self::AdamantDartTip,
            Self::AdamantSword,
            Self::AdamantArrowtips,
            Self::AdamantHasta,
            Self::AdamantScimitar,
            Self::AdamantSpear,
            Self::AdamantiteLimbs,
            Self::AdamantJavelinHeads,
            Self::AdamantLongsword,
            Self::AdamantKnife,
            Self::AdamantFullHelm,
            Self::AdamantSqShield,
            Self::AdamantWarhammer,
            Self::AdamantBattleaxe,
            Self::AdamantChainbody,
            Self::AdamantKiteshield,
            Self::AdamantClaws,
            Self::Adamant2hSword,
            Self::RuniteBar,
            Self::RuneDagger,
            Self::RuneAxe,
            Self::AdamantPlatelegs,
            Self::AdamantPlateskirt,
            Self::RuneMace,
            Self::RuneMedHelm,
            Self::RuniteBoltsUnf,
            Self::AdamantPlatebody,
            Self::RuneDartTip,
            Self::RuneNails,
            Self::RuneSword,
            Self::RuneArrowtips,
            Self::RuneHasta,
            Self::RuneScimitar,
            Self::RuneSpear,
            Self::DragonfireShield,
            Self::RuneJavelinHeads,
            Self::RuniteLimbs,
            Self::RuneLongsword,
            Self::RuneKnife,
            Self::RuneFullHelm,
            Self::RuneSqShield,
            Self::RuneWarhammer,
            Self::RuneBattleaxe,
            Self::RuneChainbody,
            Self::RuneKiteshield,
            Self::RuneClaws,
            Self::Rune2hSword,
            Self::RunePlatelegs,
            Self::RunePlateskirt,
            Self::RunePlatebody,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::Cannonball,
            Self::GoldBar,
            Self::MithrilDartTip,
            Self::MithrilPlatebody,
            Self::AdamantDartTip,
            Self::AdamantPlatebody,
            Self::RunePlatelegs,
            Self::RunePlatebody,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::BronzeBar => ("Bronze Bar", 1, 6.2),
            Self::BronzeAxe => ("Bronze Axe", 1, 12.5),
            Self::BronzeDagger => ("Bronze Dagger", 1, 12.5),
            Self::BronzeMace => ("Bronze Mace", 2, 12.5),
            Self::BronzeBoltsUnf => ("Bronze Bolts Unf", 3, 12.5),
            Self::BronzeMedHelm => ("Bronze Med Helm", 3, 12.5),
            Self::BronzeDartTip => ("Bronze Dart Tip", 4, 12.5),
            Self::BronzeNails => ("Bronze Nails", 4, 12.5),
            Self::BronzeSword => ("Bronze Sword", 4, 12.5),
            Self::BronzeWire => ("Bronze Wire", 4, 12.5),
            Self::BronzeArrowtips => ("Bronze Arrowtips", 5, 12.5),
            Self::BronzeHasta => ("Bronze Hasta", 5, 25.0),
            Self::BronzeScimitar => ("Bronze Scimitar", 5, 25.0),
            Self::BronzeSpear => ("Bronze Spear", 5, 25.0),
            Self::BronzeJavelinHeads => ("Bronze Javelin Heads", 6, 12.5),
            Self::BronzeLimbs => ("Bronze Limbs", 6, 12.5),
            Self::BronzeLongsword => ("Bronze Longsword", 6, 25.0),
            Self::BronzeKnife => ("Bronze Knife", 7, 12.5),
            Self::BronzeFullHelm => ("Bronze Full Helm", 7, 25.0),
            Self::BronzeSqShield => ("Bronze Sq Shield", 8, 25.0),
            Self::BronzeWarhammer => ("Bronze Warhammer", 9, 37.5),
            Self::BronzeBattleaxe => ("Bronze Battleaxe", 10, 37.5),
            Self::BronzeChainbody => ("Bronze Chainbody", 11, 37.5),
            Self::BronzeKiteshield => ("Bronze Kiteshield", 12, 37.5),
            Self::BronzeClaws => ("Bronze Claws", 13, 25.0),
            Self::BarroniteDeposits => ("Barronite Deposit", 14, 30.0),
            Self::Bronze2hSword => ("Bronze 2h Sword", 14, 37.5),
            Self::IronBar => ("Iron Bar", 15, 12.5),
            Self::IronDagger => ("Iron Dagger", 15, 25.0),
            Self::IronAxe => ("Iron Axe", 16, 25.0),
            Self::BronzePlatelegs => ("Bronze Platelegs", 16, 37.5),
            Self::BronzePlateskirt => ("Bronze Plateskirt", 16, 37.5),
            Self::IronMace => ("Iron Mace", 17, 25.0),
            Self::IronSpit => ("Iron Spit", 17, 25.0),
            Self::IronBoltsUnf => ("Iron Bolts Unf", 18, 25.0),
            Self::IronMedHelm => ("Iron Med Helm", 18, 25.0),
            Self::BronzePlatebody => ("Bronze Platebody", 18, 62.5),
            Self::IronDartTip => ("Iron Dart Tip", 19, 25.0),
            Self::IronNails => ("Iron Nails", 19, 25.0),
            Self::IronSword => ("Iron Sword", 19, 25.0),
            Self::SilverBar => ("Silver Bar", 20, 13.7),
            Self::IronArrowtips => ("Iron Arrowtips", 20, 25.0),
            Self::IronHasta => ("Iron Hasta", 20, 50.0),
            Self::IronScimitar => ("Iron Scimitar", 20, 50.0),
            Self::IronSpear => ("Iron Spear", 20, 50.0),
            Self::IronJavelinHeads => ("Iron Javelin Heads", 21, 25.0),
            Self::IronLongsword => ("Iron Longsword", 21, 50.0),
            Self::IronKnife => ("Iron Knife", 22, 25.0),
            Self::IronFullHelm => ("Iron Full Helm", 22, 50.0),
            Self::IronLimbs => ("Iron Limbs", 23, 25.0),
            Self::IronSqShield => ("Iron Sq Shield", 23, 50.0),
            Self::IronWarhammer => ("Iron Warhammer", 24, 75.0),
            Self::IronBattleaxe => ("Iron Battleaxe", 25, 75.0),
            Self::OilLanternFrame => ("Oil Lantern Frame", 26, 25.0),
            Self::IronChainbody => ("Iron Chainbody", 26, 75.0),
            Self::IronKiteshield => ("Iron Kiteshield", 27, 75.0),
            Self::IronClaws => ("Iron Claws", 28, 50.0),
            Self::Iron2hSword => ("Iron 2h Sword", 29, 75.0),
            Self::SteelBar => ("Steel Bar", 30, 17.5),
            Self::SteelDagger => ("Steel Dagger", 30, 37.5),
            Self::SteelAxe => ("Steel Axe", 31, 37.5),
            Self::IronPlatelegs => ("Iron Platelegs", 31, 75.0),
            Self::IronPlateskirt => ("Iron Plateskirt", 31, 75.0),
            Self::SteelMace => ("Steel Mace", 32, 37.5),
            Self::SteelBoltsUnf => ("Steel Bolts Unf", 33, 37.5),
            Self::SteelMedHelm => ("Steel Med Helm", 33, 37.5),
            Self::IronPlatebody => ("Iron Platebody", 33, 125.0),
            Self::SteelDartTip => ("Steel Dart Tip", 34, 37.5),
            Self::SteelNails => ("Steel Nails", 34, 37.5),
            Self::SteelSword => ("Steel Sword", 34, 37.5),
            Self::Cannonball => ("Cannonball", 35, 25.6),
            Self::SteelArrowtips => ("Steel Arrowtips", 35, 37.5),
            Self::SteelHasta => ("Steel Hasta", 35, 75.0),
            Self::SteelScimitar => ("Steel Scimitar", 35, 75.0),
            Self::SteelSpear => ("Steel Spear", 35, 75.0),
            Self::SteelJavelinHeads => ("Steel Javelin Heads", 36, 37.5),
            Self::SteelLimbs => ("Steel Limbs", 36, 37.5),
            Self::SteelStuds => ("Steel Studs", 36, 37.5),
            Self::SteelLongsword => ("Steel Longsword", 36, 75.0),
            Self::SteelKnife => ("Steel Knife", 37, 37.5),
            Self::SteelFullHelm => ("Steel Full Helm", 37, 75.0),
            Self::SteelSqShield => ("Steel Sq Shield", 38, 75.0),
            Self::SteelWarhammer => ("Steel Warhammer", 39, 112.5),
            Self::GoldBar => ("Gold Bar", 40, 22.5),
            Self::SteelBattleaxe => ("Steel Battleaxe", 40, 112.5),
            Self::SteelChainbody => ("Steel Chainbody", 41, 112.5),
            Self::SteelKiteshield => ("Steel Kiteshield", 42, 112.5),
            Self::SteelClaws => ("Steel Claws", 43, 75.0),
            Self::Steel2hSword => ("Steel 2h Sword", 44, 112.5),
            Self::SteelPlatelegs => ("Steel Platelegs", 46, 112.5),
            Self::SteelPlateskirt => ("Steel Plateskirt", 46, 112.5),
            Self::SteelPlatebody => ("Steel Platebody", 48, 187.5),
            Self::BullseyeLanternUnf => ("Bullseye Lantern Unf", 49, 37.0),
            Self::MithrilBar => ("Mithril Bar", 50, 30.0),
            Self::MithrilDagger => ("Mithril Dagger", 50, 50.0),
            Self::MithrilAxe => ("Mithril Axe", 51, 50.0),
            Self::MithrilMace => ("Mithril Mace", 52, 50.0),
            Self::MithrilBoltsUnf => ("Mithril Bolts Unf", 53, 50.0),
            Self::MithrilMedHelm => ("Mithril Med Helm", 53, 50.0),
            Self::MithrilDartTip => ("Mithril Dart Tip", 54, 50.0),
            Self::MithrilNails => ("Mithril Nails", 54, 50.0),
            Self::MithrilSword => ("Mithril Sword", 54, 50.0),
            Self::MithrilArrowtips => ("Mithril Arrowtips", 55, 50.0),
            Self::MithrilHasta => ("Mithril Hasta", 55, 100.0),
            Self::MithrilScimitar => ("Mithril Scimitar", 55, 100.0),
            Self::MithrilSpear => ("Mithril Spear", 55, 100.0),
            Self::MithrilJavelinHeads => ("Mithril Javelin Heads", 56, 50.0),
            Self::MithrilLimbs => ("Mithril Limbs", 56, 50.0),
            Self::MithrilLongsword => ("Mithril Longsword", 56, 100.0),
            Self::MithrilKnife => ("Mithril Knife", 57, 50.0),
            Self::MithrilFullHelm => ("Mithril Full Helm", 57, 100.0),
            Self::MithrilSqShield => ("Mithril Sq Shield", 58, 100.0),
            Self::MithGrappleTip => ("Mith Grapple Tip", 59, 50.0),
            Self::MithrilWarhammer => ("Mithril Warhammer", 59, 150.0),
            Self::DragonSqShield => ("Dragon Sq Shield", 60, 75.0),
            Self::MithrilBattleaxe => ("Mithril Battleaxe", 60, 150.0),
            Self::MithrilChainbody => ("Mithril Chainbody", 61, 150.0),
            Self::MithrilKiteshield => ("Mithril Kiteshield", 62, 150.0),
            Self::MithrilClaws => ("Mithril Claws", 63, 100.0),
            Self::Mithril2hSword => ("Mithril 2h Sword", 64, 150.0),
            Self::MithrilPlatelegs => ("Mithril Platelegs", 66, 150.0),
            Self::MithrilPlateskirt => ("Mithril Plateskirt", 66, 150.0),
            Self::MithrilPlatebody => ("Mithril Platebody", 68, 250.0),
            Self::AdamantiteBar => ("Adamantite Bar", 70, 37.5),
            Self::AdamantDagger => ("Adamant Dagger", 70, 62.5),
            Self::AdamantAxe => ("Adamant Axe", 71, 62.5),
            Self::AdamantMace => ("Adamant Mace", 72, 62.5),
            Self::AdamantBoltsUnf => ("Adamant Boltsunf", 73, 62.5),
            Self::AdamantMedHelm => ("Adamant Med Helm", 73, 62.5),
            Self::AdamantiteNails => ("Adamantite Nails", 74, 62.5),
            Self::AdamantDartTip => ("Adamant Dart Tip", 74, 62.5),
            Self::AdamantSword => ("Adamant Sword", 74, 62.5),
            Self::AdamantArrowtips => ("Adamant Arrowtips", 75, 62.5),
            Self::AdamantHasta => ("Adamant Hasta", 75, 125.0),
            Self::AdamantScimitar => ("Adamant Scimitar", 75, 125.0),
            Self::AdamantSpear => ("Adamant Spear", 75, 125.0),
            Self::AdamantiteLimbs => ("Adamantite Limbs", 76, 62.5),
            Self::AdamantJavelinHeads => ("Adamant Javelin Heads", 76, 62.5),
            Self::AdamantLongsword => ("Adamant Longsword", 76, 125.0),
            Self::AdamantKnife => ("Adamant Knife", 77, 62.5),
            Self::AdamantFullHelm => ("Adamant Full Helm", 77, 125.0),
            Self::AdamantSqShield => ("Adamant Sq Shield", 78, 125.0),
            Self::AdamantWarhammer => ("Adamant Warhammer", 79, 187.5),
            Self::AdamantBattleaxe => ("Adamant Battleaxe", 80, 187.5),
            Self::AdamantChainbody => ("Adamant Chainbody", 81, 187.5),
            Self::AdamantKiteshield => ("Adamant Kiteshield", 82, 187.5),
            Self::AdamantClaws => ("Adamant Claws", 83, 125.0),
            Self::Adamant2hSword => ("Adamant 2h Sword", 84, 187.5),
            Self::RuniteBar => ("Runite Bar", 85, 50.0),
            Self::RuneDagger => ("Rune Dagger", 85, 75.0),
            Self::RuneAxe => ("Rune Axe", 86, 75.0),
            Self::AdamantPlatelegs => ("Adamant Platelegs", 86, 187.5),
            Self::AdamantPlateskirt => ("Adamant Plateskirt", 86, 187.5),
            Self::RuneMace => ("Rune Mace", 87, 75.0),
            Self::RuneMedHelm => ("Rune Med Helm", 88, 75.0),
            Self::RuniteBoltsUnf => ("Runite Bolts Unf", 88, 75.0),
            Self::AdamantPlatebody => ("Adamant Platebody", 88, 312.5),
            Self::RuneDartTip => ("Rune Dart Tip", 89, 75.0),
            Self::RuneNails => ("Rune Nails", 89, 75.0),
            Self::RuneSword => ("Rune Sword", 89, 75.0),
            Self::RuneArrowtips => ("Rune Arrowtips", 90, 75.0),
            Self::RuneHasta => ("Rune Hasta", 90, 150.0),
            Self::RuneScimitar => ("Rune Scimitar", 90, 150.0),
            Self::RuneSpear => ("Rune Spear", 90, 150.0),
            Self::DragonfireShield => ("Dragonfire Shield", 90, 2000.0),
            Self::RuneJavelinHeads => ("Rune Javelin Heads", 91, 75.0),
            Self::RuniteLimbs => ("Runite Limbs", 91, 75.0),
            Self::RuneLongsword => ("Rune Longsword", 91, 150.0),
            Self::RuneKnife => ("Rune Knife", 92, 75.0),
            Self::RuneFullHelm => ("Rune Full Helm", 92, 150.0),
            Self::RuneSqShield => ("Rune Sq Shield", 93, 150.0),
            Self::RuneWarhammer => ("Rune Warhammer", 94, 225.0),
            Self::RuneBattleaxe => ("Rune Battleaxe", 95, 225.0),
            Self::RuneChainbody => ("Rune Chainbody", 96, 225.0),
            Self::RuneKiteshield => ("Rune Kiteshield", 97, 225.0),
            Self::RuneClaws => ("Rune Claws", 98, 150.0),
            Self::Rune2hSword => ("Rune 2h Sword", 99, 225.0),
            Self::RunePlatelegs => ("Rune Platelegs", 99, 225.0),
            Self::RunePlateskirt => ("Rune Plateskirt", 99, 225.0),
            Self::RunePlatebody => ("Rune Platebody", 99, 375.0),
        };

        Details::Smithing(SmithingDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: match self {
                Self::GoldBar => vec![Multipliers::Smithing(
                    SmithingMultipliers::GoldsmithGauntlets,
                )],
                _ => vec![],
            },
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

impl Detail for Smithing {
    fn multipliers(&self) -> Vec<Multipliers> {
        if let Details::Smithing(obj) = self.details() {
            return obj.multipliers;
        }

        vec![]
    }

    fn name(&self) -> String {
        if let Details::Smithing(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Smithing(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Smithing(obj) = self.details() {
            return obj.xp;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct SmithingDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for SmithingDetails {
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

        self.multipliers.iter().for_each(|x| {
            let a = match x {
                Multipliers::Smithing(x) => x,
                _ => return,
            };
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
        });

        vec.join(" ")
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SmithingMultipliers {
    GoldsmithGauntlets,
}

impl SmithingMultipliers {
    pub fn details(&self) -> SmithingMultiplierDetails {
        let details = match self {
            Self::GoldsmithGauntlets => ("Gauntlets", 2.5),
        };

        SmithingMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct SmithingMultiplierDetails {
    pub name: String,
    pub value: f64,
}
