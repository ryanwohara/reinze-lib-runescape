use common::{c1, c2, l};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WorldInfo {
    world_number: u32,
    hits: u32,
    stream_order: String,
}

pub fn world() -> Result<Vec<String>, ()> {
    let url = "https://togcrowdsourcing.com/worldinfo";

    let worlds: Vec<WorldInfo> = match reqwest::blocking::get(url) {
        Ok(response) => match response.json() {
            Ok(json) => json,
            _ => return Err(()),
        },
        _ => return Err(()),
    };

    let mut matching: Vec<u32> = worlds
        .into_iter()
        .filter(|w| w.stream_order == "gggbbb")
        .map(|w| w.world_number)
        .collect();

    matching.sort_unstable();

    let sorted = matching
        .iter()
        .map(|m| c2(&m.to_string()))
        .collect::<Vec<String>>()
        .join(vec![&c1(","), " "].join("").as_str());

    let output = vec![
        l("ToG Worlds"),
        sorted,
        c1("|| https://togcrowdsourcing.com"),
    ]
    .join(" ");

    Ok(vec![output])
}
