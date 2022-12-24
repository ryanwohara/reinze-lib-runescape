use super::common::c1;
use super::common::c2;
use super::common::l;
use super::common::xp_to_level;

pub fn level(query: &str) -> Result<Vec<String>, ()> {
    let xp = match query.parse::<u32>() {
        Ok(xp) => xp,
        Err(_) => return Ok(vec!["Invalid arguments".to_string()]),
    };

    let output = vec![format!(
        "{} {} = {}",
        l("XP->Level"),
        c1(&xp.to_string()),
        c2(&format!("Level {}", &xp_to_level(xp).to_string()))
    )];

    return Ok(output);
}
