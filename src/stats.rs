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
pub mod skill;
mod smithing;
mod thieving;
mod woodcutting;

use super::common::{
    Entry, HiscoreName, Listing, Listings, Stats, collect_hiscores, eval_query, level_to_xp, skill,
    skills, xp_to_level,
};
use crate::stats::skill::details_by_skill_id;
use common::{c1, c2, commas, l, p, source::Source};
use regex::Regex;

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
    pub fn filter(&self, input: &u32) -> bool {
        (input > &0)
            && ((self.filter_by == FilterBy::None)
                || (self.filter_by == FilterBy::GreaterThan && input > &self.filter_at)
                || (self.filter_by == FilterBy::FewerThan && input < &self.filter_at)
                || (self.filter_by == FilterBy::GreaterThanOrEqualTo && input >= &self.filter_at)
                || (self.filter_by == FilterBy::FewerThanOrEqualTo && input <= &self.filter_at)
                || (self.filter_by == FilterBy::EqualTo && input == &self.filter_at))
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
    Regex::new(r"(?:^|\b|\s)(?:(-([serox]|[iuhdlt1]|sk|fs))|([<>=]=?)\s?([\d,.]+[kmb]?)|([#^])([\d,.]+[kmb]?)|(@)(\S+))(?:\b|$)").unwrap()
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

fn prepare(command: &str) -> (usize, String) {
    let skill_name = skill(command);
    let skill_names = skills();
    let skill_id = skill_names
        .iter()
        .position(|r| r.eq(&skill_name))
        .unwrap_or(0);

    (skill_id, skill_name)
}

fn prefix(skill_name: &str, flags: &StatsFlags) -> String {
    vec![
        l(&skill_name),
        flags
            .account_type
            .name()
            .map_or("".to_string(), |name| l(&name)),
        flags.prefix.to_string(),
    ]
    .join(" ")
    .trim()
    .replace("  ", " ")
}

pub fn lookup(s: Source) -> Result<Vec<String>, ()> {
    let (skill_id, skill_name) = prepare(&s.command);

    let flags = stats_parameters(&s.query);
    let joined: String = strip_stats_parameters(&s.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let prefix = prefix(&skill_name, &flags);

    let not_found = vec![invalid(&prefix)];

    let start_xp = if flags.start > 126 {
        flags.start
    } else {
        level_to_xp(flags.start)
    };

    let start_level = xp_to_level(start_xp);

    let mut hiscores: Listings = HiscoreName::all()
        .iter()
        .map(|name| match name.to() {
            Listing::Entry(entry) => Listing::Entry(Entry {
                name: entry.name,
                level: start_level,
                xp: start_xp,
                rank: 0,
            }),
            Listing::SubEntry(subentry) => Listing::SubEntry(subentry.to_owned()),
        })
        .collect();

    if flags.start == 0 {
        hiscores = match collect_hiscores(&joined, &s, &flags) {
            Ok(hiscores) => hiscores,
            Err(_) => return Ok(not_found),
        };
        hiscores.retain_entries();
    }

    let mut stats: Stats = Stats {
        flags,
        hiscores,
        source: s,
    };

    let s = &stats.source;

    if skill_id > 0 {
        // Individual skill lookup

        let listing = stats.hiscores.skill(&skill_name);

        if listing.is_none() {
            return Ok(not_found);
        }
        let listing = listing.unwrap();

        let next_level = listing.next_level(&stats.flags);
        let next_level_xp = level_to_xp(next_level);
        let xp_difference = next_level_xp - listing.xp();
        let actual_level = listing.actual_level();

        let actual_level_string = if actual_level > listing.level() {
            p(&actual_level.to_string())
        } else {
            "".to_string()
        };

        let goal = vec![
            s.c1(&format!("XP to {}", next_level)),
            s.c2(&commas(xp_difference as f64, "d")),
            s.p(&format!("{}%", {
                let current_level_xp = level_to_xp(actual_level);
                let total_level_gap = next_level_xp - current_level_xp;
                let percentage = (1.0 - (xp_difference as f64 / total_level_gap as f64)) * 100.0;

                percentage.round()
            })),
        ]
        .join(" ");

        let level_string = vec![
            prefix,
            s.c1("Level"),
            s.c2(&commas(listing.actual_level() as f64, "d")),
            actual_level_string,
        ]
        .join(" ");

        let xp_string = vec![s.c1("XP"), s.c2(&commas(listing.xp() as f64, "d"))].join(" ");

        let rank_string = vec![s.c1("Rank"), s.c2(&commas(listing.rank() as f64, "d"))].join(" ");

        let mut result = vec![
            level_string.trim(),
            xp_string.trim(),
            goal.trim(),
            rank_string.trim(),
        ];
        result.retain(|x| x.len() > 0);

        let output = result.join(&c1(" | "));

        let details = details_by_skill_id(skill_id as u32, &stats.flags.search);

        let calc = details
            .iter()
            .map(|detail| detail.to_string(xp_difference as f64))
            .collect::<Vec<String>>()
            .join(&s.c1(" | "));

        Ok(vec![output, calc])
    } else {
        // Overall lookup

        let combat = stats.combat();
        let overall = stats.summary("Overall");

        stats.hiscores.filter(&stats.flags);

        let results = &mut stats
            .hiscores
            .iter()
            .map(|listing| match stats.flags.flag {
                MutuallyExclusiveFlag::Sort => {
                    let next_level = listing.next_level(&stats.flags);
                    let next_level_xp = level_to_xp(next_level);
                    let xp_difference = next_level_xp - listing.xp();

                    (listing.name().to_string(), xp_difference)
                }
                MutuallyExclusiveFlag::Order | MutuallyExclusiveFlag::Exp => {
                    (listing.name().to_string(), listing.xp())
                }
                MutuallyExclusiveFlag::Rank => (listing.name().to_string(), listing.rank()),
                MutuallyExclusiveFlag::None => (listing.name().to_string(), listing.actual_level()),
            })
            .collect::<Vec<(String, u32)>>();

        let summary = vec![combat.to_string(s), overall].join(" ");

        match stats.flags.flag {
            MutuallyExclusiveFlag::Order | MutuallyExclusiveFlag::Sort => {
                results.sort_by(|(_name1, number1), (_name2, number2)| number1.cmp(number2))
            }
            _ => (),
        }

        let tmp = if stats.flags.flag.ne(&MutuallyExclusiveFlag::Order) {
            results
        } else {
            &mut results
                .iter()
                .map(|(name, number)| (name.to_string(), xp_to_level(*number)))
                .collect::<Vec<(String, u32)>>()
        };

        let message = tmp
            .iter()
            .map(|(name, number)| {
                vec![
                    s.c1(&vec![name, ":"].join("")),
                    s.c2(&commas(*number as f64, "d")),
                ]
                .join("")
            })
            .collect::<Vec<String>>()
            .join(" ");

        let output = vec![prefix, summary, message].join(" ");

        Ok(vec![output])
    }
}

#[allow(dead_code)]
fn tier(points: u32) -> String {
    match points {
        0..=2499 => "Unranked",
        2500..=4999 => "Bronze",
        5000..=9999 => "Iron",
        10000..=17999 => "Steel",
        18000..=27999 => "Mithril",
        28000..=41999 => "Adamant",
        42000..=55999 => "Rune",
        _ => "Dragon",
    }
    .to_string()
}

pub fn combat(s: Source) -> Result<Vec<String>, ()> {
    let prefix = s.l("Combat");

    let not_found: Vec<String> =
        vec![vec![prefix.as_str(), &s.c1("No combat stats found")].join(" ")];

    let flags = stats_parameters(&s.query);
    let joined: String = strip_stats_parameters(&s.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let hiscores = match collect_hiscores(&joined, &s, &flags) {
        Ok(hiscores) => hiscores,
        Err(_) => return Ok(not_found),
    };

    let mut stats: Stats = Stats {
        flags,
        hiscores,
        source: s,
    };

    let s = &stats.source;

    let combat = stats.combat();
    stats.hiscores.retain_combat();
    let total_level: u32 = stats.hiscores.iter().map(|listing| listing.level()).sum();
    if total_level == 0 {
        return Ok(not_found);
    }
    let total_lvl_str = vec![s.c1("Levels:"), s.c2(&commas(total_level as f64, "d"))].join(" ");

    let total_xp: u32 = stats.hiscores.iter().map(|listing| listing.xp()).sum();
    let total_xp_str = vec![s.c1("XP:"), s.c2(&commas(total_xp as f64, "d"))].join(" ");
    let total_str = vec![total_lvl_str, total_xp_str].join(&s.c1(" | "));

    let summary = stats
        .hiscores
        .iter()
        .map(|listing| {
            vec![
                s.c1(&vec![&listing.name().to_string(), ":"].join("")),
                s.c2(&listing.level().to_string()),
            ]
            .join("")
        })
        .collect::<Vec<String>>()
        .join(" ");

    let mut calculations = combat.calc(&stats);
    calculations.retain(|(_string, int)| int > &0u32);
    let calc = calculations
        .iter()
        .map(|(string, int)| {
            vec![s.c1(&vec![string, ":"].join("")), s.c2(&int.to_string())].join("")
        })
        .collect::<Vec<String>>()
        .join(" ");

    let output = vec![
        prefix,
        combat.to_string(s),
        s.c1("Total Combat"),
        s.l(&total_str),
        s.c1("To Next Level:"),
        s.p(&calc),
        s.c1("Current Levels:"),
        s.p(&summary),
    ]
    .join(" ");

    Ok(vec![output])
}
