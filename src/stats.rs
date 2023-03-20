use crate::common;

pub fn stats(command: &str, query: &str, author: &str) -> Result<Vec<String>, ()> {
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

    let split: Vec<&str> = query.split(" ").collect();
    let rsn: String;

    if split.len() == 0 {
        rsn = author.to_string();
    } else {
        rsn = query.to_string();
    }

    let resp = match reqwest::blocking::get(&format!(
        "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player={}",
        rsn
    )) {
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

    println!("{:?}", hiscores_collected);

    let mut index = 0 - 1 as isize;
    for split in hiscores_collected {
        index += 1;

        if index as usize == skill_id {
            if split[1] == "-1" || split[0].contains("404 - Page not found") {
                return Ok(not_found);
            }

            let rank = split[0];
            let level = split[1];
            let xp = split[2];
            let actual_level = common::xp_to_level(xp.parse::<u32>().unwrap());
            let next_level = actual_level + 1;
            let next_level_xp = common::level_to_xp(next_level);
            let xp_difference = next_level_xp - xp.parse::<u32>().unwrap();

            let mut output: Vec<String> = Vec::new();

            let actual_level_string;
            if skill != "Overall" && actual_level > level.parse::<u32>().unwrap() {
                actual_level_string = format!(" {}", common::p(&actual_level.to_string()));
            } else {
                actual_level_string = "".to_string();
            }

            output.push(format!(
                "{} {}{}",
                common::c1("Level"),
                common::c2(&common::commas_from_string(level)),
                actual_level_string
            ));

            output.push(format!(
                "{} {}",
                common::c1("XP"),
                common::c2(&common::commas_from_string(xp))
            ));

            if skill != "Overall" && next_level < 127 {
                output.push(format!(
                    "{} {}",
                    common::c1(&format!("XP to {}", next_level)),
                    common::c2(&common::commas_from_string(&xp_difference.to_string()))
                ));
            }

            output.push(format!(
                "{} {}",
                common::c1("Rank"),
                common::c2(&common::commas_from_string(rank))
            ));

            let message = format!("{} {}", prefix, output.join(&common::c1(" | ")));

            return Ok(vec![message]);
        }
    }

    Ok(not_found)
}
