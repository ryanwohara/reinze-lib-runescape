mod bosses;
mod common;
mod ge;
mod items;
mod level;
mod params;
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
            "players".to_string(),
            "price".to_string(),
            "xp".to_string(),
        ]),
        _ => Ok(vec![]),
    }
}

#[no_mangle]
pub extern "C" fn boss(rsn: &str) -> Result<Vec<String>, ()> {
    bosses::bosses(rsn)
}

#[no_mangle]
pub extern "C" fn ge(query: &str) -> Result<Vec<String>, ()> {
    ge::ge(query)
}

#[no_mangle]
pub extern "C" fn level(query: &str) -> Result<Vec<String>, ()> {
    level::level(query)
}

#[no_mangle]
pub extern "C" fn params(query: &str) -> Result<Vec<String>, ()> {
    params::params(query)
}

#[no_mangle]
pub extern "C" fn players() -> Result<Vec<String>, ()> {
    players::players()
}

#[no_mangle]
pub extern "C" fn prices(query: &str) -> Result<Vec<String>, ()> {
    prices::prices(query)
}

#[no_mangle]
pub extern "C" fn xp(rsn: &str) -> Result<Vec<String>, ()> {
    xp::xp(rsn)
}
