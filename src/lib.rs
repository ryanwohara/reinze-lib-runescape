pub mod bosses;
pub mod common;
pub mod ge;
mod items;
pub mod params;
pub mod players;
pub mod prices;
pub mod xp;

#[no_mangle]
pub extern "C" fn exported(command: &str, query: &str) -> Result<Vec<String>, ()> {
    match command {
        "players" => players::players(),
        "params" => params::params(query),
        "price" => prices::prices(query),
        "ge" => ge::ge(query),
        "boss" => bosses::bosses(query),
        "xp" => xp::xp(query),
        "" => Ok(vec![
            "players".to_string(),
            "params".to_string(),
            "price".to_string(),
            "ge".to_string(),
            "boss".to_string(),
            "xp".to_string(),
        ]),
        _ => Ok(vec![]),
    }
}

#[no_mangle]
pub extern "C" fn players() -> Result<Vec<String>, ()> {
    players::players()
}

#[no_mangle]
pub extern "C" fn params(query: &str) -> Result<Vec<String>, ()> {
    params::params(query)
}

#[no_mangle]
pub extern "C" fn prices(query: &str) -> Result<Vec<String>, ()> {
    prices::prices(query)
}

#[no_mangle]
pub extern "C" fn ge(query: &str) -> Result<Vec<String>, ()> {
    ge::ge(query)
}

#[no_mangle]
pub extern "C" fn boss(rsn: &str) -> Result<Vec<String>, ()> {
    bosses::bosses(rsn)
}

#[no_mangle]
pub extern "C" fn xp(rsn: &str) -> Result<Vec<String>, ()> {
    xp::xp(rsn)
}
