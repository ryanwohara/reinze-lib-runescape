use anyhow::Result;
use common::source::Source;

pub fn query(s: &Source) -> Result<Vec<String>> {
    if s.query.is_empty() {
        return Ok(vec!["Usage: +wiki <query>".to_string()]);
    }

    let link = vec![
        "https://oldschool.runescape.wiki/w/Special:Search?search=",
        &s.query.replace(" ", "+"),
    ]
    .join("");

    let output = vec![s.l("Wiki"), s.c2(link)].join(" ");

    Ok(vec![output])
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::common::ColorResult;
    use ::common::author::Author;
    use std::os::raw::c_char;

    extern "C" fn stub_color(_host: *const c_char, _colors: *const c_char) -> ColorResult {
        ColorResult::default()
    }

    fn stub_source(query: &str) -> Source {
        Source::create("0", Author::create("nick!ident@host", stub_color), "wiki", query)
    }

    /// The bot answers to `-` and `+`, never `!` - the usage line said `!wiki`,
    /// which is a prefix no one can type.
    #[test]
    fn the_usage_line_names_a_prefix_the_bot_answers_to() {
        let lines = query(&stub_source("")).expect("wiki renders");

        assert_eq!(lines, vec!["Usage: -wiki <query>"]);
        assert!(!lines[0].contains('!'), "'!' is not a prefix this bot takes");
    }

    #[test]
    fn a_query_becomes_a_wiki_search_link() {
        let lines = query(&stub_source("abyssal whip")).expect("wiki renders");

        assert!(
            lines[0].contains(
                "https://oldschool.runescape.wiki/w/Special:Search?search=abyssal+whip"
            ),
            "got: {}",
            lines[0]
        );
    }
}
