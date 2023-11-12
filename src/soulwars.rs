use common::{
    c1, c2, commas_from_string, convert_split_to_string, get_rsn, get_stats, l, p,
    process_account_type_flags, unranked,
};
use mysql::from_row;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    let sw: [&str; 1] = ["Points"];

    let split: Vec<String> = convert_split_to_string(query.split(" ").collect());

    let (split, mut prefix, base_url) = process_account_type_flags(query, split);

    let nick = author.split("!").collect::<Vec<&str>>()[0].to_string();
    let rsn = if split.is_empty() || split[0].is_empty() {
        get_rsn(author, rsn_n)
            .ok()
            .and_then(|db_rsn| db_rsn.first().map(|db_rsn| from_row(db_rsn.to_owned())))
            .unwrap_or_else(|| nick.to_owned())
    } else {
        split.join(" ")
    };

    let stats = match get_stats(&rsn, &base_url) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    prefix = vec![l("Soul Wars"), prefix].join(" ").replace("  ", " ");

    let mut zeal: Vec<String> = Vec::new();
    let mut index = 0 - 1 as isize;
    let offset = 39;

    for line in stats {
        index += 1;

        if index - offset >= 0 && index - offset < sw.len() as isize {
            if line[0] == "-1" {
                continue;
            }

            let name: &str = sw[(index - offset) as usize];
            let rank = &line[0];
            let points = &line[1];

            if sw.contains(&name) {
                zeal.push(format!(
                    "{}: {} {}",
                    c1(name),
                    c2(&commas_from_string(points, "d")),
                    p(&commas_from_string(rank, "d"))
                ));
            }
        }
    }

    let output = format!("{} {}", prefix, unranked(zeal));

    Ok(vec![output])
}
