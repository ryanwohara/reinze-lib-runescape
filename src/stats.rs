use common::{
    c1, c2, commas, commas_from_string, get_cmb, get_rsn, l, level_to_xp, p, skill, skills,
    unranked, xp_to_level,
};
use mysql::from_row;
use regex::Regex;
use std::collections::HashMap;

pub fn stats(command: &str, query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let skill = skill(command);
    let skills = skills();
    // Get the skill ID from the skill name
    let skill_id = match skills.iter().position(|r| r.to_string() == skill) {
        Some(index) => index,
        None => 0,
    };

    let mut split: Vec<&str> = query.split(" ").collect();

    let mut flag_sort = false;
    let mut flag_exp = false;
    let mut flag_rank = false;
    let mut flag_ironman = false;
    let mut flag_ultimate = false;
    let mut flag_hardcore = false;
    let mut flag_deadman = false;
    let mut flag_leagues = false;
    let mut flag_tournament = false;
    let mut flag_1_def = false;
    let mut flag_skill = false;
    let mut flag_freshstart = false;
    let mut flag_filter_by = "";
    let mut flag_filter_at = 1;

    let re_ser = Regex::new(r"(?:^|\b)-([seriuhdlt1]|sk|fs)(?:\b|$)").unwrap();
    let re_ser_match = match re_ser.captures(query) {
        Some(captures) => vec![captures],
        None => vec![],
    };

    if re_ser_match.len() > 0 {
        let flag = match re_ser_match[0].get(1) {
            Some(flag) => flag.as_str(),
            None => "",
        };
        match flag {
            "s" => flag_sort = true,
            "e" => flag_exp = true,
            "r" => flag_rank = true,
            "i" => flag_ironman = true,
            "u" => flag_ultimate = true,
            "h" => flag_hardcore = true,
            "d" => flag_deadman = true,
            "l" => flag_leagues = true,
            "t" => flag_tournament = true,
            "1" => flag_1_def = true,
            "sk" => flag_skill = true,
            "fs" => flag_freshstart = true,
            _ => (),
        };
        for (index, arg) in split.iter().enumerate() {
            if arg.eq(&format!("-{}", flag)) {
                split.remove(index);
                break;
            }
        }
    }

    let re_filter = Regex::new(r"([<>=]=?)\s?(\d+)").unwrap();
    let re_filter_match = match re_filter.captures(query) {
        Some(captures) => vec![captures],
        None => vec![],
    };

    if re_filter_match.len() > 0 {
        flag_filter_by = re_filter_match[0].get(1).unwrap().as_str();
        flag_filter_at = re_filter_match[0]
            .get(2)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap_or(1);
        for (index, arg) in split.iter().enumerate() {
            if arg.eq(&flag_filter_by) {
                split.remove(index);
                split.remove(index);
                break;
            } else if arg.eq(&format!("{}{}", flag_filter_by, flag_filter_at)) {
                split.remove(index);
                break;
            }
        }
    }

    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();
    let rsn = if split.is_empty() || split[0].is_empty() {
        get_rsn(author, rsn_n)
            .ok()
            .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
            .unwrap_or_else(|| nick.to_owned())
    } else {
        split.join(" ")
    };

    println!("{} => {}", nick, rsn);

    let base_url;
    let mut prefix;

    if flag_ironman {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_ironman/index_lite.ws?player=";
        prefix = vec![l("Iron"), l(&skill)].join(" ");
    } else if flag_ultimate {
        base_url =
            "https://secure.runescape.com/m=hiscore_oldschool_ultimate/index_lite.ws?player=";
        prefix = vec![l("Ultimate"), l(&skill)].join(" ");
    } else if flag_hardcore {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_hardcore_ironman/index_lite.ws?player=";
        prefix = vec![l("Hardcode"), l(&skill)].join(" ");
    } else if flag_deadman {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_deadman/index_lite.ws?player=";
        prefix = vec![l("Deadman"), l(&skill)].join(" ");
    } else if flag_leagues {
        base_url =
            "https://secure.runescape.com/m=hiscore_oldschool_seasonal/index_lite.ws?player=";
        prefix = vec![l("Seasonal"), l(&skill)].join(" ");
    } else if flag_tournament {
        base_url =
            "https://secure.runescape.com/m=hiscore_oldschool_tournament/index_lite.ws?player=";
        prefix = vec![l("Tournament"), l(&skill)].join(" ");
    } else if flag_1_def {
        base_url =
            "https://secure.runescape.com/m=hiscore_oldschool_skiller_defence/index_lite.ws?player=";
        prefix = vec![l("1 Def"), l(&skill)].join(" ");
    } else if flag_skill {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_skiller/index_lite.ws?player=";
        prefix = vec![l("Skiller"), l(&skill)].join(" ");
    } else if flag_freshstart {
        base_url =
            "https://secure.runescape.com/m=hiscore_oldschool_fresh_start/index_lite.ws?player=";
        prefix = vec![l("Fresh Start"), l(&skill)].join(" ");
    } else {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=";
        prefix = l(&skill);
    }

    if flag_exp {
        prefix = vec![prefix, p("XP")].join(" ");
    } else if flag_rank {
        prefix = vec![prefix, p("Rank")].join(" ");
    } else if flag_sort {
        prefix = vec![prefix, p("XP to Level")].join(" ");
    } else {
        prefix = vec![prefix, p("Level")].join(" ");
    }

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
    } else if skill_id == 0 {
        println!("{:?}", skill_lookup_data);
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
