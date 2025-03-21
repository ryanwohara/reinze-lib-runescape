use common::{c1, c2, p};
use crate::stats::skill::{Detail, Details, IntoString, Multipliers, Skill};

pub enum Construction {
    Plank,
    DockLeaf,
    FernBigPlant,
    Plant,
    ShortPlant,
    CrudeWoodenChair,
    ExitPortal,
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
        Self: Sized
    {
        todo!()
    }

    fn defaults() -> Vec<Details> {
        todo!()
    }

    fn details(&self) -> Details {
        let details = match (self) {
            Construction::Plank => ("Plank", 1, 29),
            Construction::DockLeaf => ("Dock Leaf", 1, 31),
            Construction::FernBigPlant => ("Fern (big plant)", 1, 31),
            Construction::Plant => ("Plant", 1, 31),
            Construction::ShortPlant => ("Short Plant", 1, 31),
            Construction::CrudeWoodenChair => ("Crude Wooden Chair", 1, 58),
            Construction::ExitPortal => ("Exit Portal", 1, 100),
            Construction::BrownRug => ("Brown Rug", 2, 30),
            Construction::TornCurtains => ("Torn Curtains", 2, 132),
            Construction::ClayFireplace => ("Clay Fireplace", 3, 30),
            Construction::WoodenBookcase => ("Wooden Bookcase", 4, 115),
            Construction::CatBlanket => ("Cat Blanket", 5, 15),
            Construction::Tree => ("Tree", 5, 31),
            Construction::Firepit => ("Firepit", 5, 40),
            Construction::DecorativeRock => ("Decorative Rock", 5, 100),
            Construction::Bush => ("Bush", 6, 70),
            Construction::LargeLeafBush => ("Large Leaf Bush", 6, 70),
            Construction::SmallFern => ("Small Fern", 6, 70),
            Construction::Thistle => ("Thistle", 6, 70),
            Construction::WoodenShelves1 => ("Wooden Shelves 1", 6, 87),
            Construction::BeerBarrel => ("Beer Barrel", 7, 87),
            Construction::PumpAndDrain => ("Pump and Drain", 7, 100),
            Construction::WoodenChair => ("Wooden Chair", 8, 87),
            Construction::WoodenLarder => ("Wooden Larder", 9, 228),
            Construction::NiceTree => ("Nice Tree", 10, 44),
            Construction::Pond => ("Pond", 10, 100),
            Construction::WoodenBench => ("Wooden Bench", 10, 115),
            Construction::WoodDiningTable => ("Wood Dining Table", 10, 115),
            Construction::FirepitWithHook => ("Firepit with Hook", 11, 60),
            Construction::WoodTable => ("Wood Table", 12, 87),
            Construction::CiderBarrel => ("Cider Barrel", 12, 91),
            Construction::FernSmallPlant => ("Fern (small plant)", 12, 100),
            Construction::HugePlant => ("Huge Plant", 12, 100),
            Construction::Reeds => ("Reeds", 12, 100),
            Construction::TallPlant => ("Tall Plant", 12, 100),
            Construction::WoodenShelves2 => ("Wooden Shelves 2", 12, 147),
            Construction::Rug => ("Rug", 13, 60),
            Construction::RockingChair => ("Rocking Chair", 14, 87),
            Construction::OakPlank => ("Oak Plank", 15, 60),
            Construction::OakTree => ("Oak Tree", 15, 70),
            Construction::ImpStatue => ("Imp Statue", 15, 150),
            Construction::OakDecoration => ("Oak Decoration", 16, 120),
            Construction::FirepitWithPot => ("Firepit with Pot", 17, 80),
            Construction::AsgarnianAle => ("Asgarnian Ale", 18, 184),
            Construction::Curtains => ("Curtains", 18, 225),
            Construction::CatBasket => ("Cat Basket", 19, 58),
            Construction::OakChair => ("Oak Chair", 19, 120),
            Construction::ShoeBox => ("Shoe Box", 20, 58),
            Construction::WoodenBed => ("Wooden Bed", 20, 117),
            Construction::ShavingStand => ("Shaving Stand", 21, 30),
            Construction::OakBench => ("Oak Bench", 22, 240),
            Construction::OakDiningTable => ("Oak Dining Table", 22, 240),
            Construction::WoodenShelves3 => ("Wooden Shelves 3", 23, 147),
            Construction::SmallOven => ("Small Oven", 24, 80),
            Construction::OakClock => ("Oak Clock", 25, 142),
            Construction::RopeBellPull => ("Rope Bell-Pull", 26, 64),
            Construction::OakArmchair => ("Oak Armchair", 26, 180),
            Construction::GreenmansAle => ("Greenman's Ale", 26, 184),
            Construction::OakDrawers => ("Oak Drawers", 27, 120),
            Construction::PumpAndTub => ("Pump and Tub", 27, 200),
            Construction::DeadmanRug => ("Deadman rug", 28, 283),
            Construction::OakShavingStand => ("Oak Shaving Stand", 29, 61),
            Construction::LargeOven => ("Large Oven", 29, 100),
            Construction::OakBookcase => ("Oak Bookcase", 29, 180),
            Construction::WillowTree => ("Willow Tree", 30, 100),
            Construction::OakBed => ("Oak Bed", 30, 210),
            Construction::LongBone => ("Long Bone", 30, 4500),
            Construction::CurvedBone => ("Curved Bone", 30, 6750),
            Construction::CarvedOakBench => ("Carved Oak Bench", 31, 240),
            Construction::CarvedOakTable => ("Carved Oak Table", 31, 360),
            Construction::OakKitchenTable => ("Oak Kitchen Table", 32, 180),
            Construction::BoxingRing => ("Boxing Ring", 32, 420),
            Construction::StoneFireplace => ("Stone Fireplace", 33, 40),
            Construction::CushionedBasket => ("Cushioned Basket", 33, 58),
            Construction::OakLarder => ("Oak Larder", 33, 480),
            Construction::GloveRack => ("Glove Rack", 34, 120),
            Construction::SteelRange => ("Steel Range", 34, 120),
            Construction::OakShelves1 => ("Oak Shelves 1", 34, 240),
            Construction::LargeOakBed => ("Large Oak Bed", 34, 330),
            Construction::TeakPlank => ("Teak Plank", 35, 90),
            Construction::TeakArmchair => ("Teak Armchair", 35, 180),
            Construction::TeakDecoration => ("Teak Decoration", 36, 180),
            Construction::DragonBitter => ("Dragon Bitter", 36, 224),
            Construction::BellPull => ("Bell-Pull", 37, 120),
            Construction::OakDresser => ("Oak Dresser", 37, 121),
            Construction::TeakBench => ("Teak Bench", 38, 360),
            Construction::TeakTable => ("Teak Table", 38, 360),
            Construction::OakWardrobe => ("Oak Wardrobe", 39, 180),
            Construction::OakLectern => ("Oak Lectern", 40, 60),
            Construction::MahoganyPlank => ("Mahogany Plank", 40, 140),
            Construction::TeakBed => ("Teak Bed", 40, 300),
            Construction::OpulentCurtains => ("Opulent Curtains", 40, 315),
            Construction::MahoganyBookcase => ("Mahogany Bookcase", 40, 420),
            Construction::Globe => ("Globe", 41, 180),
            Construction::FencingRing => ("Fencing Ring", 41, 570),
            Construction::FancyRange => ("Fancy Range", 42, 160),
            Construction::CrystalBall => ("Crystal Ball", 42, 280),
            Construction::AlchemicalChart => ("Alchemical Chart", 43, 30),
            Construction::TeakLarder => ("Teak larder", 43, 750),
            Construction::WoodenTelescope => ("Wooden Telescope", 44, 121),
            Construction::WeaponsRack => ("Weapons Rack", 44, 180),
            Construction::CarvedTeakBench => ("Carved Teak Bench", 44, 360),
            Construction::MapleTree => ("Maple Tree", 45, 122),
            Construction::OakShelves2 => ("Oak Shelves 2", 45, 240),
            Construction::LargeTeakBed => ("Large Teak Bed", 45, 480),
            Construction::CarvedTeakTable => ("Carved Teak Table", 45, 600),
            Construction::TeakDresser => ("Teak Dresser", 46, 181),
            Construction::DemonLectern => ("Demon Lectern", 47, 120),
            Construction::EagleLectern => ("Eagle Lectern", 47, 120),
            Construction::Sink => ("Sink", 47, 300),
            Construction::MountedMythicalCape => ("Mounted Mythical Cape", 47, 370),
            Construction::ChefsDelight => ("Chef's Delight", 48, 224),
            Construction::TeleportFocus => ("Teleport Focus", 50, 40),
            Construction::OrnamentalGlobe => ("Ornamental Globe", 50, 270),
            Construction::TeakPortal => ("Teak Portal", 50, 270),
            Construction::MahoganyArmchair => ("Mahogany Armchair", 50, 280),
            Construction::TeakDrawers => ("Teak Drawers", 51, 180),
            Construction::CombatRing => ("Combat Ring", 51, 630),
            Construction::TeakKitchenTable => ("Teak Kitchen Table", 52, 270),
            Construction::MahoganyBench => ("Mahogany Bench", 52, 560),
            Construction::MahoganyTable => ("Mahogany Table", 52, 840),
            Construction::FourPosterBed => ("4-Poster Bed", 53, 450),
            Construction::ExtraWeaponsRack => ("Extra Weapons Rack", 54, 440),
            Construction::ElementalSphere => ("Elemental Sphere", 54, 580),
            Construction::TeakClock => ("Teak Clock", 55, 202),
            Construction::FancyTeakDresser => ("Fancy Teak Dresser", 56, 182),
            Construction::TeakShelves1 => ("Teak Shelves 1", 56, 330),
            Construction::GildedDecoration => ("Gilded Decoration", 56, 1020),
            Construction::TeakDemonLectern => ("Teak Demon Lectern", 57, 180),
            Construction::TeakEagleLectern => ("Teak Eagle Lectern", 57, 180),
            Construction::LimestoneAttackStone => ("Limestone attack stone", 59, 200),
            Construction::LunarGlobe => ("Lunar Globe", 59, 570),
            Construction::YewTree => ("Yew Tree", 60, 141),
            Construction::SpiceRack => ("Spice Rack", 60, 374),
            Construction::PoshBellPull => ("Posh Bell-Pull", 60, 420),
            Construction::GildedFourPosterBed => ("Gilded 4-Poster Bed", 60, 1330),
            Construction::GildedBench => ("Gilded Bench", 61, 1760),
            Construction::AstronomicalChart => ("Astronomical Chart", 63, 45),
            Construction::TeakWardrobe => ("Teak Wardrobe", 63, 270),
            Construction::MarbleFireplace => ("Marble Fireplace", 63, 500),
            Construction::TeakTelescope => ("Teak Telescope", 64, 181),
            Construction::MahoganyDresser => ("Mahogany Dresser", 64, 281),
            Construction::OpulentRug => ("Opulent Rug", 65, 360),
            Construction::MahoganyPortal => ("Mahogany Portal", 65, 420),
            Construction::GreaterFocus => ("Greater Focus", 65, 500),
            Construction::TeakGardenBench => ("Teak Garden Bench", 66, 540),
            Construction::CrystalOfPower => ("Crystal of Power", 66, 890),
            Construction::MahoganyDemonLectern => ("Mahogany Demon Lectern", 67, 580),
            Construction::MahoganyEagleLectern => ("Mahogany Eagle Lectern", 67, 580),
            Construction::TeakShelves2 => ("Teak Shelves 2", 67, 930),
            Construction::CelestialGlobe => ("Celestial Globe", 68, 570),
            Construction::DungeonEntrance => ("Dungeon Entrance", 70, 500),
            Construction::RangingPedestals => ("Ranging Pedestals", 71, 720),
            Construction::OpulentTable => ("Opulent Table", 72, 3100),
            Construction::GildedDresser => ("Gilded Dresser", 74, 582),
            Construction::OakDoor => ("Oak Door", 74, 600),
            Construction::MagicTree => ("Magic Tree", 75, 223),
            Construction::MahoganyWardrobe => ("Mahogany Wardrobe", 75, 420),
            Construction::GnomeBench => ("Gnome Bench", 77, 840),
            Construction::ArmillaryGlobe => ("Armillary Globe", 77, 960),
            Construction::MarblePortal => ("Marble Portal", 80, 1500),
            Construction::ScryingPool => ("Scrying Pool", 80, 2000),
            Construction::BalanceBeam => ("Balance Beam", 81, 1000),
            Construction::InfernalChart => ("Infernal Chart", 83, 60),
            Construction::MahoganyTelescope => ("Mahogany Telescope", 84, 281),
            Construction::GildedClock => ("Gilded Clock", 85, 602),
            Construction::SmallOrrery => ("Small Orrery", 86, 1320),
            Construction::GildedWardrobe => ("Gilded Wardrobe", 87, 720),
            Construction::LargeOrrery => ("Large Orrery", 95, 1420),
        };

        Details::Construction(ConstructionDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: details
                .3
                .iter()
                .map(|x| Multipliers::Construction(x.clone()))
                .collect(),
        })
    }

    fn search<T>(query: T) -> Vec<Self>
    where
        T: ToString,
        Self: Sized
    {
        todo!()
    }
}


impl Detail for Construction {
    fn multipliers(&self) -> Vec<Multipliers> {
        if let Details::Construction(obj) = self.details() {
            return obj.multipliers
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
            return obj.xp;
        }

        0.0
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct ConstructionDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl IntoString for crate::stats::construction::ConstructionDetails {
    fn to_string(&self, xp_difference: f64) -> String {
        let mut vec = vec![format!(
            "{}: {}",
            c1(self.name.as_str()),
            c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp).ceil()).as_str(),
                "d"
            )
                .as_str())
        )];

        self.multipliers.iter().for_each(|Multipliers::Construction(c)| {
            let d = c.details();
            vec.push(p(format!(
                "{} {}",
                c1(format!("{}:", d.name.as_str()).as_str()),
                c2(common::commas_from_string(
                    format!("{}", (xp_difference / (self.xp * d.value)).ceil()).as_str(),
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
    WildernessConstructionTicket11To50,
    WildernessConstructionTicket51To100,
    WildernessConstructionTicket101Plus,
    DesertHardDiary,
    FremennikHardDiary,
}

impl ConstructionMultipliers {
    pub fn details(&self) -> ConstructionMultiplierDetails {
        let details = match self {
            ConstructionMultipliers::WildernessConstructionTicket11To50 => ("11-50 Tix", 1.05),
            ConstructionMultipliers::WildernessConstructionTicket51To100 => ("51-100 Tix", 1.1),
            ConstructionMultipliers::WildernessConstructionTicket101Plus => ("101+ Tix", 1.15),
            ConstructionMultipliers::DesertHardDiary => ("Desert Hard Diary", 1016.0 / 890.0),
            ConstructionMultipliers::FremennikHardDiary => ("Fremmy Hard Diary", 920.0 / 780.0),
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
