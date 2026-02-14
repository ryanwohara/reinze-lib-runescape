use common::source::Source;

pub fn query(s: &Source) -> Result<Vec<String>, ()> {
    let link = vec![
        "https://oldschool.runescape.wiki/w/Special:Search?search=",
        &s.query.replace(" ", "+"),
    ]
    .join("");

    let output = vec![s.l("Wiki"), s.c2(link)].join(" ");

    Ok(vec![output])
}
