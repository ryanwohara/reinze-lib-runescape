use super::common::{
    get_cmb, get_rsn, get_stats, level_to_xp, process_account_type_flags, skill, skills,
    xp_to_level, Combat,
};
use common::{c1, c2, commas, commas_from_string, convert_split_to_string, l, p, unranked};
use mysql::from_row;
use regex::Regex;
use std::collections::HashMap;

pub fn stats(command: &str, query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let skill = skill(command);
    let skills = skills();
    // Get the skill ID from the skill name
    let skill_id = skills
        .iter()
        .position(|r| r.to_string() == skill)
        .unwrap_or(0);

    let split: Vec<String> = convert_split_to_string(query.split(" ").collect());

    let (split, (flag_filter_by, flag_filter_at)) = process_filter_by_flags(query, split);

    let (split, mut prefix, base_url) = process_account_type_flags(query, split);

    let (split, (flag_sort, flag_exp, flag_rank)) = process_ser_flags(query, split);

    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();
    let rsn = if split.is_empty() || split[0].is_empty() {
        get_rsn(author, rsn_n)
            .ok()
            .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
            .unwrap_or_else(|| nick.to_owned())
    } else {
        split.join(" ")
    };

    let mut combat_command = false;
    if command == "combat" || command == "cmb" {
        combat_command = true;
    }

    if flag_exp {
        prefix = vec![l(&skill), prefix, p("XP")].join(" ");
    } else if flag_rank {
        prefix = vec![l(&skill), prefix, p("Rank")].join(" ");
    } else if flag_sort {
        prefix = vec![l(&skill), prefix, p("XP to Level")].join(" ");
    } else if combat_command {
        prefix = l("Combat");
    } else {
        prefix = vec![l(&skill), prefix, p("Level")].join(" ");
    }

    prefix = prefix.replace("  ", " ");

    let not_found = vec![format!(
        "{} {} {} {} {} {} {} {} {}",
        prefix,
        c1("Level"),
        p("N/A"),
        c2("|"),
        c1("XP"),
        p("N/A"),
        c2("|"),
        c1("Rank"),
        p("N/A")
    )];

    let resp = match reqwest::blocking::get(&format!("{}{}", base_url, rsn)) {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error making HTTP request: {}", e);
            return Err(());
        }
    };

    let string = match resp.text() {
        Ok(string) => string,
        Err(e) => {
            println!("Error getting text: {}", e);
            return Err(());
        }
    };

    let hiscores_split = string.split('\n').collect::<Vec<&str>>();
    let mut hiscores_len = hiscores_split.len() - 1;
    if hiscores_len > 23 {
        // 23 skills
        hiscores_len = 23;
    }

    let hiscores_collected = hiscores_split[0..=hiscores_len]
        .iter()
        .map(|x| x.split(',').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut index = 0 - 1 as isize;
    let mut skill_data = Vec::new();
    let mut sortable_data = Vec::new();
    let mut skill_lookup_data = HashMap::new();
    let mut skill_xp_lookup_data = HashMap::new();

    for split in hiscores_collected {
        index += 1;

        if skill_id != 0 && index as usize == skill_id {
            // individual skill
            if split[0].contains("404 - Page not found") || split.len() < 2 || split[1] == "-1" {
                return Ok(not_found);
            }

            let rank = split[0];
            let str_level = split[1];
            let level = match str_level.parse::<u32>() {
                Ok(level) => level,
                Err(_) => 0,
            };
            let str_xp = split[2];
            let xp = match str_xp.parse::<u32>() {
                Ok(xp) => xp,
                Err(_) => 0,
            };
            let actual_level = xp_to_level(xp);
            let next_level = actual_level + 1;
            let next_level_xp = level_to_xp(next_level);
            let xp_difference = next_level_xp - xp;

            let mut output: Vec<String> = Vec::new();

            let actual_level_string;
            if actual_level > level {
                actual_level_string = format!(" {}", p(&actual_level.to_string()));
            } else {
                actual_level_string = "".to_string();
            }

            output.push(format!(
                "{} {}{}",
                c1("Level"),
                c2(&commas_from_string(str_level, "d")),
                actual_level_string
            ));

            output.push(format!(
                "{} {}",
                c1("XP"),
                c2(&commas_from_string(str_xp, "d"))
            ));

            if skill != "Overall" && next_level < 127 {
                output.push(format!(
                    "{} {}",
                    c1(&format!("XP to {}", next_level)),
                    c2(&commas_from_string(&xp_difference.to_string(), "d"))
                ));
            }

            output.push(format!(
                "{} {}",
                c1("Rank"),
                c2(&commas_from_string(rank, "d"))
            ));

            let message = format!("{} {}", prefix, unranked(output));

            return Ok(vec![message]);
        } else if skill_id == 0 && index == 0 {
            // overall
            if split[0] != "-1" && !split[0].contains("404 - Page not found") {
                if !combat_command {
                    skill_data.push(format!(
                        "{}{} {}{} {}{}",
                        c1("Lvl:"),
                        c2(&commas_from_string(split[1], "d")),
                        c1("XP:"),
                        c2(&commas_from_string(split[2], "d")),
                        c1("Rank:"),
                        c2(&commas_from_string(split[0], "d")),
                    ));
                }
            }
        } else if skill_id == 0 && index < hiscores_len as isize {
            // all skills
            if split[0] != "-1" && !split[0].contains("404 - Page not found") {
                let rank = split[0];
                let str_level = split[1];
                let level = match str_level.parse::<u32>() {
                    Ok(level) => level,
                    Err(_) => 0,
                };
                let str_xp = split[2];
                let xp = match str_xp.parse::<u32>() {
                    Ok(xp) => xp,
                    Err(_) => 0,
                };
                let actual_level = xp_to_level(xp);
                let next_level = actual_level + 1;
                let next_level_xp = level_to_xp(next_level);
                let xp_difference = next_level_xp - xp;

                skill_lookup_data.insert(&skills[index as usize], level);
                skill_xp_lookup_data.insert(&skills[index as usize], xp);

                // if filters were passed
                if (flag_filter_by.len() == 0)
                    || (flag_filter_by.len() > 0
                        && ((flag_filter_by == ">" && level > flag_filter_at)
                            || (flag_filter_by == "<" && level < flag_filter_at)
                            || (flag_filter_by == ">=" && level >= flag_filter_at)
                            || (flag_filter_by == "<=" && level <= flag_filter_at)
                            || (flag_filter_by.starts_with("=") && level == flag_filter_at)))
                {
                    if flag_sort {
                        // if -s was passed
                        if level < 99 {
                            sortable_data.push(((rank, level, xp, xp_difference), index));
                        }
                    } else if flag_exp {
                        // if -e was passed
                        skill_data.push(format!(
                            "{}{}",
                            c1(&format!("{}:", &skills[index as usize])),
                            c2(&commas(xp as f64, "d")),
                        ));
                    } else if flag_rank {
                        // if -r was passed
                        skill_data.push(format!(
                            "{}{}",
                            c1(&format!("{}:", &skills[index as usize])),
                            c2(&commas_from_string(rank, "d")),
                        ));
                    } else if combat_command {
                        if index > 0 && index < 8 {
                            // if combat skill
                            skill_data.push(format!(
                                "{}{}",
                                c1(&format!("{}:", &skills[index as usize])),
                                c2(&commas_from_string(str_level, "d")),
                            ));
                        }
                    } else {
                        // otherwise...
                        skill_data.push(format!(
                            "{}{}",
                            c1(&format!("{}:", &skills[index as usize])),
                            c2(&actual_level.to_string()),
                        ));
                    }
                }
            }
        }
    }

    // if -s was passed
    if flag_sort {
        // sort the skills by xp_difference
        sortable_data.sort_by(|a, b| a.0 .3.cmp(&b.0 .3));

        skill_data = Vec::new();

        for data in sortable_data {
            let xp_difference = data.0 .3;
            let index = data.1;

            skill_data.push(format!(
                "{} {}",
                c1(&format!("{}:", &skills[index as usize])),
                c2(&commas(xp_difference as f64, "d")),
            ));
        }
    } else if skill_id == 0 && !skill_data.is_empty() {
        // we need to include combat level
        // in the overall summary
        let cmb = get_cmb(
            skill_lookup_data.get(&"Attack".to_string()).unwrap_or(&1),
            skill_lookup_data.get(&"Strength".to_string()).unwrap_or(&1),
            skill_lookup_data.get(&"Defence".to_string()).unwrap_or(&1),
            skill_lookup_data
                .get(&"Hitpoints".to_string())
                .unwrap_or(&10),
            skill_lookup_data.get(&"Ranged".to_string()).unwrap_or(&1),
            skill_lookup_data.get(&"Prayer".to_string()).unwrap_or(&1),
            skill_lookup_data.get(&"Magic".to_string()).unwrap_or(&1),
        );

        let total_level = skill_lookup_data.get(&"Attack".to_string()).unwrap_or(&0) + 
        skill_lookup_data.get(&"Strength".to_string()).unwrap_or(&0) + 
        skill_lookup_data.get(&"Defence".to_string()).unwrap_or(&0) + 
        skill_lookup_data
            .get(&"Hitpoints".to_string())
            .unwrap_or(&1151) + // HP is level 10
        skill_lookup_data.get(&"Ranged".to_string()).unwrap_or(&0) + 
        skill_lookup_data.get(&"Prayer".to_string()).unwrap_or(&0) + 
        skill_lookup_data.get(&"Magic".to_string()).unwrap_or(&0);

        let total_xp = skill_xp_lookup_data.get(&"Attack".to_string()).unwrap_or(&0) + 
        skill_xp_lookup_data.get(&"Strength".to_string()).unwrap_or(&0) + 
        skill_xp_lookup_data.get(&"Defence".to_string()).unwrap_or(&0) + 
        skill_xp_lookup_data
            .get(&"Hitpoints".to_string())
            .unwrap_or(&1151) + // HP is level 10
        skill_xp_lookup_data.get(&"Ranged".to_string()).unwrap_or(&0) + 
        skill_xp_lookup_data.get(&"Prayer".to_string()).unwrap_or(&0) + 
        skill_xp_lookup_data.get(&"Magic".to_string()).unwrap_or(&0);

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
        return Ok(vec![format!("{} {}", prefix, skill_data.join(" "))]);
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

fn parse_skill_data_for_cmb(
    skill: &str,
    skill_lookup_data: &HashMap<&String, u32>,
    level_difference: f64,
) -> u32 {
    match skill_lookup_data.get(&skill.to_owned()).unwrap_or(&1) {
        99 => 0.0,
        _ => match skill {
            "Attack" | "Strength" => level_difference / 0.325,
            "Defence" | "Hitpoints" => level_difference / 0.25,
            "Prayer" => level_difference / 0.125,
            _ => level_difference / 0.4875,
        },
    }
    .ceil() as u32
}

fn process_ser_flags(query: &str, mut split: Vec<String>) -> (Vec<String>, (bool, bool, bool)) {
    let re_ser = Regex::new(r"(?:^|\b|\s)-([ser])(?:\s|\b|$)").unwrap();
    let nil = (false, false, false);

    let (output, flag) = re_ser
        .captures(query)
        .map(|capture| {
            let flag = capture.get(1).map_or("", |flag| flag.as_str());
            (
                match flag {
                    "s" => (true, false, false),
                    "e" => (false, true, false),
                    "r" => (false, false, true),
                    _ => nil,
                },
                flag,
            )
        })
        .unwrap_or_else(|| (nil, ""));

    if !flag.is_empty() {
        split.retain(|arg| arg != &format!("-{}", flag));
    }

    (split.into_iter().map(|s| s.to_string()).collect(), output)
}

fn process_filter_by_flags(query: &str, mut split: Vec<String>) -> (Vec<String>, (String, u32)) {
    let re_filter = Regex::new(r"(?:^|\b|\s)([<>=]=?)\s?(\d+)(?:\s|\b|$)").unwrap();
    let nil = ("".to_string(), 0);

    let (flag, filter_at) = re_filter
        .captures(query)
        .map(|capture| {
            let flag = capture.get(1).map_or("", |flag| flag.as_str());
            let filter_at = capture
                .get(2)
                .map_or("", |filter_at| filter_at.as_str())
                .parse::<u32>()
                .unwrap_or(1);
            match flag {
                ">" | "<" | ">=" | "<=" | "=" => (flag.to_string(), u32::max(filter_at, 1)),
                _ => nil.to_owned(),
            }
        })
        .unwrap_or(nil);

    if !flag.is_empty() {
        split.retain(|arg| {
            arg != &flag
                && arg != &filter_at.to_string()
                && arg != &format!("{}{}", flag, filter_at)
        });
    }

    (split, (flag, filter_at))
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

    let (split, flag_prefix, base_url) = process_account_type_flags(query, split);

    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();
    let rsn = if split.is_empty() || split[0].is_empty() {
        get_rsn(author, rsn_n)
            .ok()
            .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
            .unwrap_or_else(|| nick.to_owned())
    } else {
        split.join(" ")
    };

    let stats = match get_stats(&rsn, &base_url) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    let mut prefix_vec = vec![cmd_prefix, &flag_prefix];
    prefix_vec.retain(|x| !x.is_empty());
    let prefix = prefix_vec.join(" ");

    let mut vec: Vec<String> = Vec::new();
    let mut index = 0 - 1 as isize;

    for line in stats {
        index += 1;

        if index - offset >= 0 && index - offset < categories.len() as isize {
            if line[0] == "-1" {
                continue;
            }

            let name: &str = categories[(index - offset) as usize];
            let rank = &line[0];
            let points = &line[1];

            if categories.contains(&name) {
                vec.push(format!(
                    "{}: {} {}",
                    c1(name),
                    c2(&commas_from_string(points, "d")),
                    p(&commas_from_string(rank, "d"))
                ));
            }
        }
    }

    let output = format!("{} {}", prefix, unranked(vec));

    Ok(vec![output])
}
