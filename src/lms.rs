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

    let lms: [&str; 1] = ["Kills"];

    let split: Vec<String> = convert_split_to_string(query.split(" ").collect());

    let (split, mut prefix, base_url) = process_account_type_flags(query, split);

    let stats = match get_stats(&rsn, &base_url) {
        Ok(stats) => stats,
        Err(_) => return Err(()),
    };

    let lms_rank: Vec<String> = stats
        .iter()
        .enumerate()
        .filter(|(index, line)| {
            let offset = 37;
            let lms_index = *index as isize - offset;
            lms_index >= 0 && lms_index < lms.len() as isize && line[0] != "-1"
        })
        .map(|(index, line)| {
            let name = lms[(index as isize - 37) as usize];
            let rank = &line[0];
            let completions = &line[1];
            format!(
                "{}: {} {}",
                c1(name),
                c2(&commas_from_string(completions, "d")),
                p(&commas_from_string(rank, "d"))
            )
        })
        .collect();

    let output = format!("{} {}", l("Last Man Standing"), unranked(lms_rank));

    Ok(vec![output])
}
