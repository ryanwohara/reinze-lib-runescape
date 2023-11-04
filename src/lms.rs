use crate::common::c1;
use crate::common::c2;
use crate::common::commas_from_string;
use crate::common::get_rsn;
use crate::common::get_stats;
use crate::common::l;
use crate::common::p;
use mysql::{from_row, Row};

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let row: Vec<Row>;
    let mut rsn: String = query.to_string();
    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();

    if rsn.len() == 0 {
        row = match get_rsn(author, rsn_n) {
            Ok(db_rsn) => db_rsn,
            Err(_) => vec![],
        };

        rsn = match row.first() {
            Some(db_rsn) => from_row(db_rsn.to_owned()),
            None => nick, // Default to the user's IRC nickname
        };
    }

    let lms: [&str; 1] = ["Kills"];

    let stats = match get_stats(&rsn) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    let mut lms_rank: Vec<String> = Vec::new();
    let mut index = 0 - 1 as isize;
    let offset = 37;

    for line in stats {
        index += 1;

        if index - offset >= 0 && index - offset < lms.len() as isize {
            if line[0] == "-1" {
                continue;
            }

            let name: &str = lms[(index - offset) as usize];
            let rank = &line[0];
            let completions = &line[1];

            if lms.contains(&name) {
                lms_rank.push(format!(
                    "{}: {} {}",
                    c1(name),
                    c2(&commas_from_string(completions)),
                    p(&commas_from_string(rank))
                ));
            }
        }
    }

    let output = format!("{} {}", l("Last Man Standing"), lms_rank.join(&c1(" | ")));

    Ok(vec![output])
}
