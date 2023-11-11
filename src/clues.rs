use common::{c1, c2, commas_from_string, get_rsn, get_stats, l, p};
use mysql::from_row;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();
    let rsn = if query.is_empty() {
        get_rsn(author, rsn_n)
            .ok()
            .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
            .unwrap_or_else(|| nick)
    } else {
        query.to_string()
    };

    let clues: [&str; 7] = [
        "All", "Beginner", "Easy", "Medium", "Hard", "Elite", "Master",
    ];

    let stats = match get_stats(&rsn) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    let mut clue_completions: Vec<String> = Vec::new();
    let mut index = 0 - 1 as isize;
    let offset = 29;

    for line in stats {
        index += 1;

        if index - offset >= 0 && index - offset < clues.len() as isize {
            if line[0] == "-1" {
                continue;
            }

            let name: &str = clues[(index - offset) as usize];
            let rank = &line[0];
            let completions = &line[1];

            if clues.contains(&name) {
                clue_completions.push(format!(
                    "{}: {} {}",
                    c1(name),
                    c2(&commas_from_string(completions, "d")),
                    p(&commas_from_string(rank, "d"))
                ));
            }
        }
    }

    let output = format!("{} {}", l("Clues"), clue_completions.join(&c1(" | ")));

    Ok(vec![output])
}
