use common::{
    c1, c2, commas_from_string, convert_split_to_string, get_rsn, get_stats, l, p,
    process_account_type_flags, unranked,
};
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

    let bh: [&str; 4] = ["Hunter", "Rogue", "Legacy Hunter", "Legacy Rogue"];

    let split: Vec<String> = convert_split_to_string(query.split(" ").collect());

    let (split, mut prefix, base_url) = process_account_type_flags(query, split);

    let stats = match get_stats(&rsn, &base_url) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    let mut bh_ranks: Vec<String> = Vec::new();
    let mut index = 0 - 1 as isize;
    let offset = 25;

    for line in stats {
        index += 1;

        if index - offset >= 0 && index - offset < bh.len() as isize {
            if line[0] == "-1" {
                continue;
            }

            let name: &str = bh[(index - offset) as usize];
            let rank = &line[0];
            let completions = &line[1];

            if bh.contains(&name) {
                bh_ranks.push(format!(
                    "{}: {} {}",
                    c1(name),
                    c2(&commas_from_string(completions, "d")),
                    p(&commas_from_string(rank, "d"))
                ));
            }
        }
    }

    let output = format!("{} {}", l("Bounty Hunter"), unranked(bh_ranks));

    Ok(vec![output])
}
