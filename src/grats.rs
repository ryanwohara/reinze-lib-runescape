extern crate common;
use crate::common::skill as get_skill;
use meval::eval_str;
use regex::Regex;

pub fn get(query: &str, author: &str) -> Result<Vec<String>, ()> {
    let mut split = query.split(" ");

    let first_token = split.next().unwrap_or_default();
    let second_token = split.next().unwrap_or_default();
    let third_token = split.next().unwrap_or_default();

    let mut nick = author.split("!").collect::<Vec<&str>>()[0];
    let mut milestone = first_token;
    let mut skill_token = second_token;

    let err = Ok(vec!["Syntax: +congrats [nick] (level) (skill)".to_string()]);

    if third_token.len() > 0 {
        nick = first_token;
        milestone = second_token;
        skill_token = third_token;
    } else if second_token.len() == 0 {
        return err;
    }

    let skill = &get_skill(skill_token);

    if skill.len() == 0 {
        return err;
    }

    let re = Regex::new(r"^(\d+)[kmb]?$").unwrap();
    _ = match re.captures(milestone) {
        Some(captures) => vec![captures],
        None => return err,
    };

    let processed_milestone = eval_query(milestone.replace(",", ""))? as u32;
    let comma_milestone = common::commas(processed_milestone as f64, "d");

    let output: String;

    if skill == "Overall" {
        output = format!(
            "{0}: Congratulations on {1} {2}! Pretty impresive!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 40 {
        output = format!(
            "{0}: grats on {1} {2}! Keep up the good work.",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 55 {
        output = format!(
            "{0}: getting somewhere! Grats on {1} {2}!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 70 {
        output = format!(
            "{0}: awesome! Congratulations on {1} {2}!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 85 {
        output = format!(
            "{0}: you are a CHAMPION! Congratulations on {1} {2}!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 95 {
        output = format!(
            "{0}: you are one of the elite! Congratulations on {1} {2}!!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone < 99 {
        output = format!(
            "{0}: I am not worthy! Congratulations on {1} {2}!!!!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone > 98 && processed_milestone <= 150 {
        output = format!(
            "{0}: \\o/ CONGRATULATIONS ON {1} {2}! You are a true Runescaper!",
            nick, comma_milestone, skill
        )
    // We'll just assume it's XP
    } else if processed_milestone <= 1000000 {
        output = format!(
            "{0}: Congratulations on {1} {2} xp! Pretty impresive!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 2500000 {
        output = format!(
            "{0}: Hey congratulations on {1} {2} xp! Moving on up!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 5000000 {
        output = format!(
            "{0}: Congrats on {1} {2} xp! Almost halfway to 92!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 7500000 {
        output = format!(
            "{0}: More than halfway there! Keep on trucking! Congratulations for {1} {2} xp!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 10000000 {
        output = format!(
            "{0}: ALMOST TO 99! Congratulations for reaching {1} {2} xp! *jealous*",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 15000000 {
        output = format!(
            "{0}: IMPRESSIVE WORK! You must really love {2}. Congrats on {1} {2} xp.",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 20000000 {
        output = format!(
            "{0}: WOW congratulations on {1} {2} xp! Go get yourself a snack. You earned it.",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 50000000 {
        output = format!(
            "{0}: I\"m jealous of your {1} {2} xp! Congrats though!",
            nick, comma_milestone, skill
        )
    } else if processed_milestone <= 100000000 {
        output = format!("{0}: You might be insane! Incredible congratulations on {1} xp! Everyone else is super jelly of your {2} skillz.", nick, comma_milestone, skill)
    } else if processed_milestone < 200000000 {
        output = format!("{0}: I have no more words for you. I am Hulk green with envy. Go train {1} more you beast. (Okay, congrats on {2} xp!)", nick, skill, comma_milestone)
    } else if processed_milestone == 200000000 {
        output = format!("{0}: Okay, you win. You are on the highscores forever. Endless congratulations on maxing {1}. Go get some sunshine and a nice snack to celebrate!", nick, skill)
    } else {
        output = format!("{0}: That is not even a thing, get out of here.", nick)
    }

    Ok(vec![output])
}

fn eval_query(query: String) -> Result<f64, ()> {
    let re_kmb = Regex::new(r"(?P<num>[\d.]+)(?P<kmb>[kmb])").unwrap();
    let processed = re_kmb.replace_all(&query, replace_all).to_string();

    eval_str(&processed).map_err(|e| {
        println!("Error: {}", e);
        ()
    })
}

fn replace_all(caps: &regex::Captures) -> String {
    let (num, kmb) = (
        caps.name("num").unwrap().as_str(),
        caps.name("kmb").unwrap().as_str(),
    );
    let mut num = num.parse::<f64>().unwrap_or_default();

    if let Some(factor) = match kmb {
        "k" => Some(1_000.0),
        "m" => Some(1_000_000.0),
        "b" => Some(1_000_000_000.0),
        _ => None,
    } {
        num *= factor;
    }
    num.to_string()
}
