use super::stats::process_stats_subsection;
use common::l;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 40;

    let categories: Vec<&str> = vec!["Closed"];

    process_stats_subsection(
        query,
        author,
        rsn_n,
        &l("Guardians of the Rift"),
        categories,
        OFFSET,
    )
}
