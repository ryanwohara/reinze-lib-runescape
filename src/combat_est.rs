use crate::common::HiscoreName::{Attack, Defence, Hitpoints, Magic, Prayer, Ranged, Strength};
use crate::common::{Listing, Listings, Stats, eval_query};
use crate::stats::{stats_parameters, strip_stats_parameters};
use anyhow::Result;
use common::{commas, source::Source};
use regex::{Match, Regex};

struct CmbEst {
    pub a: Option<Listing>,
    pub s: Option<Listing>,
    pub d: Option<Listing>,
    pub p: Option<Listing>,
    pub h: Option<Listing>,
    pub r: Option<Listing>,
    pub m: Option<Listing>,
}

impl CmbEst {
    pub fn new() -> Self {
        Self {
            a: None,
            s: None,
            d: None,
            p: None,
            h: None,
            r: None,
            m: None,
        }
    }

    pub fn calc(self) -> Listings {
        let attack = self.a.unwrap_or(Listing::set_level(Attack, 1));
        let strength = self.s.unwrap_or(Listing::set_level(Strength, 1));
        let defence = self.d.unwrap_or(Listing::set_level(Defence, 1));
        let prayer = self.p.unwrap_or(Listing::set_level(Prayer, 1));
        let hitpoints = self.h.unwrap_or(Listing::set_level(Hitpoints, 10));
        let ranged = self.r.unwrap_or(Listing::set_level(Ranged, 1));
        let magic = self.m.unwrap_or(Listing::set_level(Magic, 1));

        Listings::new(vec![
            attack, strength, defence, prayer, hitpoints, ranged, magic,
        ])
    }
}

fn parse(input: Option<Option<Match>>) -> String {
    match input {
        Some(result) => match result {
            Some(number) => number.as_str(),
            None => "",
        },
        None => "",
    }
    .to_string()
}

pub fn estimate(s: Source) -> Result<Vec<String>> {
    let prefix = s.l("Combat Estimation");
    let mut cmbest = CmbEst::new();

    let flags = stats_parameters(&s.query);

    let re = Regex::new(r"^(\d{1,2})([ASDRMPHasdrmph])$").unwrap();
    strip_stats_parameters(&s.query)
        .split_whitespace()
        .for_each(|token| {
            let captures = match re.captures(token) {
                Some(captured) => captured,
                None => return,
            };

            let mut iter = captures.iter();
            let _total_match = iter.next();

            let number = match eval_query(parse(iter.next())) {
                Ok(num) => num as u32,
                _ => return,
            };
            let letter = parse(iter.next()).to_lowercase();

            match letter.as_str() {
                "a" => cmbest.a = Some(Listing::set_level(Attack, number)),
                "s" => cmbest.s = Some(Listing::set_level(Strength, number)),
                "d" => cmbest.d = Some(Listing::set_level(Defence, number)),
                "p" => cmbest.p = Some(Listing::set_level(Prayer, number)),
                "h" => cmbest.h = Some(Listing::set_level(Hitpoints, number)),
                "r" => cmbest.r = Some(Listing::set_level(Ranged, number)),
                "m" => cmbest.m = Some(Listing::set_level(Magic, number)),
                _ => (),
            };
        });

    let hiscores = cmbest.calc();

    let stats = Stats {
        flags,
        hiscores,
        source: s,
    };
    let combat = stats.combat();

    let total_level: u32 = stats.hiscores.iter().map(|listing| listing.level()).sum();
    let total_lvl_str = vec![
        stats.source.c1("Levels:"),
        stats.source.c2(&commas(total_level as f64, "d")),
    ]
    .join(" ");

    let total_xp: u32 = stats.hiscores.iter().map(|listing| listing.xp()).sum();
    let total_xp_str = vec![
        stats.source.c1("XP:"),
        stats.source.c2(&commas(total_xp as f64, "d")),
    ]
    .join(" ");
    let total_str = &vec![total_lvl_str, total_xp_str].join(&stats.source.c1(" | "));

    let summary = &stats
        .hiscores
        .iter()
        .map(|listing| {
            vec![
                stats
                    .source
                    .c1(&vec![&listing.name().to_string(), ":"].join("")),
                stats.source.c2(&listing.level().to_string()),
            ]
            .join("")
        })
        .collect::<Vec<String>>()
        .join(" ");

    let mut calculations = combat.calc(&stats);
    calculations.retain(|(_string, int)| int > &0u32);
    let calc = &calculations
        .iter()
        .map(|(string, int)| {
            vec![
                stats.source.c1(&vec![string, ":"].join("")),
                stats.source.c2(&int.to_string()),
            ]
            .join("")
        })
        .collect::<Vec<String>>()
        .join(" ");

    let output = vec![
        prefix,
        combat.to_string(&stats.source),
        stats.source.c1("Total Combat"),
        stats.source.l(total_str),
        stats.source.c1("To Next Level:"),
        stats.source.p(calc),
        stats.source.c1("Current Levels:"),
        stats.source.p(summary),
    ]
    .join(" ");

    Ok(vec![output])
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
            "cmb-est",
            query,
        )
    }

    #[test]
    fn cmb_est_output_uses_the_callers_colors() {
        let out = estimate(source_with("99a 99s 90d 70h 43p 1r 1m")).unwrap();
        let text = &out[0];

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
}
