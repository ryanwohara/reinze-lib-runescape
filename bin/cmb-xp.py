#!/usr/bin/env python3
# https://oldschool.runescape.wiki/w/Bestiary

from bs4 import BeautifulSoup

import re
import requests

links = ["Levels_1_to_10",
         "Levels_11_to_20",
         "Levels_21_to_30",
         "Levels_31_to_40",
         "Levels_41_to_50",
         "Levels_51_to_60",
         "Levels_61_to_70",
         "Levels_71_to_80",
         "Levels_81_to_90",
         "Levels_91_to_100",
         "Levels_101_to_110",
         "Levels_111_to_120",
         "Levels_121_to_130",
         "Levels_131_to_140",
         "Levels_141_to_150",
         "Levels_151_to_160",
         "Levels_161_to_170",
         "Levels_171_to_180",
         "Levels_181_to_190",
         "Levels_191_to_200",
         "Levels_201_to_400",
         "Levels_higher_than_400"]

class NPC:
    name = None
    members = None
    hitpoints = None
    attack = None
    defence = None
    magic = None
    ranged = None
    stab = None
    slash = None
    crush = None
    magic_def = None
    light_ranged = None
    std_ranged = None
    heavy_ranged = None
    flat_armor = None
    weakness = None
    combat_xp = None
    hitpoints_xp = None
    slayer_req = None
    slayer_xp = None
    slayer_categories = None
    slayer_masters = None

    def __init__(self, data):
        if not len(data):
            return

        name = data[1].text
        if not name or not len(data[4].text):
            return
        self.name = name
        members = data[2].find(name="img")
        if members:
            self.members = members.get("alt")
        else:
            self.members = False
        self.combat = data[3].text
        if len(data[4].text):
            self.hitpoints = int(data[4].text)
        else:
            return

        self.attack = data[5].text
        self.defence = data[6].text
        self.magic = data[7].text
        self.ranged = data[8].text
        self.stab = data[9].text
        self.slash = data[10].text
        self.crush = data[11].text
        self.magic_def = data[12].text
        self.light_ranged = data[13].text
        self.std_ranged = data[14].text
        self.heavy_ranged = data[15].text
        self.flat_armor = data[16].text

        weakness_result = data[17]
        self.weakness = ""
        element = weakness_result.find(name="img")
        if element is not None:
            self.weakness = "{} {}".format(weakness_result.text, element.get("alt"))
        self.combat_xp = self.hitpoints * 4
        self.hitpoints_xp = self.hitpoints * 1.3

    def slayer(self, slayer_req, slayer_xp, slayer_categories, slayer_masters):
        self.slayer_req = 0 if not len(slayer_req) else int(slayer_req)
        self.slayer_xp = 0.0 if not len(slayer_xp) else float(slayer_xp)
        self.slayer_categories = slayer_categories
        self.slayer_masters = slayer_masters

    def to_string(self):
        return " ".join([self.name, "||", self.members, "|| Cmb XP: ", str(self.combat_xp), "|| Hitpoints XP:",
                         str(self.hitpoints_xp), "||", self.weakness if self.weakness else "No weakness"])


def sanitize(dirty):
    return re.sub(r"[\[\](){}*.',\s!&\\/%-]", "", dirty)

npcs = {}

for link in links:
    response = requests.request(method="GET", url=f"https://oldschool.runescape.wiki/w/Bestiary/{link}",
                                headers={"User-Agent": "Reinze - https://reinze.com/"})

    if response.status_code != 200:
        print("Got response code: {}".format(response.status_code))
        exit(1)

    soup = BeautifulSoup(response.text, 'html.parser')

    wikitable = soup.find_all(class_="wikitable")

    for table in wikitable:
        rows = table.find_all(name="tr")

        for row in rows:
            data = row.find_all(name="td")

            npc = NPC(data)
            if npc.name:
                npcs[sanitize(npc.name)] = npc


response = requests.request(method="GET", url="https://oldschool.runescape.wiki/w/Slayer/Experience_table",
                            headers={"User-Agent": "Reinze - https://reinze.com/"})

if response.status_code != 200:
    print("Got response code: {}".format(response.status_code))
    exit(1)

soup = BeautifulSoup(response.text, 'html.parser')

wikitable = soup.find_all(class_="wikitable")[1]

rows = wikitable.find_all(name="tr")

for row in rows:
    data = row.find_all(name="td")

    if not data or len(data) < 2:
        continue

    sanitized = sanitize(data[1].text)

    if npcs[sanitized]:
        npcs[sanitized].slayer(data[18].text, data[19].text, data[20].text, data[21].text)

fh = open("./src/npc/data.rs", "w")

fh.write("""use crate::stats::skill::{Details, IntoString, Skill};
use common::{c1, c2};
use std::fmt;

""")

fh.write("""
#[derive(Clone,PartialEq)]
pub enum Npc {
""")
for name in npcs:
    fh.write(f"     {name},\n")
fh.write("""
    None,
}

impl Npc {
    pub fn lookup(name: &str) -> Self {
        Self::all()
            .iter()
            .find(|npc| {
                NpcMetadata::from(npc)
                    .name
                    .to_lowercase()
                    .eq(&name.to_lowercase())
            })
            .unwrap_or(&Self::None)
            .clone()
    }

    pub fn hitpoints() -> Vec<Details> {
        vec![
            Self::HillGiant,
            Self::SandCrabActive,
            Self::GreendragonLevel79,
            Self::Bluedragon1,
            Self::AbyssaldemonStandard,
            Self::Deviantspectre,
        ]
        .iter()
        .map(|x| x.hitpoints_details())
        .collect()
    }

    pub fn hitpoints_details(&self) -> Details {
        Details::Hitpoints(NpcMetadata::from(&self))
    }

    pub fn slayer() -> Vec<Details> {
        vec![
            Self::HillGiant,
            Self::SandCrabActive,
            Self::GreendragonLevel79,
            Self::Bluedragon1,
            Self::AbyssaldemonStandard,
            Self::Deviantspectre,
        ]
        .iter()
        .map(|x| x.slayer_details())
        .collect()
    }

    pub fn slayer_details(&self) -> Details {
        Details::Slayer(NpcMetadata::from(&self))
    }
}

impl Skill for Npc {
    fn all() -> Vec<Self> {
        vec![
            Self::None,
""")
for name in npcs:
    fh.write(f"             Self::{name},\n")
fh.write("""
        ]
    }

    fn defaults() -> Vec<Details> {
        vec![
            Self::HillGiant,
            Self::SandCrabActive,
            Self::GreendragonLevel79,
            Self::Bluedragon1,
            Self::AbyssaldemonStandard,
            Self::Deviantspectre,
        ]
        .iter()
        .map(|x| x.details())
        .collect()
    }

    fn details(&self) -> Details {
        Details::Npc(NpcMetadata::from(self))
    }

    fn search<T>(name: T) -> Vec<Self>
    where
        T: ToString,
        Self: Sized,
    {
        let mut all = Self::all();

        all.retain(|npc| {
            NpcMetadata::from(npc)
                .name
                .to_lowercase()
                .contains(&name.to_string().to_lowercase())
        });

        all
    }
}    
""")

fh.write("""
#[derive(Clone, PartialEq, PartialOrd)]
pub struct NpcMetadata {
    pub name: String,
    pub members: bool,
    pub hitpoints: u32,
    pub attack: u32,
    pub defence: u32,
    pub magic: u32,
    pub ranged: u32,
    pub stab: i32,
    pub slash: i32,
    pub crush: i32,
    pub magic_def: i32,
    pub light_ranged: i32,
    pub std_ranged: i32,
    pub heavy_ranged: i32,
    #[allow(dead_code)]
    pub flat_armor: String,
    pub weakness: String,
    pub combat_xp: f64,
    pub hitpoints_xp: f64,
    pub slayer_req: u32,
    pub slayer_xp: f64,
    pub slayer_categories: String,
    pub slayer_masters: String
}

impl NpcMetadata {
    pub fn hp(&self, xp_difference: f64) -> String {
        format!(
            "{}: {}",
            c1(self.name.as_str()),
            c2(common::commas_from_string(
                format!("{}", (xp_difference / self.hitpoints_xp).ceil()).as_str(),
                "d"
            )
            .as_str())
        )
    }
    
    pub fn slay(&self, xp_difference: f64) -> String {
        format!(
            "{}: {}",
            c1(self.name.as_str()),
            c2(common::commas_from_string(
                format!("{}", (xp_difference / self.slayer_xp).ceil()).as_str(),
                "d"
            )
            .as_str())
        )
    }

    pub fn create(
        name: String,
        members: bool,
        hitpoints: u32,
        attack: u32,
        defence: u32,
        magic: u32,
        ranged: u32,
        stab: i32,
        slash: i32,
        crush: i32,
        magic_def: i32,
        light_ranged: i32,
        std_ranged: i32,
        heavy_ranged: i32,
        flat_armor: String,
        weakness: String,
        combat_xp: f64,
        hitpoints_xp: f64,
        slayer_req: u32,
        slayer_xp: f64,
        slayer_categories: String,
        slayer_masters: String) -> Self {
            Self {
                name,
                members,
                hitpoints,
                attack,
                defence,
                magic,
                ranged,
                stab,
                slash,
                crush,
                magic_def,
                light_ranged,
                std_ranged,
                heavy_ranged,
                flat_armor,
                weakness,
                combat_xp,
                hitpoints_xp,
                slayer_req,
                slayer_xp,
                slayer_categories,
                slayer_masters
        }
    }

    pub fn from(npc: &Npc) -> Self {
        match npc {
""")

for name in npcs:
    value = npcs[name]
    strname = value.name
    members = "true" if value.members == "Members" else "false"
    hitpoints = (value.hitpoints * 1) if value.hitpoints else 0
    attack = (value.attack * 1) if value.attack else 0
    defence = (value.defence * 1) if value.defence else 0
    magic = (value.magic * 1) if value.magic else 0
    ranged = (value.ranged * 1) if value.ranged else 0
    stab = (value.stab * 1) if value.stab else 0
    slash = (value.slash * 1) if value.slash else 0
    crush = (value.crush * 1) if value.crush else 0
    magic_def = (value.magic_def * 1) if value.magic_def else 0
    light_ranged = (value.light_ranged * 1) if value.light_ranged else 0
    std_ranged = (value.std_ranged * 1) if value.std_ranged else 0
    heavy_ranged = (value.heavy_ranged * 1) if value.heavy_ranged else 0
    flat_armor = value.flat_armor
    weakness = value.weakness
    combat_xp = value.combat_xp * 1.0
    hitpoints_xp = value.hitpoints_xp * 1.0
    slayer_req = (value.slayer_req * 1) if value.slayer_req else 0
    slayer_xp = (value.slayer_xp * 1.0) if value.slayer_xp else 0.0
    slayer_categories = value.slayer_categories if value.slayer_categories else ""
    slayer_masters = value.slayer_masters if value.slayer_masters else ""
    fh.write(f"""
            Npc::{name} => Self::create(
                "{strname}".to_string(),
                {members},
                {hitpoints},
                {attack},
                {defence},
                {magic},
                {ranged},
                {stab},
                {slash},
                {crush},
                {magic_def},
                {light_ranged},
                {std_ranged},
                {heavy_ranged},
                "{flat_armor}".to_string(),
                "{weakness}".to_string(),
                {combat_xp},
                {hitpoints_xp},
                {slayer_req},
                {slayer_xp},
                "{slayer_categories}".to_string(),
                "{slayer_masters}".to_string()
                ),
""")


fh.write("""
            Npc::None => Self::create(
                "".to_string(),
                false,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                "0".to_string(),
                "".to_string(),
                0.0,
                0.0,
                0,
                0.0,
                "".to_string(),
                "".to_string(),
            ),
        }
    }
}
""")

fh.write("""
impl fmt::Display for NpcMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl IntoString for NpcMetadata {
    fn to_string(&self, xp_difference: f64) -> String {
        format!(
            "{}: {}",
            c1(self.name.as_str()),
            c2(common::commas_from_string(
                format!("{}", (xp_difference / self.combat_xp).ceil()).as_str(),
                "d"
            )
            .as_str())
        )
    }
}
""")

fh.close()

print("Exfiltration successful.")
