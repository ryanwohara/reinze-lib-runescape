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
    Entry, HiscoreName, Listing, Listings, MAX_SKILL_LEVEL, MAX_SKILL_XP, Stats, collect_hiscores,
    eval_query, level_to_xp, skill, skills, xp_to_level,
};
use crate::stats::skill::details_by_skill_id;
use anyhow::Result;
use common::{commas, source::Source};
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

impl Default for StatsFlags {
    fn default() -> Self {
        Self {
            filter_by: FilterBy::None,
            filter_at: 0,
            prefix: Prefix::None,
            account_type: AccountType::Default,
            flag: MutuallyExclusiveFlag::None,
            start: 0,
            end: 0,
            search: "".to_string(),
        }
    }
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
    pub fn to_string(&self, s: &Source) -> String {
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
            s.p(prefix)
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

    pub fn mode(&self) -> &str {
        match self {
            Self::Default => "main",
            Self::Iron => "iron",
            Self::Ultimate => "ultimate",
            Self::Hardcore => "hardcore",
            Self::Deadman => "deadman",
            Self::Leagues => "seasonal",
            Self::Tournament => "tournament",
            Self::OneDefence => "1def",
            Self::Skiller => "skiller",
            Self::FreshStart => "freshstart",
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

fn invalid<T>(prefix: T, s: &Source) -> String
where
    T: ToString,
{
    vec![
        prefix.to_string(),
        s.c1("Level"),
        s.p("N/A"),
        s.c2("|"),
        s.c1("XP"),
        s.p("N/A"),
        s.c2("|"),
        s.c1("Rank"),
        s.p("N/A"),
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

/// What a skill is working towards. Past level 126 there is no next level, so
/// the remaining milestone is the 200m XP cap - and at the cap there is nothing
/// left to work towards at all.
#[derive(Debug, PartialEq)]
pub enum Goal {
    /// (level, XP remaining, % of the way through the current level)
    NextLevel(u32, u32, u32),
    /// (XP remaining to 200m, % of the way from level 126 to 200m)
    MaxXp(u32, u32),
    /// Sitting on 200m XP.
    Maxed,
}

impl Goal {
    /// XP still to earn before the goal is met.
    fn remaining(&self) -> u32 {
        match self {
            Goal::NextLevel(_, remaining, _) => *remaining,
            Goal::MaxXp(remaining, _) => *remaining,
            Goal::Maxed => 0,
        }
    }
}

/// The levels to display: the level the hiscores report (the real in-game one,
/// which stops at 99), plus the XP-derived virtual level when it has run past
/// it. Unranked skills report no level at all, so the XP-derived one stands in.
fn level_display(reported: u32, actual: u32) -> (u32, Option<u32>) {
    let reported = if reported > 0 { reported } else { actual };

    (reported, (actual > reported).then_some(actual))
}

/// How far `xp` sits between `from` and `to`, as a rounded percentage.
fn percent(from: u32, xp: u32, to: u32) -> u32 {
    if to <= from {
        return 100;
    }

    let progress = xp.saturating_sub(from) as f64 / (to - from) as f64;
    (progress * 100.0).round().min(100.0) as u32
}

fn goal(xp: u32, actual_level: u32, next_level: u32) -> Goal {
    if next_level > MAX_SKILL_LEVEL {
        // No level exists above 126, so the last milestone is the XP cap.
        if xp >= MAX_SKILL_XP {
            return Goal::Maxed;
        }

        return Goal::MaxXp(
            MAX_SKILL_XP - xp,
            percent(level_to_xp(MAX_SKILL_LEVEL), xp, MAX_SKILL_XP),
        );
    }

    let next_level_xp = level_to_xp(next_level);

    Goal::NextLevel(
        next_level,
        next_level_xp.saturating_sub(xp),
        percent(level_to_xp(actual_level), xp, next_level_xp),
    )
}

fn goal_string(goal: &Goal, s: &Source) -> String {
    match goal {
        Goal::Maxed => vec![s.c1("200m XP"), s.p("100%")].join(" "),
        Goal::MaxXp(remaining, percentage) => vec![
            s.c1("XP to 200m"),
            s.c2(&commas(*remaining as f64, "d")),
            s.p(&format!("{}%", percentage)),
        ]
        .join(" "),
        Goal::NextLevel(level, remaining, percentage) => vec![
            s.c1(&format!("XP to {}", level)),
            s.c2(&commas(*remaining as f64, "d")),
            s.p(&format!("{}%", percentage)),
        ]
        .join(" "),
    }
}

fn prefix(skill_name: &str, flags: &StatsFlags, s: &Source) -> String {
    vec![
        s.l(&skill_name),
        flags
            .account_type
            .name()
            .map_or("".to_string(), |name| s.l(&name)),
        flags.prefix.to_string(s),
    ]
    .join(" ")
    .trim()
    .replace("  ", " ")
}

pub fn lookup(s: Source) -> Result<Vec<String>> {
    let (skill_id, skill_name) = prepare(&s.command);

    let flags = stats_parameters(&s.query);
    let joined: String = strip_stats_parameters(&s.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let prefix = prefix(&skill_name, &flags, &s);

    let not_found = vec![invalid(&prefix, &s)];

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

        let actual_level = listing.actual_level();
        let next_level = listing.next_level(&stats.flags);
        let goal = goal(listing.xp(), actual_level, next_level);
        let xp_difference = goal.remaining();

        let goal_string = goal_string(&goal, s);

        let (reported_level, virtual_level) = level_display(listing.level(), actual_level);

        let level_string = vec![
            prefix,
            s.c1("Level"),
            s.c2(&commas(reported_level as f64, "d")),
            virtual_level.map_or(String::new(), |level| s.p(&level.to_string())),
        ]
        .join(" ");

        let xp_string = vec![s.c1("XP"), s.c2(&commas(listing.xp() as f64, "d"))].join(" ");

        let rank_string = vec![s.c1("Rank"), s.c2(&commas(listing.rank() as f64, "d"))].join(" ");

        let mut result = vec![
            level_string.trim(),
            xp_string.trim(),
            goal_string.trim(),
            rank_string.trim(),
        ];
        result.retain(|x| x.len() > 0);

        let output = result.join(&s.c1(" | "));

        // At 200m XP there is no XP left to earn, so the calculator line has
        // nothing to say.
        let calc = if goal == Goal::Maxed {
            String::new()
        } else {
            details_by_skill_id(skill_id as u32, &stats.flags.search)
                .iter()
                .map(|detail| detail.to_string(s, xp_difference as f64))
                .collect::<Vec<String>>()
                .join(&s.c1(" | "))
        };

        let mut lines = vec![output, calc];
        lines.retain(|line| !line.trim().is_empty());

        Ok(lines)
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

#[cfg(test)]
mod tests {
    use super::*;
    use ::common::ColorResult;
    use ::common::author::Author;
    use std::os::raw::c_char;

    extern "C" fn stub_color(_host: *const c_char, _colors: *const c_char) -> ColorResult {
        ColorResult::default()
    }

    fn stub_source() -> Source {
        Source::create(
            "0",
            Author::create("nick!ident@host", stub_color),
            "attack",
            "",
        )
    }

    #[test]
    fn level_display_shows_the_virtual_level_beside_the_reported_one() {
        assert_eq!(level_display(99, 113), (99, Some(113)));
        assert_eq!(level_display(99, 126), (99, Some(126)));
    }

    #[test]
    fn level_display_omits_the_parenthetical_below_99() {
        assert_eq!(level_display(70, 70), (70, None));
    }

    #[test]
    fn level_display_falls_back_to_xp_when_the_skill_is_unranked() {
        // Unranked skills come back as `-1,-1,-1`, so no level is reported.
        assert_eq!(level_display(0, 1), (1, None));
    }

    #[test]
    fn goal_below_the_cap_targets_the_next_level() {
        let xp = level_to_xp(70);
        assert_eq!(
            goal(xp, 70, 71),
            Goal::NextLevel(71, level_to_xp(71) - xp, 0)
        );
    }

    #[test]
    fn goal_percentage_measures_progress_through_the_current_level() {
        let start = level_to_xp(70);
        let gap = level_to_xp(71) - start;
        assert_eq!(
            goal(start + gap / 2, 70, 71),
            Goal::NextLevel(71, gap - gap / 2, 50)
        );
    }

    #[test]
    fn goal_at_level_126_targets_200m_not_level_127() {
        let xp = level_to_xp(MAX_SKILL_LEVEL);
        assert_eq!(goal(xp, 126, 127), Goal::MaxXp(MAX_SKILL_XP - xp, 0));
    }

    #[test]
    fn goal_at_level_126_measures_progress_towards_200m() {
        let start = level_to_xp(MAX_SKILL_LEVEL);
        let gap = MAX_SKILL_XP - start;
        assert_eq!(
            goal(start + gap / 2, 126, 127),
            Goal::MaxXp(gap - gap / 2, 50)
        );
    }

    #[test]
    fn goal_at_200m_xp_is_maxed() {
        assert_eq!(goal(MAX_SKILL_XP, 126, 127), Goal::Maxed);
    }

    #[test]
    fn maxed_goal_has_nothing_left_to_earn() {
        assert_eq!(Goal::Maxed.remaining(), 0);
    }

    #[test]
    fn goal_never_underflows_when_the_target_is_already_passed() {
        // Reachable via an explicit `#` target below the player's level.
        assert_eq!(goal(level_to_xp(99), 99, 50), Goal::NextLevel(50, 0, 100));
    }

    #[test]
    fn renders_200m_as_the_target_at_level_126() {
        let rendered = goal_string(&Goal::MaxXp(11_115_260, 40), &stub_source());
        assert!(rendered.contains("XP to 200m"), "got: {rendered}");
        assert!(rendered.contains("11,115,260"), "got: {rendered}");
        assert!(rendered.contains("40%"), "got: {rendered}");
    }

    #[test]
    fn renders_100_percent_of_200m_when_maxed() {
        let rendered = goal_string(&Goal::Maxed, &stub_source());
        assert!(rendered.contains("200m XP"), "got: {rendered}");
        assert!(rendered.contains("100%"), "got: {rendered}");
        assert!(!rendered.contains("XP to"), "no target left: {rendered}");
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

pub fn combat(s: Source) -> Result<Vec<String>> {
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
