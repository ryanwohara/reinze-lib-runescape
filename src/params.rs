extern crate ini;

use crate::common::skill as common_skill;
use anyhow::Result;
use common::source::Source;
use common::capitalize;
use ini::Ini;
use log::error;

pub fn lookup(s: &Source) -> Result<Vec<String>> {
    let prefix = s.l("Params");

    let (skill, param) = match s.query.split_once(" ") {
        Some((skill, param)) if !skill.is_empty() && !param.is_empty() => {
            (common_skill(skill), param)
        }
        _ => {
            return Ok(vec![format!(
                "{} {}",
                prefix,
                s.c2("Invalid number of arguments")
            )]);
        }
    };

    if skill.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, s.c2("Invalid skill"))]);
    }

    let database = Ini::load_from_file("lib/Database.ini").map_err(|e| {
        error!("Error loading Database.ini: {}", e);
        anyhow::anyhow!("Error loading Database.ini: {}", e)
    })?;

    let prefix = s.l(&capitalize(&skill));

    let section = match database.section(Some(capitalize(&skill))) {
        Some(section) => section,
        _ => return Ok(vec![format!("{} {}", prefix, s.c1("No results found"))]),
    };

    let underscored = param.replace(" ", "_");

    let found_params: Vec<String> = section
        .iter()
        .filter(|(k, _)| {
            k.to_ascii_lowercase()
                .contains(&underscored.to_ascii_lowercase())
        })
        .take(10)
        .map(|(k, v)| {
            format!(
                "{} {}",
                s.c1(&k.replace("_", " ")),
                s.c2(&format!("{}xp", v))
            )
        })
        .collect();
    Ok(vec![format!("{} {}", prefix, s.not_found(found_params))])
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::ColorResult;
    use common::author::Author;
    use std::ffi::CString;
    use std::os::raw::c_char;

    /// Returns distinctive colors so a hard-coded default is easy to spot.
    /// `Author::colors` takes ownership of both pointers and frees them, so
    /// these must be freshly allocated on every call.
    extern "C" fn stub_color(_host: *const c_char, _colors: *const c_char) -> ColorResult {
        ColorResult {
            c1: CString::new("07").unwrap().into_raw(),
            c2: CString::new("13").unwrap().into_raw(),
        }
    }

    fn source_with(query: &str) -> Source {
        Source::create(
            "0",
            Author::create("nick!ident@host", stub_color),
            "params",
            query,
        )
    }

    fn assert_caller_colors(text: &str) {
        assert!(text.contains("\x0307"), "expected caller c1 (07) in: {text:?}");
        assert!(text.contains("\x0313"), "expected caller c2 (13) in: {text:?}");
        assert!(
            !text.contains("\x0314"),
            "hard-coded default c1 (14) leaked into: {text:?}"
        );
        assert!(
            !text.contains("\x0304"),
            "hard-coded default c2 (04) leaked into: {text:?}"
        );
    }

    // Both cases return before the Database.ini load, so these stay offline.

    #[test]
    fn params_bad_arguments_use_the_callers_colors() {
        let out = lookup(&source_with("")).unwrap();
        assert_caller_colors(&out[0]);
    }

    #[test]
    fn params_invalid_skill_uses_the_callers_colors() {
        let out = lookup(&source_with("notaskill somequery")).unwrap();
        assert_caller_colors(&out[0]);
    }
}
