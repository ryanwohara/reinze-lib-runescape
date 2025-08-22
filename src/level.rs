use crate::common::{eval_query, xp_to_level};
use common::{c1, c2, commas, l};

pub fn level(query: &str) -> Result<Vec<String>, ()> {
    let err = vec![format!(
        "{} {}",
        l("XP->Level"),
        c1("You may only use experience within the range 0 - 200,000,000"),
    )];

    let xp = match eval_query(query) {
        Ok(xp) => u32::min(xp as u32, 200000000),
        Err(_) => return Ok(err),
    };

    let output = vec![format!(
        "{} {} = {}",
        l("XP->Level"),
        c1(&commas(xp as f64, "d")),
        c2(&format!("Level {}", &xp_to_level(xp).to_string()))
    )];

    return Ok(output);
}
