use common::{
    c1, c2, commas_from_string, convert_split_to_string, get_rsn, get_stats, l, p,
    process_account_type_flags, unranked,
};
use mysql::from_row;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();

    let rifts: [&str; 1] = ["Closed"];

    let split: Vec<String> = convert_split_to_string(query.split(" ").collect());

    let (split, mut prefix, base_url) = process_account_type_flags(query, split);

    let rsn = if query.is_empty() {
        get_rsn(author, rsn_n)
            .ok()
            .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
            .unwrap_or_else(|| nick)
    } else {
        query.to_string()
    };

    let stats = match get_stats(&rsn, &base_url) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    prefix = vec![l("Guardians of the Rift"), prefix]
        .join(" ")
        .replace("  ", " ");

    let mut rifts_closed: Vec<String> = Vec::new();
    let mut index = 0 - 1 as isize;
    let offset = 40;

    for line in stats {
        index += 1;

        if index - offset >= 0 && index - offset < rifts.len() as isize {
            if line[0] == "-1" {
                continue;
            }

            let name: &str = rifts[(index - offset) as usize];
            let rank = &line[0];
            let completions = &line[1];

            if rifts.contains(&name) {
                rifts_closed.push(format!(
                    "{}: {} {}",
                    c1(name),
                    c2(&commas_from_string(completions, "d")),
                    p(&commas_from_string(rank, "d"))
                ));
            }
        }
    }

    let output = format!("{} {}", prefix, unranked(rifts_closed));

    Ok(vec![output])
}
