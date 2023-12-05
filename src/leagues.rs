use super::stats::process_stats_subsection;
use common::l;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 24;

    let categories: Vec<&str> = vec!["Points"];

    let query = format!("{} -l", query);

    process_stats_subsection(&query, author, rsn_n, &l("Leagues"), categories, OFFSET)
}
