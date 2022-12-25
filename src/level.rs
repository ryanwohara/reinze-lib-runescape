use super::common::c1;
use super::common::c2;
use super::common::l;
use super::common::xp_to_level;

pub fn level(query: &str) -> Result<Vec<String>, ()> {
    let err = vec![format!(
        "{} {}",
        l("XP->Level"),
        c1("You may only use experience within the range 0 - 200,000,000"),
    )];

    let xp = match query.parse::<u32>() {
        Ok(xp) => xp,
        Err(_) => return Ok(err),
    };

    if xp > 200000000 {
        return Ok(err);
    }

    let output = vec![format!(
        "{} {} = {}",
        l("XP->Level"),
        c1(query),
        c2(&format!("Level {}", &xp_to_level(xp).to_string()))
    )];

    return Ok(output);
}
