use anyhow::Result;
use common::source::Source;

pub fn query(s: &Source) -> Result<Vec<String>> {
    if s.query.is_empty() {
        return Ok(vec!["Usage: !wiki <query>".to_string()]);
    }

    let link = vec![
        "https://oldschool.runescape.wiki/w/Special:Search?search=",
        &s.query.replace(" ", "+"),
    ]
    .join("");

    let output = vec![s.l("Wiki"), s.c2(link)].join(" ");

    Ok(vec![output])
}
