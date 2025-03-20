use crate::stats::skill::{Detail, Details, Multipliers, Skill, ToString};
use common::{c1, c2, p};

pub enum Agility {
    GnomeStrongholdCourse,
    DraynorVillageRooftop,
    ShayzienBasicCourse,
    LeapingTrout,
    AlKharidRooftop,
    LeapingSalmon,
    VarrockRooftop,
    PenguinAgilityCourse,
    BarbarianOutpost,
    CanifisRooftop,
    LeapingSturgeon,
    ShayzienAdvancedCourse,
    ApeAtollCourse,
    ColossalWyrmBasicCourse,
    FaladorRooftop,
    WildernessAgilityCourseTicket,
    WildernessAgilityCourse,
    HallowedSepulchreFloor1,
    SeersVillageRooftop,
    WerewolfAgilityCourse,
    ColossalWyrmAdvancedCourse,
    HallowedSepulchreFloor2,
    PollnivneachRooftop,
    DorgeshKaanAgilityCourse,
    HallowedSepulchreFloor3,
    PrifddinasAgilityCourse,
    RellekkaRooftop,
    HallowedSepulchreFloor4,
    ArdougneRooftop,
    HallowedSepulchreFloor5,
}

impl Skill for Agility {
    fn defaults() -> Vec<Details> {
        vec![
            Agility::WildernessAgilityCourse,
            Agility::WildernessAgilityCourseTicket,
            Agility::SeersVillageRooftop,
            Agility::PollnivneachRooftop,
            Agility::RellekkaRooftop,
            Agility::ArdougneRooftop,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        let details = match (self) {
            Agility::GnomeStrongholdCourse => ("Gnome Stronghold", 1, 110.5, vec![]),
            Agility::DraynorVillageRooftop => ("Draynor Village Rooftop", 1, 120.0, vec![]),
            Agility::ShayzienBasicCourse => ("Shayzien Basic", 1, 153.5, vec![]),
            Agility::LeapingTrout => ("Leaping trout", 15, 5.0, vec![]),
            Agility::AlKharidRooftop => ("Al Kharid Rooftop", 20, 216.0, vec![]),
            Agility::LeapingSalmon => ("Leaping salmon", 30, 6.0, vec![]),
            Agility::VarrockRooftop => ("Varrock Rooftop", 30, 269.7, vec![]),
            Agility::PenguinAgilityCourse => ("Penguin", 30, 540.0, vec![]),
            Agility::BarbarianOutpost => ("Barbarian Outpost", 35, 152.5, vec![]),
            Agility::CanifisRooftop => ("Canifis Rooftop", 40, 240.0, vec![]),
            Agility::LeapingSturgeon => ("Leaping sturgeon", 45, 7.0, vec![]),
            Agility::ShayzienAdvancedCourse => ("Shayzien Advanced", 45, 508.0, vec![]),
            Agility::ApeAtollCourse => ("Ape Atoll", 48, 580.0, vec![]),
            Agility::ColossalWyrmBasicCourse => ("Colossal Wyrm Basic", 50, 504.1, vec![]),
            Agility::FaladorRooftop => ("Falador Rooftop", 50, 586.0, vec![]),
            Agility::WildernessAgilityCourse => ("Wilderness", 52, 571.4, vec![]),
            Agility::WildernessAgilityCourseTicket => (
                "Wilderness Agility Ticket",
                52,
                200.0,
                vec![
                    AgilityMultipliers::WildernessAgilityTicket11To50,
                    AgilityMultipliers::WildernessAgilityTicket51To100,
                    AgilityMultipliers::WildernessAgilityTicket101Plus,
                ],
            ),
            Agility::HallowedSepulchreFloor1 => ("Hallowed Sepulchre Fl 1", 52, 575.0, vec![]),
            Agility::SeersVillageRooftop => ("Seers' Village Rooftop", 60, 570.0, vec![]),
            Agility::WerewolfAgilityCourse => ("Werewolf", 60, 730.0, vec![]),
            Agility::ColossalWyrmAdvancedCourse => ("Colossal Wyrm Advanced", 62, 749.6, vec![]),
            Agility::HallowedSepulchreFloor2 => ("Hallowed Sepulchre Fl 2", 62, 925.0, vec![]),
            Agility::PollnivneachRooftop => (
                "Pollnivneach Rooftop",
                70,
                890.0,
                vec![AgilityMultipliers::DesertHardDiary],
            ),
            Agility::DorgeshKaanAgilityCourse => ("Dorgesh-Kaan", 70, 2750.0, vec![]),
            Agility::HallowedSepulchreFloor3 => ("Hallowed Sepulchre Fl 3", 72, 1600.0, vec![]),
            Agility::PrifddinasAgilityCourse => ("Prifddinas", 75, 1337.0, vec![]),
            Agility::RellekkaRooftop => (
                "Rellekka Rooftop",
                80,
                780.0,
                vec![AgilityMultipliers::FremennikHardDiary],
            ),
            Agility::HallowedSepulchreFloor4 => ("Hallowed Sepulchre Fl 4", 82, 2875.0, vec![]),
            Agility::ArdougneRooftop => ("Ardougne Rooftop", 90, 889.0, vec![]),
            Agility::HallowedSepulchreFloor5 => ("Hallowed Sepulchre Fl 5", 92, 5725.0, vec![]),
        };

        Details::Agility(AgilityDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            multipliers: details
                .3
                .iter()
                .map(|x| Multipliers::Agility(x.clone()))
                .collect(),
        })
    }
}

impl Detail for Agility {
    fn multipliers(&self) -> Vec<Multipliers> {
        let Details::Agility(obj) = self.details();

        obj.multipliers
    }

    fn name(&self) -> String {
        let Details::Agility(obj) = self.details();

        obj.name
    }

    fn level(&self) -> u32 {
        let Details::Agility(obj) = self.details();

        obj.level
    }

    fn xp(&self) -> f64 {
        let Details::Agility(obj) = self.details();

        obj.xp
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct AgilityDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub multipliers: Vec<Multipliers>,
}

impl ToString for AgilityDetails {
    fn to_string(&self, xp_difference: f64) -> String {
        let mut vec = vec![format!(
            "{}: {}",
            c1(self.name.as_str()),
            c2(format!("{}", (xp_difference / self.xp).ceil()).as_str())
        )];

        self.multipliers.iter().for_each(|Multipliers::Agility(a)| {
            let d = a.details();
            vec.push(p(format!(
                "{} {}",
                c1(format!("{}:", d.name.as_str()).as_str()),
                c2(format!("{}", (xp_difference / (self.xp * d.value)).ceil()).as_str())
            )
            .as_str()));
        });

        vec.join(" ")
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AgilityMultipliers {
    WildernessAgilityTicket11To50,
    WildernessAgilityTicket51To100,
    WildernessAgilityTicket101Plus,
    DesertHardDiary,
    FremennikHardDiary,
}

impl AgilityMultipliers {
    pub fn details(&self) -> AgilityMultiplierDetails {
        let details = match self {
            AgilityMultipliers::WildernessAgilityTicket11To50 => ("11-50 Tix", 1.05),
            AgilityMultipliers::WildernessAgilityTicket51To100 => ("51-100 Tix", 1.1),
            AgilityMultipliers::WildernessAgilityTicket101Plus => ("101+ Tix", 1.15),
            AgilityMultipliers::DesertHardDiary => ("Desert Hard Diary", 1016.0 / 890.0),
            AgilityMultipliers::FremennikHardDiary => ("Fremmy Hard Diary", 920.0 / 780.0),
        };

        AgilityMultiplierDetails {
            name: details.0.to_owned(),
            value: details.1,
        }
    }
}

pub struct AgilityMultiplierDetails {
    pub name: String,
    pub value: f64,
}
