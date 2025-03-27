use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use common::{c1, c2, p};
use regex::Regex;

pub enum Hunter {
    PolarKebbit,
    CrimsonSwift,
    CommonKebbit,
    GoldenWarbler,
    RegularBirdHouse,
    FeldipWeasel,
    CopperLongtail,
    CeruleanTwitch,
    DesertDevil,
    OakBirdHouse,
    RubyHarvest,
    BabyImpling,
    TropicalWagtail,
    MossLizard,
    YoungImpling,
    WildKebbit,
    WillowBirdHouse,
    SapphireGlacialis,
    Ferret,
    WhiteRabbit,
    GourmetImpling,
    SwampLizard,
    SpinedLarupia,
    BarbTailedKebbit,
    TeakBirdHouse,
    SnowyKnight,
    EarthImpling,
    PricklyKebbit,
    EmbertailedJerboa,
    HornedGraahk,
    EssenceImpling,
    SpottedKebbit,
    MapleBirdHouse,
    BlackWarlock,
    OrangeSalamander,
    RazorBackedKebbit,
    MahoganyBirdHouse,
    EclecticImpling,
    SabreToothedKebbit,
    Chinchompa,
    SabreToothedKyatt,
    DarkKebbit,
    PyreFox,
    NatureImpling,
    RedSalamander,
    YewBirdHouse,
    ManiacalMonkey,
    CarnivorousChinchompa,
    MagpieImplingPuroPuro,
    SunlightMoth,
    MagpieImplingGielinor,
    BlackSalamander,
    DashingKebbit,
    SunlightAntelope,
    BlackChinchompa,
    NinjaImplingPuroPuro,
    NinjaImplingGielinor,
    MagicBirdHouse,
    MoonlightMoth,
    TecuSalamander,
    CrystalImpling,
    DragonImplingPuroPuro,
    DragonImplingGielinor,
    LuckyImpling,
    RedwoodBirdHouse,
    MoonlightAntelope,
}

impl Detail for Hunter {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Hunter(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Hunter(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Hunter(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Hunter {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::PolarKebbit,
            Self::CrimsonSwift,
            Self::CommonKebbit,
            Self::GoldenWarbler,
            Self::RegularBirdHouse,
            Self::FeldipWeasel,
            Self::CopperLongtail,
            Self::CeruleanTwitch,
            Self::DesertDevil,
            Self::OakBirdHouse,
            Self::RubyHarvest,
            Self::BabyImpling,
            Self::TropicalWagtail,
            Self::MossLizard,
            Self::YoungImpling,
            Self::WildKebbit,
            Self::WillowBirdHouse,
            Self::SapphireGlacialis,
            Self::Ferret,
            Self::WhiteRabbit,
            Self::GourmetImpling,
            Self::SwampLizard,
            Self::SpinedLarupia,
            Self::BarbTailedKebbit,
            Self::TeakBirdHouse,
            Self::SnowyKnight,
            Self::EarthImpling,
            Self::PricklyKebbit,
            Self::EmbertailedJerboa,
            Self::HornedGraahk,
            Self::EssenceImpling,
            Self::SpottedKebbit,
            Self::MapleBirdHouse,
            Self::BlackWarlock,
            Self::OrangeSalamander,
            Self::RazorBackedKebbit,
            Self::MahoganyBirdHouse,
            Self::EclecticImpling,
            Self::SabreToothedKebbit,
            Self::Chinchompa,
            Self::SabreToothedKyatt,
            Self::DarkKebbit,
            Self::PyreFox,
            Self::NatureImpling,
            Self::RedSalamander,
            Self::YewBirdHouse,
            Self::ManiacalMonkey,
            Self::CarnivorousChinchompa,
            Self::MagpieImplingPuroPuro,
            Self::SunlightMoth,
            Self::MagpieImplingGielinor,
            Self::BlackSalamander,
            Self::DashingKebbit,
            Self::SunlightAntelope,
            Self::BlackChinchompa,
            Self::NinjaImplingPuroPuro,
            Self::NinjaImplingGielinor,
            Self::MagicBirdHouse,
            Self::MoonlightMoth,
            Self::TecuSalamander,
            Self::CrystalImpling,
            Self::DragonImplingPuroPuro,
            Self::DragonImplingGielinor,
            Self::LuckyImpling,
            Self::RedwoodBirdHouse,
            Self::MoonlightAntelope,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::SwampLizard,
            Self::OrangeSalamander,
            Self::RedSalamander,
            Self::BlackSalamander,
            Self::Chinchompa,
            Self::CarnivorousChinchompa,
            Self::BlackChinchompa,
            Self::YewBirdHouse,
            Self::MagicBirdHouse,
            Self::RedwoodBirdHouse,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::PolarKebbit => ("Polar Kebbit", 1, 30.0),
            Self::CrimsonSwift => ("Crimson Swift", 1, 34.0),
            Self::CommonKebbit => ("Common Kebbit", 3, 36.0),
            Self::GoldenWarbler => ("Golden Warbler", 5, 47.0),
            Self::RegularBirdHouse => ("Regular Bird House", 5, 280.0),
            Self::FeldipWeasel => ("Feldip Weasel", 7, 48.0),
            Self::CopperLongtail => ("Copper Longtail", 9, 61.0),
            Self::CeruleanTwitch => ("Cerulean Twitch", 11, 64.5),
            Self::DesertDevil => ("Desert Devil", 13, 66.0),
            Self::OakBirdHouse => ("Oak Bird House", 14, 420.0),
            Self::RubyHarvest => ("Ruby Harvest", 15, 24.0),
            Self::BabyImpling => ("Baby Impling", 17, 18.0),
            Self::TropicalWagtail => ("Tropical Wagtail", 19, 95.0),
            Self::MossLizard => ("Moss Lizard", 20, 90.0),
            Self::YoungImpling => ("Young Impling", 22, 20.0),
            Self::WildKebbit => ("Wild Kebbit", 23, 128.0),
            Self::WillowBirdHouse => ("Willow Bird House", 24, 560.0),
            Self::SapphireGlacialis => ("Sapphire Glacialis", 25, 34.0),
            Self::Ferret => ("Ferret", 27, 115.0),
            Self::WhiteRabbit => ("White Rabbit", 27, 144.0),
            Self::GourmetImpling => ("Gourmet Impling", 28, 22.0),
            Self::SwampLizard => ("Swamp Lizard", 29, 152.0),
            Self::SpinedLarupia => ("Spined Larupia", 31, 180.0),
            Self::BarbTailedKebbit => ("Barb-tailed Kebbit", 33, 168.0),
            Self::TeakBirdHouse => ("Teak Bird House", 34, 700.0),
            Self::SnowyKnight => ("Snowy Knight", 35, 44.0),
            Self::EarthImpling => ("Earth Impling", 36, 25.0),
            Self::PricklyKebbit => ("Prickly Kebbit", 37, 204.0),
            Self::EmbertailedJerboa => ("Embertailed Jerboa", 39, 137.0),
            Self::HornedGraahk => ("Horned Graahk", 41, 240.0),
            Self::EssenceImpling => ("Essence Impling", 42, 27.0),
            Self::SpottedKebbit => ("Spotted Kebbit", 43, 104.0),
            Self::MapleBirdHouse => ("Maple Bird House", 44, 820.0),
            Self::BlackWarlock => ("Black Warlock", 45, 54.0),
            Self::OrangeSalamander => ("Orange Salamander", 47, 224.0),
            Self::RazorBackedKebbit => ("Razor-backed Kebbit", 49, 348.0),
            Self::MahoganyBirdHouse => ("Mahogany Bird House", 49, 960.0),
            Self::EclecticImpling => ("Eclectic Impling", 50, 32.0),
            Self::SabreToothedKebbit => ("Sabre-toothed Kebbit", 51, 200.0),
            Self::Chinchompa => ("Chinchompa", 53, 198.4),
            Self::SabreToothedKyatt => ("Sabre-toothed Kyatt", 55, 300.0),
            Self::DarkKebbit => ("Dark Kebbit", 57, 132.0),
            Self::PyreFox => ("Pyre Fox", 57, 222.0),
            Self::NatureImpling => ("Nature Impling", 58, 34.0),
            Self::RedSalamander => ("Red Salamander", 59, 272.0),
            Self::YewBirdHouse => ("Yew Bird House", 59, 1020.0),
            Self::ManiacalMonkey => ("Maniacal Monkey", 60, 1000.0),
            Self::CarnivorousChinchompa => ("Carnivorous Chinchompa", 63, 265.0),
            Self::MagpieImplingPuroPuro => ("Magpie Impling (Puro Puro)", 65, 44.0),
            Self::SunlightMoth => ("Sunlight Moth", 65, 74.0),
            Self::MagpieImplingGielinor => ("Magpie Impling (Gielinor)", 65, 216.0),
            Self::BlackSalamander => ("Black Salamander", 67, 319.5),
            Self::DashingKebbit => ("Dashing Kebbit", 69, 156.0),
            Self::SunlightAntelope => ("Sunlight Antelope", 72, 380.0),
            Self::BlackChinchompa => ("Black Chinchompa", 73, 315.0),
            Self::NinjaImplingPuroPuro => ("Ninja Impling (Puro Puro)", 74, 52.0),
            Self::NinjaImplingGielinor => ("Ninja Impling (Gielinor)", 74, 240.0),
            Self::MagicBirdHouse => ("Magic Bird House", 74, 1140.0),
            Self::MoonlightMoth => ("Moonlight Moth", 75, 84.0),
            Self::TecuSalamander => ("Tecu Salamander", 79, 344.0),
            Self::CrystalImpling => ("Crystal Impling", 80, 280.0),
            Self::DragonImplingPuroPuro => ("Dragon Impling (Puro Puro)", 83, 65.0),
            Self::DragonImplingGielinor => ("Dragon Impling (Gielinor)", 83, 300.0),
            Self::LuckyImpling => ("Lucky Impling", 89, 380.0),
            Self::RedwoodBirdHouse => ("Redwood Bird House", 89, 1200.0),
            Self::MoonlightAntelope => ("Moonlight Antelope", 91, 450.0),
        };

        Details::Hunter(HunterDetails {
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
            all.retain(|activity| {
                pattern
                    .captures(activity.name().to_lowercase().as_str())
                    .iter()
                    .count()
                    > 0
            });
        }

        all
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct HunterDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
}

impl IntoString for HunterDetails {
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
