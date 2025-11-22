use common::{c1, c2, l, not_found, p};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

pub fn lookup(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Salvage");
    let salvage: Salvage = query.parse()?;

    let locations = salvage
        .locations()
        .iter()
        .map(|location| c2(&location))
        .collect();

    let output1 = vec![prefix.clone(), format_salvage(&salvage.to_string())].join(" ");
    let output2 = vec![prefix, not_found(locations)].join(" ");

    Ok(vec![output1, output2])
}

fn format_salvage(salvage: &str) -> String {
    if !salvage.is_empty() {
        format!("{} ", p(&salvage))
    } else {
        "".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Salvage {
    SmallShipwreck,
    FishermansShipwreck,
    BarracudaShipwreck,
    LargeShipwreck,
    PirateShipwreck,
    MercenaryShipwreck,
    FremennikShipwreck,
    MerchantShipwreck,
    None,
}

impl Salvage {
    fn all() -> Vec<Self> {
        vec![
            Self::SmallShipwreck,
            Self::FishermansShipwreck,
            Self::BarracudaShipwreck,
            Self::LargeShipwreck,
            Self::PirateShipwreck,
            Self::MercenaryShipwreck,
            Self::FremennikShipwreck,
            Self::MerchantShipwreck,
            Self::None,
        ]
    }

    fn locations(&self) -> &'static [&'static str] {
        match self {
            Self::SmallShipwreck => &["Kharidian Sea", "Shipwreck Cove"],
            Self::FishermansShipwreck => &[
                "NE of Gu'Tanoth Bay",
                "NE of Port Piscarilius",
                "N of Fishing Platform",
            ],
            Self::BarracudaShipwreck => &[
                "SW of Pandemonium",
                "Ruins of Unkah",
                "W of Corsair Cove [Fetid]",
            ],
            Self::LargeShipwreck => &[
                "S of Shimmering Atoll",
                "N of Isle of Serpents",
                "S of Lonely Sea",
                "S of Little Pearl",
            ],
            Self::PirateShipwreck => &[
                "S of Pest Control",
                "SE of Aureum Coast",
                "SE of Buccaneer's Haven",
                "N of Pirate's Cove",
            ],
            Self::MercenaryShipwreck => &["W of Isle of Serpents"],
            Self::FremennikShipwreck => &["Everwinter Sea", "N of Ungael", "NW of Iceberg"],
            Self::MerchantShipwreck => &["N of River of Souls", "W of Dusk's Maw"],
            Self::None => &[],
        }
    }

    fn details(&self) -> SalvageDetails {
        match self {
            Self::SmallShipwreck => {
                SalvageDetails::from("Small Shipwreck", 15, 10.0, 6.0, 4.5, 60, 90)
            }
            Self::FishermansShipwreck => {
                SalvageDetails::from("Fishermans Shipwreck", 26, 17.0, 10.5, 9.0, 180, 120)
            }
            Self::BarracudaShipwreck => {
                SalvageDetails::from("Barracuda Shipwreck", 35, 31.0, 18.5, 15.5, 180, 180)
            }
            Self::LargeShipwreck => {
                SalvageDetails::from("Large Shipwreck", 53, 48.0, 28.5, 24.0, 180, 0)
            }
            Self::PirateShipwreck => {
                SalvageDetails::from("Pirate Shipwreck", 64, 63.0, 38.0, 31.5, 0, 0)
            }
            Self::MercenaryShipwreck => {
                SalvageDetails::from("Mercenary Shipwreck", 73, 127.0, 89.0, 63.5, 0, 0)
            }
            Self::FremennikShipwreck => {
                SalvageDetails::from("Fremennik Shipwreck", 80, 150.0, 105.0, 75.0, 0, 0)
            }
            Self::MerchantShipwreck => {
                SalvageDetails::from("Merchant Shipwreck", 87, 190.0, 0.0, 85.0, 0, 5)
            }
            Self::None => SalvageDetails::from("", 0, 0.0, 0.0, 0.0, 0, 0),
        }
    }
}

impl FromStr for Salvage {
    type Err = ();

    fn from_str(query: &str) -> Result<Self, Self::Err> {
        Salvage::all()
            .into_iter()
            .find(|salvage| {
                salvage
                    .to_string()
                    .to_lowercase()
                    .contains(&query.to_lowercase())
            })
            .ok_or(())
    }
}

impl fmt::Display for Salvage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SmallShipwreck => "Small Shipwreck",
            Self::FishermansShipwreck => "Fishermans Shipwreck",
            Self::BarracudaShipwreck => "Barracuda Shipwreck",
            Self::LargeShipwreck => "Large Shipwreck",
            Self::PirateShipwreck => "Pirate Shipwreck",
            Self::MercenaryShipwreck => "Mercenary Shipwreck",
            Self::FremennikShipwreck => "Fremennik Shipwreck",
            Self::MerchantShipwreck => "Merchant Shipwreck",
            Self::None => "",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SalvageDetails {
    name: String,
    level: u32,
    salvaging_xp: f64,
    crewmate_xp: f64,
    sorting_xp: f64,
    avg_lifespan: u32,
    respawn_time: u32,
}

impl SalvageDetails {
    fn from<T>(
        name: T,
        level: u32,
        salvaging_xp: f64,
        crewmate_xp: f64,
        sorting_xp: f64,
        avg_lifespan: u32,
        respawn_time: u32,
    ) -> Self
    where
        T: ToString,
    {
        Self {
            name: name.to_string(),
            level,
            salvaging_xp,
            crewmate_xp,
            sorting_xp,
            avg_lifespan,
            respawn_time,
        }
    }

    fn name(&self) -> String {
        self.name.replace("_", " ")
    }

    fn to_string(&self) -> String {
        vec![
            p(&self.name()),
            c1("Level:"),
            c2(&self.level.to_string()),
            c1("Salvaging XP:"),
            c2(&self.salvaging_xp.to_string()),
            c1("Sorting XP:"),
            c2(&self.sorting_xp.to_string()),
            c1("Total XP:"),
            c2(&(self.salvaging_xp + self.sorting_xp).to_string()),
            p(&vec![
                c1("Crewmate XP:"),
                c2(&self.crewmate_xp.to_string()),
                c1("Total XP:"),
                c2(&(self.crewmate_xp + self.sorting_xp).to_string()),
            ]
            .join(" ")),
            c1("Average Lifespan:"),
            c2(&self.avg_lifespan.to_string()),
            c1("Respawn Time:"),
            c2(&self.respawn_time.to_string()),
        ]
        .join(" ")
    }
}
