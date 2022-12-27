use crate::common;

pub fn stats(command: &str, query: &str, author: &str) -> Result<Vec<String>, ()> {
    let skill = common::skill(command);
    let skills = common::skills();
    // Get the skill ID from the skill name
    let skill_id = match skills.iter().position(|r| r.to_string() == skill) {
        Some(index) => index,
        None => 0,
    };
    let not_found = vec![format!(
        "{} {} {} {} {} {} {} {} {}",
        common::l(&skill),
        common::c1("Level"),
        common::c2("N/A"),
        common::c2("|"),
        common::c1("XP"),
        common::c2("N/A"),
        common::c2("|"),
        common::c1("Rank"),
        common::c2("N/A")
    )];

    let split: Vec<&str> = query.split(" ").collect();
    let rsn: String;

    if split.len() == 0 {
        rsn = author.to_string();
    } else {
        rsn = query.to_string();
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    let resp = match rt.block_on(reqwest::get(&format!(
        "https://secure.runescape.com/m=hiscore_oldschool/index_lite.ws?player={}",
        rsn
    ))) {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error making HTTP request: {}", e);
            return Err(());
        }
    };

    let string = match rt.block_on(resp.text()) {
        Ok(string) => string,
        Err(e) => {
            println!("Error getting text: {}", e);
            return Err(());
        }
    };

    let hiscores_split = string.split('\n').collect::<Vec<&str>>();
    let mut index = 0 - 1 as isize;
    for line in hiscores_split {
        index += 1;

        if index as usize == skill_id {
            let split: Vec<&str> = line.split(',').collect();

            if split[0] == "-1" || split[0] == "<!DOCTYPE html><html><head><title>404 - Page not found</title> <meta charset=\"UTF-8\"/><meta name=\"viewport\" content=\"width=device-width"{
                return Ok(not_found);
            }

            let rank = split[0];
            let level = split[1];
            let xp = split[2];

            let output = format!(
                "{} {} {} {} {} {} {} {} {}",
                common::l(&skill),
                common::c1("Level"),
                common::c2(&common::commas_from_string(level)),
                common::c2("|"),
                common::c1("XP"),
                common::c2(&common::commas_from_string(xp)),
                common::c2("|"),
                common::c1("Rank"),
                common::c2(&common::commas_from_string(rank))
            );

            return Ok(vec![output]);
        }
    }

    Ok(not_found)
}
