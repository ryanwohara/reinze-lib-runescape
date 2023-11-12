use common::{
    c1, c2, commas, commas_from_string, convert_split_to_string, get_cmb, get_rsn, l, level_to_xp,
    p, process_account_type_flags, skill, skills, unranked, xp_to_level,
};
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

    if flag_exp {
        prefix = vec![l(&skill), prefix, p("XP")].join(" ");
    } else if flag_rank {
        prefix = vec![l(&skill), prefix, p("Rank")].join(" ");
    } else if flag_sort {
        prefix = vec![l(&skill), prefix, p("XP to Level")].join(" ");
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

    for split in hiscores_collected {
        index += 1;

        if skill_id != 0 && index as usize == skill_id {
            // individual skill
            if split[1] == "-1" || split[0].contains("404 - Page not found") {
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
                            "{} {}",
                            c1(&skills[index as usize]),
                            c2(&commas(xp as f64, "d")),
                        ));
                    } else if flag_rank {
                        // if -r was passed
                        skill_data.push(format!(
                            "{} {}",
                            c1(&skills[index as usize]),
                            c2(&commas_from_string(rank, "d")),
                        ));
                    } else {
                        // otherwise...
                        skill_data.push(format!(
                            "{} {}",
                            c1(&skills[index as usize]),
                            c2(&commas(level as f64, "d")),
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
                c1(&skills[index as usize]),
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

        skill_data.insert(1, format!("{}{}", c1("Combat:"), c2(&cmb.to_string())));
    }

    // wrap up the data and return it
    if skill_data.len() > 0 {
        return Ok(vec![format!("{} {}", prefix, skill_data.join(" "))]);
    }

    Ok(not_found)
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

pub fn get_stats_subsection(query: &str, author: &str, rsn_n: &str) -> Vec<Vec<String>> {
    vec![vec!["".to_owned()]]
}
