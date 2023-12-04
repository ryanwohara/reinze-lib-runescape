use crate::common::level_to_xp;
use common::{c1, c2, commas, l};

pub fn xp(query: &str) -> Result<Vec<String>, ()> {
    let split: Vec<&str> = query.split("-").collect();

    if split.len() > 2 || split.len() == 0 {
        return Ok(vec!["Invalid number of arguments".to_string()]);
    } else if split.len() == 2 {
        let start = match split[0].parse::<u32>() {
            Ok(start) => u32::min(start, 126),
            Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
        };

        let end = match split[1].parse::<u32>() {
            Ok(end) => u32::min(end, 126),
            Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
        };

        let output = vec![format!(
            "{} {}->{} = {} xp",
            l("Level->XP"),
            c1(&start.to_string()),
            end,
            c2(&commas((level_to_xp(end) - level_to_xp(start)) as f64, "d").to_string())
        )];

        return Ok(output);
    } else if split.len() == 1 {
        let level = match split[0].parse::<u32>() {
            Ok(level) => level,
            Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
        };

        let output = vec![format!(
            "{} {} = {} xp",
            l("Level->XP"),
            c1(&level.to_string()),
            c2(&commas(level_to_xp(level as u32) as f64, "d"))
        )];

        return Ok(output);
    }

    Err(())
}
