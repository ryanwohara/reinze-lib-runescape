mod bosses;
mod common;
mod ge;
mod items;
mod level;
mod params;
mod patch;
mod players;
mod prices;
mod xp;

#[no_mangle]
pub extern "C" fn exported(command: &str, query: &str, author: &str) -> Result<Vec<String>, ()> {
    match command {
        "boss" => bosses::bosses(query),
        "exp" => xp::xp(query),
        "experience" => xp::xp(query),
        "ge" => ge::ge(query),
        "level" => level::level(query),
        "lvl" => level::level(query),
        "params" => params::params(query),
        "patch" => patch::patch(query),
        "players" => players::players(),
        "price" => prices::prices(query),
        "xp" => xp::xp(query),
        "" => Ok(vec![
            "boss".to_string(),
            "exp".to_string(),
            "experience".to_string(),
            "ge".to_string(),
            "level".to_string(),
            "lvl".to_string(),
            "params".to_string(),
            "patch".to_string(),
            "players".to_string(),
            "price".to_string(),
            "xp".to_string(),
        ]),
        _ => Ok(vec![]),
    }
}
