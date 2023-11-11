use common;
use mysql::{from_row, Row};
use regex::Regex;

pub fn stats(command: &str, query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let skill = common::skill(command);
    let skills = common::skills();
    // Get the skill ID from the skill name
    let skill_id = match skills.iter().position(|r| r.to_string() == skill) {
        Some(index) => index,
        None => 0,
    };

    let prefix = common::l(&skill);
    let not_found = vec![format!(
        "{} {} {} {} {} {} {} {} {}",
        prefix,
        common::c1("Level"),
        common::p("N/A"),
        common::c2("|"),
        common::c1("XP"),
        common::p("N/A"),
        common::c2("|"),
        common::c1("Rank"),
        common::p("N/A")
    )];

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

    let mut rsn: String = split.join(" ").to_string();
    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();
    let row: Vec<Row>;

    if rsn.len() == 0 {
        row = match common::get_rsn(author, rsn_n) {
            Ok(db_rsn) => db_rsn,
            Err(_) => vec![],
        };

        rsn = match row.first() {
            Some(db_rsn) => from_row(db_rsn.to_owned()),
            None => nick, // Default to the user's IRC nickname
        };
    }

    let base_url;

    if flag_ironman {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_ironman/index_lite.ws?player="
    } else if flag_ultimate {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_ultimate/index_lite.ws?player="
    } else if flag_hardcore {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_hardcore_ironman/index_lite.ws?player="
    } else if flag_deadman {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_deadman/index_lite.ws?player="
    } else if flag_leagues {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_seasonal/index_lite.ws?player="
    } else if flag_tournament {
        base_url =
            "https://secure.runescape.com/m=hiscore_oldschool_tournament/index_lite.ws?player="
    } else if flag_1_def {
        base_url =
            "https://secure.runescape.com/m=hiscore_oldschool_skiller_defence/index_lite.ws?player="
    } else if flag_skill {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool_skiller/index_lite.ws?player="
    } else if flag_freshstart {
        base_url =
            "https://secure.runescape.com/m=hiscore_oldschool_fresh_start/index_lite.ws?player="
    } else {
        base_url = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player=";
    }

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
            let actual_level = common::xp_to_level(xp);
            let next_level = actual_level + 1;
            let next_level_xp = common::level_to_xp(next_level);
            let xp_difference = next_level_xp - xp;

            let mut output: Vec<String> = Vec::new();

            let actual_level_string;
            if actual_level > level {
                actual_level_string = format!(" {}", common::p(&actual_level.to_string()));
            } else {
                actual_level_string = "".to_string();
            }

            output.push(format!(
                "{} {}{}",
                common::c1("Level"),
                common::c2(&common::commas_from_string(str_level, "d")),
                actual_level_string
            ));

            output.push(format!(
                "{} {}",
                common::c1("XP"),
                common::c2(&common::commas_from_string(str_xp, "d"))
            ));

            if skill != "Overall" && next_level < 127 {
                output.push(format!(
                    "{} {}",
                    common::c1(&format!("XP to {}", next_level)),
                    common::c2(&common::commas_from_string(&xp_difference.to_string(), "d"))
                ));
            }

            output.push(format!(
                "{} {}",
                common::c1("Rank"),
                common::c2(&common::commas_from_string(rank, "d"))
            ));

            let message = format!("{} {}", prefix, output.join(&common::c1(" | ")));

            return Ok(vec![message]);
        } else if skill_id == 0 && index == 0 {
            // overall
            if split[0] != "-1" && !split[0].contains("404 - Page not found") {
                skill_data.push(format!(
                    "{}{} {}{} {}{}",
                    common::c1("Lvl:"),
                    common::c2(&common::commas_from_string(split[1], "d")),
                    common::c1("XP:"),
                    common::c2(&common::commas_from_string(split[2], "d")),
                    common::c1("Rank:"),
                    common::c2(&common::commas_from_string(split[0], "d")),
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
                let actual_level = common::xp_to_level(xp);
                let next_level = actual_level + 1;
                let next_level_xp = common::level_to_xp(next_level);
                let xp_difference = next_level_xp - xp;

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
                            common::c1(&skills[index as usize]),
                            common::c2(&common::commas(xp as f64, "d")),
                        ));
                    } else if flag_rank {
                        // if -r was passed
                        skill_data.push(format!(
                            "{} {}",
                            common::c1(&skills[index as usize]),
                            common::c2(&common::commas_from_string(rank, "d")),
                        ));
                    } else {
                        // otherwise...
                        skill_data.push(format!(
                            "{} {}",
                            common::c1(&skills[index as usize]),
                            common::c2(&common::commas(level as f64, "d")),
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
        skill_data.push(common::p("XP to Level"));
        for data in sortable_data {
            let xp_difference = data.0 .3;
            let index = data.1;

            skill_data.push(format!(
                "{} {}",
                common::c1(&skills[index as usize]),
                common::c2(&common::commas(xp_difference as f64, "d")),
            ));
        }
    }

    // wrap up the data and return it
    if skill_data.len() > 0 {
        return Ok(vec![format!("{} {}", prefix, skill_data.join(" "))]);
    }

    Ok(not_found)
}
