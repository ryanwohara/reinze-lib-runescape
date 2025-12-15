use crate::stats::construction::ConstructionMultipliers::CarpentersOutfit;
use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};
use common::{c1, c2, p};
use regex::Regex;
use std::ops::Add;

pub enum Construction {
    Plank,
    DockLeaf,
    FernBigPlant,
    Plant,
    ShortPlant,
    CrudeWoodenChair,
    ExitPortal,
    MahoganyHomesBeginner,
    BrownRug,
    TornCurtains,
    ClayFireplace,
    WoodenBookcase,
    CatBlanket,
    Tree,
    Firepit,
    DecorativeRock,
    Bush,
    LargeLeafBush,
    SmallFern,
    Thistle,
    WoodenShelves1,
    BeerBarrel,
    PumpAndDrain,
    WoodenChair,
    WoodenLarder,
    NiceTree,
    Pond,
    WoodenBench,
    WoodDiningTable,
    FirepitWithHook,
    WoodTable,
    CiderBarrel,
    FernSmallPlant,
    HugePlant,
    Reeds,
    TallPlant,
    WoodenShelves2,
    Rug,
    RockingChair,
    OakPlank,
    OakTree,
    ImpStatue,
    OakDecoration,
    FirepitWithPot,
    AsgarnianAle,
    Curtains,
    CatBasket,
    OakChair,
    ShoeBox,
    WoodenBed,
    MahoganyHomesNovice,
    ShavingStand,
    OakBench,
    OakDiningTable,
    WoodenShelves3,
    SmallOven,
    OakClock,
    RopeBellPull,
    OakArmchair,
    GreenmansAle,
    OakDrawers,
    PumpAndTub,
    DeadmanRug,
    OakShavingStand,
    LargeOven,
    OakBookcase,
    WillowTree,
    OakBed,
    LongBone,
    CurvedBone,
    CarvedOakBench,
    CarvedOakTable,
    OakKitchenTable,
    BoxingRing,
    StoneFireplace,
    CushionedBasket,
    OakLarder,
    GloveRack,
    SteelRange,
    OakShelves1,
    LargeOakBed,
    TeakPlank,
    TeakArmchair,
    TeakDecoration,
    DragonBitter,
    BellPull,
    OakDresser,
    TeakBench,
    TeakTable,
    OakWardrobe,
    OakLectern,
    MahoganyPlank,
    TeakBed,
    OpulentCurtains,
    MahoganyBookcase,
    Globe,
    FencingRing,
    FancyRange,
    CrystalBall,
    AlchemicalChart,
    TeakLarder,
    WoodenTelescope,
    WeaponsRack,
    CarvedTeakBench,
    MapleTree,
    OakShelves2,
    LargeTeakBed,
    CarvedTeakTable,
    TeakDresser,
    DemonLectern,
    EagleLectern,
    Sink,
    MountedMythicalCape,
    ChefsDelight,
    TeleportFocus,
    OrnamentalGlobe,
    TeakPortal,
    MahoganyArmchair,
    MahoganyHomesAdept,
    TeakDrawers,
    CombatRing,
    TeakKitchenTable,
    MahoganyBench,
    MahoganyTable,
    FourPosterBed,
    ExtraWeaponsRack,
    ElementalSphere,
    TeakClock,
    FancyTeakDresser,
    TeakShelves1,
    GildedDecoration,
    TeakDemonLectern,
    TeakEagleLectern,
    LimestoneAttackStone,
    LunarGlobe,
    YewTree,
    SpiceRack,
    PoshBellPull,
    GildedFourPosterBed,
    GildedBench,
    AstronomicalChart,
    TeakWardrobe,
    MarbleFireplace,
    TeakTelescope,
    MahoganyDresser,
    OpulentRug,
    MahoganyPortal,
    GreaterFocus,
    TeakGardenBench,
    CrystalOfPower,
    MahoganyDemonLectern,
    MahoganyEagleLectern,
    TeakShelves2,
    CelestialGlobe,
    DungeonEntrance,
    MahoganyHomesExpert,
    RangingPedestals,
    OpulentTable,
    GildedDresser,
    OakDoor,
    MagicTree,
    MahoganyWardrobe,
    GnomeBench,
    ArmillaryGlobe,
    MarblePortal,
    ScryingPool,
    BalanceBeam,
    InfernalChart,
    MahoganyTelescope,
    GildedClock,
    SmallOrrery,
    GildedWardrobe,
    LargeOrrery,
}

impl Skill for Construction {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Plank,
            Self::DockLeaf,
            Self::FernBigPlant,
            Self::Plant,
            Self::ShortPlant,
            Self::CrudeWoodenChair,
            Self::ExitPortal,
            Self::MahoganyHomesBeginner,
            Self::BrownRug,
            Self::TornCurtains,
            Self::ClayFireplace,
            Self::WoodenBookcase,
            Self::CatBlanket,
            Self::Tree,
            Self::Firepit,
            Self::DecorativeRock,
            Self::Bush,
            Self::LargeLeafBush,
            Self::SmallFern,
            Self::Thistle,
            Self::WoodenShelves1,
            Self::BeerBarrel,
            Self::PumpAndDrain,
            Self::WoodenChair,
            Self::WoodenLarder,
            Self::NiceTree,
            Self::Pond,
            Self::WoodenBench,
            Self::WoodDiningTable,
            Self::FirepitWithHook,
            Self::WoodTable,
            Self::CiderBarrel,
            Self::FernSmallPlant,
            Self::HugePlant,
            Self::Reeds,
            Self::TallPlant,
            Self::WoodenShelves2,
            Self::Rug,
            Self::RockingChair,
            Self::OakPlank,
            Self::OakTree,
            Self::ImpStatue,
            Self::OakDecoration,
            Self::FirepitWithPot,
            Self::AsgarnianAle,
            Self::Curtains,
            Self::CatBasket,
            Self::OakChair,
            Self::ShoeBox,
            Self::WoodenBed,
            Self::MahoganyHomesNovice,
            Self::ShavingStand,
            Self::OakBench,
            Self::OakDiningTable,
            Self::WoodenShelves3,
            Self::SmallOven,
            Self::OakClock,
            Self::RopeBellPull,
            Self::OakArmchair,
            Self::GreenmansAle,
            Self::OakDrawers,
            Self::PumpAndTub,
            Self::DeadmanRug,
            Self::OakShavingStand,
            Self::LargeOven,
            Self::OakBookcase,
            Self::WillowTree,
            Self::OakBed,
            Self::LongBone,
            Self::CurvedBone,
            Self::CarvedOakBench,
            Self::CarvedOakTable,
            Self::OakKitchenTable,
            Self::BoxingRing,
            Self::StoneFireplace,
            Self::CushionedBasket,
            Self::OakLarder,
            Self::GloveRack,
            Self::SteelRange,
            Self::OakShelves1,
            Self::LargeOakBed,
            Self::TeakPlank,
            Self::TeakArmchair,
            Self::TeakDecoration,
            Self::DragonBitter,
            Self::BellPull,
            Self::OakDresser,
            Self::TeakBench,
            Self::TeakTable,
            Self::OakWardrobe,
            Self::OakLectern,
            Self::MahoganyPlank,
            Self::TeakBed,
            Self::OpulentCurtains,
            Self::MahoganyBookcase,
            Self::Globe,
            Self::FencingRing,
            Self::FancyRange,
            Self::CrystalBall,
            Self::AlchemicalChart,
            Self::TeakLarder,
            Self::WoodenTelescope,
            Self::WeaponsRack,
            Self::CarvedTeakBench,
            Self::MapleTree,
            Self::OakShelves2,
            Self::LargeTeakBed,
            Self::CarvedTeakTable,
            Self::TeakDresser,
            Self::DemonLectern,
            Self::EagleLectern,
            Self::Sink,
            Self::MountedMythicalCape,
            Self::ChefsDelight,
            Self::TeleportFocus,
            Self::OrnamentalGlobe,
            Self::TeakPortal,
            Self::MahoganyArmchair,
            Self::MahoganyHomesAdept,
            Self::TeakDrawers,
            Self::CombatRing,
            Self::TeakKitchenTable,
            Self::MahoganyBench,
            Self::MahoganyTable,
            Self::FourPosterBed,
            Self::ExtraWeaponsRack,
            Self::ElementalSphere,
            Self::TeakClock,
            Self::FancyTeakDresser,
            Self::TeakShelves1,
            Self::GildedDecoration,
            Self::TeakDemonLectern,
            Self::TeakEagleLectern,
            Self::LimestoneAttackStone,
            Self::LunarGlobe,
            Self::YewTree,
            Self::SpiceRack,
            Self::PoshBellPull,
            Self::GildedFourPosterBed,
            Self::GildedBench,
            Self::AstronomicalChart,
            Self::TeakWardrobe,
            Self::MarbleFireplace,
            Self::TeakTelescope,
            Self::MahoganyDresser,
            Self::OpulentRug,
            Self::MahoganyPortal,
            Self::GreaterFocus,
            Self::TeakGardenBench,
            Self::CrystalOfPower,
            Self::MahoganyDemonLectern,
            Self::MahoganyEagleLectern,
            Self::TeakShelves2,
            Self::CelestialGlobe,
            Self::DungeonEntrance,
            Self::MahoganyHomesExpert,
            Self::RangingPedestals,
            Self::OpulentTable,
            Self::GildedDresser,
            Self::OakDoor,
            Self::MagicTree,
            Self::MahoganyWardrobe,
            Self::GnomeBench,
            Self::ArmillaryGlobe,
            Self::MarblePortal,
            Self::ScryingPool,
            Self::BalanceBeam,
            Self::InfernalChart,
            Self::MahoganyTelescope,
            Self::GildedClock,
            Self::SmallOrrery,
            Self::GildedWardrobe,
            Self::LargeOrrery,
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::Plank,
            Self::OakPlank,
            Self::TeakPlank,
            Self::MahoganyPlank,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match self {
            Self::Plank => ("Plank", 1, 29),
            Self::DockLeaf => ("Dock Leaf", 1, 31),
            Self::FernBigPlant => ("Fern (big plant)", 1, 31),
            Self::Plant => ("Plant", 1, 31),
            Self::ShortPlant => ("Short Plant", 1, 31),
            Self::CrudeWoodenChair => ("Crude Wooden Chair", 1, 58),
            Self::ExitPortal => ("Exit Portal", 1, 100),
            Self::MahoganyHomesBeginner => ("Mahogany Homes Beginner", 1, 500),
            Self::BrownRug => ("Brown Rug", 2, 30),
            Self::TornCurtains => ("Torn Curtains", 2, 132),
            Self::ClayFireplace => ("Clay Fireplace", 3, 30),
            Self::WoodenBookcase => ("Wooden Bookcase", 4, 115),
            Self::CatBlanket => ("Cat Blanket", 5, 15),
            Self::Tree => ("Tree", 5, 31),
            Self::Firepit => ("Firepit", 5, 40),
            Self::DecorativeRock => ("Decorative Rock", 5, 100),
            Self::Bush => ("Bush", 6, 70),
            Self::LargeLeafBush => ("Large Leaf Bush", 6, 70),
            Self::SmallFern => ("Small Fern", 6, 70),
            Self::Thistle => ("Thistle", 6, 70),
            Self::WoodenShelves1 => ("Wooden Shelves 1", 6, 87),
            Self::BeerBarrel => ("Beer Barrel", 7, 87),
            Self::PumpAndDrain => ("Pump and Drain", 7, 100),
            Self::WoodenChair => ("Wooden Chair", 8, 87),
            Self::WoodenLarder => ("Wooden Larder", 9, 228),
            Self::NiceTree => ("Nice Tree", 10, 44),
            Self::Pond => ("Pond", 10, 100),
            Self::WoodenBench => ("Wooden Bench", 10, 115),
            Self::WoodDiningTable => ("Wood Dining Table", 10, 115),
            Self::FirepitWithHook => ("Firepit with Hook", 11, 60),
            Self::WoodTable => ("Wood Table", 12, 87),
            Self::CiderBarrel => ("Cider Barrel", 12, 91),
            Self::FernSmallPlant => ("Fern (small plant)", 12, 100),
            Self::HugePlant => ("Huge Plant", 12, 100),
            Self::Reeds => ("Reeds", 12, 100),
            Self::TallPlant => ("Tall Plant", 12, 100),
            Self::WoodenShelves2 => ("Wooden Shelves 2", 12, 147),
            Self::Rug => ("Rug", 13, 60),
            Self::RockingChair => ("Rocking Chair", 14, 87),
            Self::OakPlank => ("Oak Plank", 15, 60),
            Self::OakTree => ("Oak Tree", 15, 70),
            Self::ImpStatue => ("Imp Statue", 15, 150),
            Self::OakDecoration => ("Oak Decoration", 16, 120),
            Self::FirepitWithPot => ("Firepit with Pot", 17, 80),
            Self::AsgarnianAle => ("Asgarnian Ale", 18, 184),
            Self::Curtains => ("Curtains", 18, 225),
            Self::CatBasket => ("Cat Basket", 19, 58),
            Self::OakChair => ("Oak Chair", 19, 120),
            Self::ShoeBox => ("Shoe Box", 20, 58),
            Self::WoodenBed => ("Wooden Bed", 20, 117),
            Self::MahoganyHomesNovice => ("Mahogany Homes Novice", 1, 1250),
            Self::ShavingStand => ("Shaving Stand", 21, 30),
            Self::OakBench => ("Oak Bench", 22, 240),
            Self::OakDiningTable => ("Oak Dining Table", 22, 240),
            Self::WoodenShelves3 => ("Wooden Shelves 3", 23, 147),
            Self::SmallOven => ("Small Oven", 24, 80),
            Self::OakClock => ("Oak Clock", 25, 142),
            Self::RopeBellPull => ("Rope Bell-Pull", 26, 64),
            Self::OakArmchair => ("Oak Armchair", 26, 180),
            Self::GreenmansAle => ("Greenman's Ale", 26, 184),
            Self::OakDrawers => ("Oak Drawers", 27, 120),
            Self::PumpAndTub => ("Pump and Tub", 27, 200),
            Self::DeadmanRug => ("Deadman rug", 28, 283),
            Self::OakShavingStand => ("Oak Shaving Stand", 29, 61),
            Self::LargeOven => ("Large Oven", 29, 100),
            Self::OakBookcase => ("Oak Bookcase", 29, 180),
            Self::WillowTree => ("Willow Tree", 30, 100),
            Self::OakBed => ("Oak Bed", 30, 210),
            Self::LongBone => ("Long Bone", 30, 4500),
            Self::CurvedBone => ("Curved Bone", 30, 6750),
            Self::CarvedOakBench => ("Carved Oak Bench", 31, 240),
            Self::CarvedOakTable => ("Carved Oak Table", 31, 360),
            Self::OakKitchenTable => ("Oak Kitchen Table", 32, 180),
            Self::BoxingRing => ("Boxing Ring", 32, 420),
            Self::StoneFireplace => ("Stone Fireplace", 33, 40),
            Self::CushionedBasket => ("Cushioned Basket", 33, 58),
            Self::OakLarder => ("Oak Larder", 33, 480),
            Self::GloveRack => ("Glove Rack", 34, 120),
            Self::SteelRange => ("Steel Range", 34, 120),
            Self::OakShelves1 => ("Oak Shelves 1", 34, 240),
            Self::LargeOakBed => ("Large Oak Bed", 34, 330),
            Self::TeakPlank => ("Teak Plank", 35, 90),
            Self::TeakArmchair => ("Teak Armchair", 35, 180),
            Self::TeakDecoration => ("Teak Decoration", 36, 180),
            Self::DragonBitter => ("Dragon Bitter", 36, 224),
            Self::BellPull => ("Bell-Pull", 37, 120),
            Self::OakDresser => ("Oak Dresser", 37, 121),
            Self::TeakBench => ("Teak Bench", 38, 360),
            Self::TeakTable => ("Teak Table", 38, 360),
            Self::OakWardrobe => ("Oak Wardrobe", 39, 180),
            Self::OakLectern => ("Oak Lectern", 40, 60),
            Self::MahoganyPlank => ("Mahogany Plank", 40, 140),
            Self::TeakBed => ("Teak Bed", 40, 300),
            Self::OpulentCurtains => ("Opulent Curtains", 40, 315),
            Self::MahoganyBookcase => ("Mahogany Bookcase", 40, 420),
            Self::Globe => ("Globe", 41, 180),
            Self::FencingRing => ("Fencing Ring", 41, 570),
            Self::FancyRange => ("Fancy Range", 42, 160),
            Self::CrystalBall => ("Crystal Ball", 42, 280),
            Self::AlchemicalChart => ("Alchemical Chart", 43, 30),
            Self::TeakLarder => ("Teak larder", 43, 750),
            Self::WoodenTelescope => ("Wooden Telescope", 44, 121),
            Self::WeaponsRack => ("Weapons Rack", 44, 180),
            Self::CarvedTeakBench => ("Carved Teak Bench", 44, 360),
            Self::MapleTree => ("Maple Tree", 45, 122),
            Self::OakShelves2 => ("Oak Shelves 2", 45, 240),
            Self::LargeTeakBed => ("Large Teak Bed", 45, 480),
            Self::CarvedTeakTable => ("Carved Teak Table", 45, 600),
            Self::TeakDresser => ("Teak Dresser", 46, 181),
            Self::DemonLectern => ("Demon Lectern", 47, 120),
            Self::EagleLectern => ("Eagle Lectern", 47, 120),
            Self::Sink => ("Sink", 47, 300),
            Self::MountedMythicalCape => ("Mounted Mythical Cape", 47, 370),
            Self::ChefsDelight => ("Chef's Delight", 48, 224),
            Self::TeleportFocus => ("Teleport Focus", 50, 40),
            Self::OrnamentalGlobe => ("Ornamental Globe", 50, 270),
            Self::TeakPortal => ("Teak Portal", 50, 270),
            Self::MahoganyArmchair => ("Mahogany Armchair", 50, 280),
            Self::MahoganyHomesAdept => ("Mahogany Homes Adept", 1, 2250),
            Self::TeakDrawers => ("Teak Drawers", 51, 180),
            Self::CombatRing => ("Combat Ring", 51, 630),
            Self::TeakKitchenTable => ("Teak Kitchen Table", 52, 270),
            Self::MahoganyBench => ("Mahogany Bench", 52, 560),
            Self::MahoganyTable => ("Mahogany Table", 52, 840),
            Self::FourPosterBed => ("4-Poster Bed", 53, 450),
            Self::ExtraWeaponsRack => ("Extra Weapons Rack", 54, 440),
            Self::ElementalSphere => ("Elemental Sphere", 54, 580),
            Self::TeakClock => ("Teak Clock", 55, 202),
            Self::FancyTeakDresser => ("Fancy Teak Dresser", 56, 182),
            Self::TeakShelves1 => ("Teak Shelves 1", 56, 330),
            Self::GildedDecoration => ("Gilded Decoration", 56, 1020),
            Self::TeakDemonLectern => ("Teak Demon Lectern", 57, 180),
            Self::TeakEagleLectern => ("Teak Eagle Lectern", 57, 180),
            Self::LimestoneAttackStone => ("Limestone attack stone", 59, 200),
            Self::LunarGlobe => ("Lunar Globe", 59, 570),
            Self::YewTree => ("Yew Tree", 60, 141),
            Self::SpiceRack => ("Spice Rack", 60, 374),
            Self::PoshBellPull => ("Posh Bell-Pull", 60, 420),
            Self::GildedFourPosterBed => ("Gilded 4-Poster Bed", 60, 1330),
            Self::GildedBench => ("Gilded Bench", 61, 1760),
            Self::AstronomicalChart => ("Astronomical Chart", 63, 45),
            Self::TeakWardrobe => ("Teak Wardrobe", 63, 270),
            Self::MarbleFireplace => ("Marble Fireplace", 63, 500),
            Self::TeakTelescope => ("Teak Telescope", 64, 181),
            Self::MahoganyDresser => ("Mahogany Dresser", 64, 281),
            Self::OpulentRug => ("Opulent Rug", 65, 360),
            Self::MahoganyPortal => ("Mahogany Portal", 65, 420),
            Self::GreaterFocus => ("Greater Focus", 65, 500),
            Self::TeakGardenBench => ("Teak Garden Bench", 66, 540),
            Self::CrystalOfPower => ("Crystal of Power", 66, 890),
            Self::MahoganyDemonLectern => ("Mahogany Demon Lectern", 67, 580),
            Self::MahoganyEagleLectern => ("Mahogany Eagle Lectern", 67, 580),
            Self::TeakShelves2 => ("Teak Shelves 2", 67, 930),
            Self::CelestialGlobe => ("Celestial Globe", 68, 570),
            Self::DungeonEntrance => ("Dungeon Entrance", 70, 500),
            Self::MahoganyHomesExpert => ("Mahogany Homes Expert", 1, 2750),
            Self::RangingPedestals => ("Ranging Pedestals", 71, 720),
            Self::OpulentTable => ("Opulent Table", 72, 3100),
            Self::GildedDresser => ("Gilded Dresser", 74, 582),
            Self::OakDoor => ("Oak Door", 74, 600),
            Self::MagicTree => ("Magic Tree", 75, 223),
            Self::MahoganyWardrobe => ("Mahogany Wardrobe", 75, 420),
            Self::GnomeBench => ("Gnome Bench", 77, 840),
            Self::ArmillaryGlobe => ("Armillary Globe", 77, 960),
            Self::MarblePortal => ("Marble Portal", 80, 1500),
            Self::ScryingPool => ("Scrying Pool", 80, 2000),
            Self::BalanceBeam => ("Balance Beam", 81, 1000),
            Self::InfernalChart => ("Infernal Chart", 83, 60),
            Self::MahoganyTelescope => ("Mahogany Telescope", 84, 281),
            Self::GildedClock => ("Gilded Clock", 85, 602),
            Self::SmallOrrery => ("Small Orrery", 86, 1320),
            Self::GildedWardrobe => ("Gilded Wardrobe", 87, 720),
            Self::LargeOrrery => ("Large Orrery", 95, 1420),
        };

        Details::Construction(ConstructionDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: vec![Multipliers::Construction(CarpentersOutfit)],
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

impl Detail for Construction {
    fn multipliers(&self) -> Vec<Multipliers> {
        if let Details::Construction(obj) = self.details() {
            return obj.multipliers;
        }

        vec![]
    }

    fn name(&self) -> String {
        if let Details::Construction(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Construction(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Construction(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct ConstructionDetails {
    pub name: String,
    pub level: u32,
    pub xp: u32,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for ConstructionDetails {
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
                Multipliers::Construction(x) => x,
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
pub enum ConstructionMultipliers {
    CarpentersOutfit,
}

impl ConstructionMultipliers {
    pub fn details(&self) -> ConstructionMultiplierDetails {
        let details = match self {
            Self::CarpentersOutfit => ("Outfit", 1.025),
        };

        ConstructionMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct ConstructionMultiplierDetails {
    pub name: String,
    pub value: f64,
}
