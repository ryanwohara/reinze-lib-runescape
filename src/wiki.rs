pub fn wiki(query: &str) -> Result<Vec<String>, ()> {
    let mut output = Vec::new();
    let mut url = String::new();
    url.push_str("https://oldschool.runescape.wiki/w/Special:Search?search=");
    url.push_str(&query.replace(" ", "+"));

    Ok(output)
}
