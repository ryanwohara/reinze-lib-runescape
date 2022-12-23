use super::common::c1;
use super::common::c2;
use super::common::l;
use super::common::level_to_xp;
// use super::common::xp_to_level;

pub fn xp(query: &str) -> Result<Vec<String>, ()> {
    let split: Vec<&str> = query.split("-").collect();

    if split.len() > 2 || split.len() == 0 {
        return Ok(vec!["Invalid number of arguments".to_string()]);
    } else if split.len() == 2 {
        let start = match split[0].parse::<u32>() {
            Ok(start) => start,
            Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
        };

        let end = match split[1].parse::<u32>() {
            Ok(end) => end,
            Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
        };

        let output = vec![format!(
            "{}-{}: {}xp",
            start,
            end,
            level_to_xp(end) - level_to_xp(start)
        )];

        return Ok(output);
    } else if split.len() == 1 {
        let level = match split[0].parse::<u32>() {
            Ok(level) => level,
            Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
        };

        let output = vec![format!(
            "{} {} = {}xp",
            l("Level->XP"),
            c1(&level.to_string()),
            c2(&level_to_xp(level).to_string())
        )];

        return Ok(output);
    }

    Err(())
}
