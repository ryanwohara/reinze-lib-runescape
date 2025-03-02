use super::stats::process_stats_subsection;
use common::l;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 42;

    let categories: Vec<&str> = vec!["Entries"];

    process_stats_subsection(
        query,
        author,
        rsn_n,
        &l("Collection Log"),
        categories,
        OFFSET,
    )
}
