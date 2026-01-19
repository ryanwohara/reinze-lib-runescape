mod agility;
mod construction;
mod cooking;
mod crafting;
mod farming;
mod firemaking;
mod fishing;
mod fletching;
mod herblore;
mod hunter;
mod magic;
mod mining;
mod prayer;
mod runecraft;
mod sailing;
mod skill;
mod smithing;
mod thieving;
mod woodcutting;

use super::common::{
    Combat, Skills, eval_query, get_rsn, get_stats, get_total, level_to_xp,
    process_account_type_flags, skill, skills, xp_to_level,
};
use crate::stats::skill::details_by_skill_id;
use common::{c1, c2, commas, commas_from_string, convert_split_to_string, l, p, unranked};
use mysql::from_row;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct StatsFlags {
    pub filter_by: FilterBy,
    pub filter_at: u32,
    pub prefix: Prefix,
    pub account_type: AccountType,
    pub flag: MutuallyExclusiveFlag,
    pub start: u32,
    pub end: u32,
    pub search: String,
}

impl StatsFlags {
    pub fn filter(&self, level: &u32) -> bool {
        (self.filter_by == FilterBy::None)
            || (self.filter_by == FilterBy::GreaterThan && level > &self.filter_at)
            || (self.filter_by == FilterBy::FewerThan && level < &self.filter_at)
            || (self.filter_by == FilterBy::GreaterThanOrEqualTo && level >= &self.filter_at)
            || (self.filter_by == FilterBy::FewerThanOrEqualTo && level <= &self.filter_at)
            || (self.filter_by == FilterBy::EqualTo && level == &self.filter_at)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FilterBy {
    EqualTo,
    FewerThan,
    FewerThanOrEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    None,
}

impl From<&str> for FilterBy {
    fn from(value: &str) -> Self {
        match value.to_string().as_str() {
            "<" => FilterBy::FewerThan,
            "<=" => FilterBy::FewerThanOrEqualTo,
            ">" => FilterBy::GreaterThan,
            ">=" => FilterBy::GreaterThanOrEqualTo,
            "=" => FilterBy::EqualTo,
            _ => FilterBy::None,
        }
    }
}

#[allow(dead_code)]
pub enum Prefix {
    Combat,
    Level,
    LowToHigh,
    None,
    Rank,
    Xp,
    XpToLevel,
}

impl Prefix {
    pub fn to_string(&self) -> String {
        let prefix = match self {
            Self::Combat => "Combat",
            Self::Level => "Level",
            Self::LowToHigh => "Low->High",
            Self::None => "",
            Self::Rank => "Rank",
            Self::Xp => "XP",
            Self::XpToLevel => "XPtoLevel",
        };

        if prefix.len() > 0 {
            p(prefix)
        } else {
            "".to_string()
        }
    }
}

pub enum AccountType {
    Default,
    Iron,
    Ultimate,
    Hardcore,
    Deadman,
    Leagues,
    Tournament,
    OneDefence,
    Skiller,
    FreshStart,
}

impl AccountType {
    pub fn link(&self) -> String {
        match self {
            Self::Default => "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=",
            Self::Iron => "https://secure.runescape.com/m=hiscore_oldschool_ironman/index_lite.ws?player=",
            Self::Ultimate => "https://secure.runescape.com/m=hiscore_oldschool_ultimate/index_lite.ws?player=",
            Self::Hardcore => "https://secure.runescape.com/m=hiscore_oldschool_hardcore_ironman/index_lite.ws?player=",
            Self::Deadman => "https://secure.runescape.com/m=hiscore_oldschool_deadman/index_lite.ws?player=",
            Self::Leagues => "https://secure.runescape.com/m=hiscore_oldschool_seasonal/index_lite.ws?player=",
            Self::Tournament => "https://secure.runescape.com/m=hiscore_oldschool_tournament/index_lite.ws?player=",
            Self::OneDefence => "https://secure.runescape.com/m=hiscore_oldschool_skiller_defence/index_lite.ws?player=",
            Self::Skiller => "https://secure.runescape.com/m=hiscore_oldschool_skiller/index_lite.ws?player=",
            Self::FreshStart => "https://secure.runescape.com/m=hiscore_oldschool_fresh_start/index_lite.ws?player=",
        }
            .to_string()
    }

    pub fn name(&self) -> Option<String> {
        let name = match self {
            Self::Default => None,
            Self::Iron => Some("Iron"),
            Self::Ultimate => Some("Ultimate"),
            Self::Hardcore => Some("Hardcore"),
            Self::Deadman => Some("Deadman"),
            Self::Leagues => Some("Leagues"),
            Self::Tournament => Some("Tournament"),
            Self::OneDefence => Some("1 Def"),
            Self::Skiller => Some("Skiller"),
            Self::FreshStart => Some("Fresh Start"),
        };

        match name {
            Some(name) => Some(name.to_string()),
            _ => None,
        }
    }
}

#[derive(PartialEq)]
pub enum MutuallyExclusiveFlag {
    Exp,
    None,
    Order,
    Rank,
    Sort,
}

impl From<&str> for MutuallyExclusiveFlag {
    fn from(s: &str) -> Self {
        match s {
            "-o" => Self::Order,
            "-s" => Self::Sort,
            "-r" => Self::Rank,
            "-x" => Self::Exp,
            _ => Self::None,
        }
    }
}

pub fn get_stats_regex() -> Regex {
    Regex::new(r"(?:^|\b|\s)(?:(-([serox]|[iuhdlt1]|sk|fs))|([<>=]=?)\s?(\d+)|([#^])([\d,.]+[kmb]?)|(@)(\S+))(?:\b|$)").unwrap()
}

pub fn stats_parameters(query: &str) -> StatsFlags {
    let mut stats = StatsFlags {
        filter_by: FilterBy::None,
        filter_at: 0,
        prefix: Prefix::None,
        account_type: AccountType::Default,
        flag: MutuallyExclusiveFlag::None,
        start: 0,
        end: 0,
        search: "".to_string(),
    };

    for (_, [flag_identifier, detail]) in get_stats_regex()
        .captures_iter(query)
        .map(|capture| capture.extract())
    {
        match flag_identifier {
            "-i" => stats.account_type = AccountType::Iron,
            "-u" => stats.account_type = AccountType::Ultimate,
            "-h" => stats.account_type = AccountType::Hardcore,
            "-d" => stats.account_type = AccountType::Deadman,
            "-l" => stats.account_type = AccountType::Leagues,
            "-t" => stats.account_type = AccountType::Tournament,
            "-1" => stats.account_type = AccountType::OneDefence,
            "-sk" => stats.account_type = AccountType::Skiller,
            "-fs" => stats.account_type = AccountType::FreshStart,
            "-s" => stats.flag = MutuallyExclusiveFlag::Sort,
            "-o" => stats.flag = MutuallyExclusiveFlag::Order,
            "-r" => stats.flag = MutuallyExclusiveFlag::Rank,
            "-e" | "-x" => stats.flag = MutuallyExclusiveFlag::Exp,
            "^" => stats.start = eval_query(detail).unwrap_or(0.0) as u32,
            "#" => stats.end = eval_query(detail).unwrap_or(0.0) as u32,
            "@" => stats.search = detail.to_string(),
            ">" | "<" | ">=" | "<=" | "=" | "==" => {
                stats.filter_by = FilterBy::from(flag_identifier);
                stats.filter_at = eval_query(detail).unwrap_or(0.0) as u32;
            }
            _ => {}
        };
    }

    stats
}

pub fn strip_stats_parameters(query: &str) -> String {
    get_stats_regex().replace_all(query, "").to_string()
}

pub struct StrEntry<'a> {
    pub name: &'a str,
    pub rank: &'a str,
    pub level: &'a str,
    pub xp: &'a str,
}

impl<'a> StrEntry<'a> {
    pub fn new(name: &'a str, split: Vec<&'a str>) -> Self {
        if split.len() == 2 {
            Self {
                name,
                rank: split[0],
                level: "0",
                xp: split[1],
            }
        } else if split.len() == 3 {
            Self {
                name,
                rank: split[0],
                level: if split[1] != "-1" { split[1] } else { "0" },
                xp: split[2],
            }
        } else {
            Self {
                name,
                rank: "-1",
                level: "0",
                xp: "0",
            }
        }
    }

    pub fn convert(&self) -> Entry {
        Entry {
            name: self.name.to_string(),
            rank: self.rank.to_string(),
            level: self.level.parse::<u32>().unwrap_or_else(|_| 0),
            xp: self.xp.parse::<u32>().unwrap_or_else(|_| 0),
        }
    }
}

#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub rank: String,
    pub level: u32,
    pub xp: u32,
}

impl Entry {
    pub fn str_level(&self) -> String {
        self.level.to_string()
    }

    pub fn str_xp(&self) -> String {
        self.xp.to_string()
    }

    pub fn empty(&self) -> bool {
        self.xp > 0 && self.rank != "404 - Page not found"
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {}{} {}{}",
            c1("Lvl:"),
            c2(&commas_from_string(&self.str_level(), "d")),
            c1("XP:"),
            c2(&commas_from_string(&self.str_xp(), "d")),
            c1("Rank:"),
            c2(if self.rank == "-1" {
                "N/A".to_string()
            } else {
                commas_from_string(&self.rank, "d")
            }
            .as_str())
        )
    }
}

fn invalid<T>(prefix: T) -> String
where
    T: ToString,
{
    vec![
        prefix.to_string(),
        c1("Level"),
        p("N/A"),
        c2("|"),
        c1("XP"),
        p("N/A"),
        c2("|"),
        c1("Rank"),
        p("N/A"),
    ]
    .join(" ")
}

pub fn stats(command: &str, input: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let skill_name = skill(command);
    let skill_names = skills();
    // Get the skill ID from the skill name
    let skill_id = skill_names
        .iter()
        .position(|r| r.to_string() == skill_name)
        .unwrap_or(0);

    let flags = stats_parameters(input);
    let query = strip_stats_parameters(input);
    let split: Vec<String> = convert_split_to_string(query.split_whitespace().collect());

    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();

    let combat_command = command == "combat" || command == "cmb";

    let prefix = vec![
        if combat_command {
            l("Combat")
        } else {
            l(&skill_name)
        },
        flags
            .account_type
            .name()
            .map_or("".to_string(), |name| l(&name)),
        flags.prefix.to_string(),
    ]
    .join(" ")
    .trim()
    .replace("  ", " ");

    let not_found = vec![invalid(prefix.clone())];

    let hiscores_collected: Vec<Vec<&str>>;
    let mut hiscores_len = 25;
    let string_hiscores_collected;
    let string;

    if flags.start == 0 {
        let re = Regex::new(r"\s").unwrap();

        let rsn_with_spaces = if flags.start > 0 {
            nick
        } else if split.is_empty() {
            get_rsn(author, rsn_n)
                .ok()
                .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
                .unwrap_or(nick)
        } else {
            split.join(" ")
        };

        let rsn = re.replace_all(&rsn_with_spaces, "_").to_string();

        string = match reqwest::blocking::get(&format!("{}{}", flags.account_type.link(), rsn)) {
            Ok(resp) => match resp.text() {
                Ok(string) => string,
                Err(e) => {
                    println!("Error getting text: {}", e);
                    return Err(());
                }
            },
            Err(e) => {
                println!("Error making HTTP request: {}", e);
                return Err(());
            }
        };

        let hiscores_split = string.split('\n').collect::<Vec<&str>>();
        hiscores_len = hiscores_split.len() - 1;
        if hiscores_len > 25 {
            // 24 skills
            hiscores_len = 25;
        }

        hiscores_collected = hiscores_split[0..=hiscores_len]
            .iter()
            .map(|x| x.split(',').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
    } else {
        let start_xp = if flags.start > 126 {
            flags.start
        } else {
            level_to_xp(flags.start)
        };

        let start_level = xp_to_level(start_xp);

        string_hiscores_collected = (0..hiscores_len)
            .collect::<Vec<usize>>()
            .iter()
            .map(|_| {
                vec![
                    "0".to_string(),
                    format!("{start_level}"),
                    format!("{start_xp}"),
                ]
            })
            .collect::<Vec<Vec<String>>>();

        hiscores_collected = string_hiscores_collected
            .iter()
            .map(|vec| vec![vec[0].as_str(), vec[1].as_str(), vec[2].as_str()])
            .collect::<Vec<Vec<&str>>>();
    }

    let mut index = -1isize;
    let mut skill_data = Vec::new();
    let mut sortable_data = Vec::new();
    let mut skill_lookup_data: Skills = HashMap::new();

    for split in hiscores_collected {
        index += 1;

        let entry = StrEntry::new(&skill_name, split).convert();

        if skill_id != 0 && index as usize == skill_id {
            // individual skill
            if entry.empty() {
                return Ok(not_found);
            }

            let actual_level = xp_to_level(entry.xp);

            let next_level;
            let next_level_xp;
            if flags.end > 0 {
                if flags.end <= 126 {
                    next_level = flags.end;
                    next_level_xp = level_to_xp(flags.end);
                } else {
                    next_level_xp = flags.end;
                    next_level = xp_to_level(flags.end);
                }
            } else {
                next_level = actual_level + 1;
                next_level_xp = level_to_xp(next_level);
            }

            let xp_difference = next_level_xp - entry.xp;

            let mut output: Vec<String> = Vec::new();

            let actual_level_string;
            if actual_level > entry.level {
                actual_level_string = format!(" {}", p(&actual_level.to_string()));
            } else {
                actual_level_string = "".to_string();
            }

            output.push(format!(
                "{} {}{}",
                c1("Level"),
                c2(&commas_from_string(&entry.str_level(), "d")),
                actual_level_string
            ));

            output.push(format!(
                "{} {}",
                c1("XP"),
                c2(&commas_from_string(&entry.str_xp(), "d"))
            ));

            if entry.name != "Overall" && next_level < 127 {
                output.push(format!(
                    "{} {} {}",
                    c1(&format!("XP to {}", next_level)),
                    c2(&commas_from_string(&xp_difference.to_string(), "d")),
                    p(&format!("{}%", {
                        let current_level_xp = level_to_xp(actual_level);
                        let total_level_gap = next_level_xp - current_level_xp;
                        let percentage =
                            (1.0 - (xp_difference as f64 / total_level_gap as f64)) * 100.0;

                        percentage.round()
                    }))
                ));
            }

            if entry.rank != "0" {
                output.push(format!(
                    "{} {}",
                    c1("Rank"),
                    c2(&commas_from_string(&entry.rank, "d"))
                ));
            }

            let message = format!("{} {}", prefix, unranked(output));

            if next_level > 126 {
                return Ok(vec![message]);
            }

            let details = details_by_skill_id(skill_id as u32, flags.search.as_str());

            let calc = details
                .iter()
                .map(|detail| detail.to_string(xp_difference as f64))
                .collect::<Vec<String>>()
                .join(format!(" {} ", c1("|")).as_str());

            if calc.len() == 0 {
                return Ok(vec![message]);
            }

            return Ok(vec![message, calc]);
        } else if skill_id == 0 && index == 0 {
            // overall
            if !entry.empty() {
                if !combat_command {
                    skill_data.push(entry.to_string());
                }
            }
        } else if skill_id == 0 && index < hiscores_len as isize {
            // all skills
            if !entry.empty() {
                let actual_level = xp_to_level(entry.xp);
                let next_level = actual_level + 1;
                let next_level_xp = level_to_xp(next_level);
                let xp_difference = next_level_xp - entry.xp;

                skill_lookup_data.insert(&skill_names[index as usize], entry.clone());

                // if filters were passed
                if flags.filter(&entry.level) {
                    if flags.flag == MutuallyExclusiveFlag::Sort
                        || flags.flag == MutuallyExclusiveFlag::Order
                    {
                        // if -o or -s was passed
                        sortable_data
                            .push(((entry.rank, entry.level, entry.xp, xp_difference), index));
                    } else if flags.flag == MutuallyExclusiveFlag::Exp {
                        // if -e was passed
                        skill_data.push(format!(
                            "{}{}",
                            c1(&format!("{}:", &entry.name)),
                            c2(&commas(entry.xp as f64, "d")),
                        ));
                    } else if flags.flag == MutuallyExclusiveFlag::Rank {
                        // if -r was passed
                        skill_data.push(format!(
                            "{}{}",
                            c1(&format!("{}:", &skill_names[index as usize])),
                            c2(&commas_from_string(&entry.rank, "d")),
                        ));
                    } else if combat_command {
                        if index > 0 && index < 8 {
                            // if combat skill
                            skill_data.push(format!(
                                "{}{}",
                                c1(&format!("{}:", &skill_names[index as usize])),
                                c2(&commas_from_string(&entry.str_level(), "d")),
                            ));
                        }
                    } else {
                        // otherwise...
                        skill_data.push(format!(
                            "{}{}",
                            c1(&format!("{}:", &skill_names[index as usize])),
                            c2(&actual_level.to_string()),
                        ));
                    }
                }
            }
        }
    }

    // if -s was passed (NOT -sk)
    if flags.flag == MutuallyExclusiveFlag::Sort {
        // sort the skills by xp_difference
        sortable_data.sort_by(|a, b| a.0.3.cmp(&b.0.3));

        skill_data = Vec::new();

        for data in sortable_data {
            let xp_difference = data.0.3;
            let index = data.1;

            skill_data.push(format!(
                "{} {}",
                c1(&format!("{}:", &skill_names[index as usize])),
                c2(&commas(xp_difference as f64, "d")),
            ));
        }
    } else if flags.flag == MutuallyExclusiveFlag::Order {
        // sort the skills by xp_difference if -o was passed
        sortable_data.sort_by(|a, b| a.0.2.cmp(&b.0.2));

        skill_data = Vec::new();

        for data in sortable_data {
            let level = data.0.1;
            let index = data.1;

            skill_data.push(format!(
                "{} {}",
                c1(&format!("{}:", &skill_names[index as usize])),
                c2(&level.to_string()),
            ));
        }
    } else if skill_id == 0 && !skill_data.is_empty() {
        // we need to include combat level in the overall summary
        let cmb = Combat::calc(&skill_lookup_data);
        let total_level = get_total(&skill_lookup_data, "level");
        let total_xp = get_total(&skill_lookup_data, "xp");

        if combat_command {
            skill_data.insert(
                0,
                format!(
                    "{}{} {} {}{} {}{}",
                    c1("Combat:"),
                    c2(&cmb.level.to_string()),
                    p(&cmb.style),
                    c1("Total Cmb Levels:"),
                    c2(&total_level.to_string()),
                    c1("Total XP:"),
                    c2(&commas(total_xp as f64, "d")),
                ),
            );

            let next_cmb_level = (cmb.level + 1.0).floor();
            let level_difference = next_cmb_level - cmb.level;

            let att_to_next_level =
                parse_skill_data_for_cmb("Attack", &skill_lookup_data, level_difference);
            let str_to_next_level =
                parse_skill_data_for_cmb("Strength", &skill_lookup_data, level_difference);
            let def_to_next_level =
                parse_skill_data_for_cmb("Defence", &skill_lookup_data, level_difference);
            let hp_to_next_level =
                parse_skill_data_for_cmb("Hitpoints", &skill_lookup_data, level_difference);
            let prayer_to_next_level =
                parse_skill_data_for_cmb("Prayer", &skill_lookup_data, level_difference);
            let range_to_next_level =
                parse_skill_data_for_cmb("Ranged", &skill_lookup_data, level_difference);
            let mage_to_next_level =
                parse_skill_data_for_cmb("Magic", &skill_lookup_data, level_difference);

            let to_next_cmb_level = calculate_next_cmb_level_req(
                cmb,
                att_to_next_level,
                str_to_next_level,
                def_to_next_level,
                hp_to_next_level,
                prayer_to_next_level,
                range_to_next_level,
                mage_to_next_level,
            );

            if to_next_cmb_level.len() > 0 {
                skill_data.push(c1("|| To next level:"));

                to_next_cmb_level
                    .iter()
                    .for_each(|x| skill_data.push(x.to_string()));
            }
        } else {
            skill_data.insert(
                0,
                format!("{}{}", c1("Combat:"), c2(&cmb.level.to_string()),),
            );
        }
    }

    // wrap up the data and return it
    if skill_data.len() > 0 {
        let output = vec![prefix, skill_data.join(" ")]
            .join(" ")
            .replace("  ", " ");
        return Ok(vec![output]);
    }

    Ok(not_found)
}

fn calculate_next_cmb_level_req(
    cmb: Combat,
    att_to_next_level: u32,
    str_to_next_level: u32,
    def_to_next_level: u32,
    hp_to_next_level: u32,
    prayer_to_next_level: u32,
    range_to_next_level: u32,
    mage_to_next_level: u32,
) -> Vec<String> {
    let mut list = Vec::new();

    match cmb.style.as_str() {
        "Melee" => {
            if att_to_next_level > 0 {
                list.push(format!(
                    "{}{}",
                    c1("Attack:"),
                    c2(&att_to_next_level.to_string())
                ));
            }

            if str_to_next_level > 0 {
                list.push(format!(
                    "{}{}",
                    c1("Strength:"),
                    c2(&str_to_next_level.to_string())
                ));
            }
        }
        "Ranged" => {
            if range_to_next_level > 0 {
                list.push(format!(
                    "{}{}",
                    c1("Ranged:"),
                    c2(&range_to_next_level.to_string())
                ));
            }
        }
        "Magic" => {
            if mage_to_next_level > 0 {
                list.push(format!(
                    "{}{}",
                    c1("Magic:"),
                    c2(&mage_to_next_level.to_string())
                ));
            }
        }
        _ => {}
    }

    if def_to_next_level > 0 {
        list.push(format!(
            "{}{}",
            c1("Defence:"),
            c2(&def_to_next_level.to_string())
        ));
    }

    if hp_to_next_level > 0 {
        list.push(format!(
            "{}{}",
            c1("Hitpoints:"),
            c2(&hp_to_next_level.to_string())
        ));
    }

    if prayer_to_next_level > 0 {
        list.push(format!(
            "{}{}",
            c1("Prayer:"),
            c2(&prayer_to_next_level.to_string())
        ));
    }

    list
}

fn parse_skill_data_for_cmb(skill: &str, skill_lookup_data: &Skills, level_difference: f64) -> u32 {
    match skill_lookup_data.get(&skill.to_owned()) {
        Some(entry) => match entry.level {
            99 => 0.0,
            _ => match skill {
                "Attack" | "Strength" => level_difference / 0.325,
                "Defence" | "Hitpoints" => level_difference / 0.25,
                "Prayer" => level_difference / 0.125,
                _ => level_difference / 0.4875,
            },
        },
        None => 0.0,
    }
    .ceil() as u32
}

pub fn process_stats_subsection(
    query: &str,
    author: &str,
    rsn_n: &str,
    cmd_prefix: &str,
    categories: Vec<&str>,
    offset: isize,
) -> Result<Vec<String>, ()> {
    let split: Vec<String> = convert_split_to_string(query.split(" ").collect());

    let (mut split, flag_prefix, base_url) = process_account_type_flags(query, split);

    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();
    let mut filter = "".to_owned();

    let joined = split.join(" ");

    if joined.contains("@") {
        let cloned = joined.clone();

        split = cloned
            .split("@")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();

        if split.len() > 1 {
            filter = split[1].trim().to_owned();
            split = vec![split[0].trim().to_owned()];
        } else {
            filter = split[0].trim().to_owned();
            split = vec!["".to_owned()];
        }
    }

    let rsn_with_spaces = if split.is_empty() || split[0].is_empty() {
        get_rsn(author, rsn_n)
            .ok()
            .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
            .unwrap_or_else(|| nick.to_owned())
    } else {
        split.join(" ")
    };

    let re = Regex::new(r"\s").unwrap();
    let rsn = re.replace_all(&rsn_with_spaces, "_").to_string();

    let stats = match get_stats(&rsn, &base_url) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    let mut prefix_vec = vec![cmd_prefix, &flag_prefix];
    prefix_vec.retain(|x| !x.is_empty());
    let prefix = prefix_vec.join(" ");

    let mut vec: Vec<String> = Vec::new();
    let mut index = 0 - 1isize;

    let mut additional = "".to_string();
    for line in stats {
        index += 1;

        if index - offset >= 0 && index - offset < categories.len() as isize {
            if line[0] == "-1" {
                continue;
            }

            let name: &str = categories[(index - offset) as usize];
            let rank = &line[0];
            let points = &line[1];

            if filter.len() == 0 || name.to_lowercase().contains(&filter.to_lowercase()) {
                vec.push(format!(
                    "{}: {} {}",
                    c1(name),
                    c2(&commas_from_string(points, "d")),
                    p(if rank == "-1" {
                        "N/A".to_string()
                    } else {
                        commas_from_string(rank, "d")
                    }
                    .as_str())
                ));

                if offset == 26 {
                    let points = points.parse::<u32>().unwrap_or(0);

                    let tier = match points {
                        0..=2499 => "Unranked",
                        2500..=4999 => "Bronze",
                        5000..=9999 => "Iron",
                        10000..=17999 => "Steel",
                        18000..=27999 => "Mithril",
                        28000..=41999 => "Adamant",
                        42000..=55999 => "Rune",
                        _ => "Dragon",
                    };

                    additional = format!(" {} {}", &c1("Tier:"), &c2(tier));
                }
            }
        }
    }

    let output = format!("{} {}{}", prefix, unranked(vec), additional);

    Ok(vec![output])
}
