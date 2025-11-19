use super::stats::process_stats_subsection;
use common::l;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 27;

    let categories: Vec<&str> = vec!["Hunter", "Rogue", "Legacy Hunter", "Legacy Rogue"];

    process_stats_subsection(
        query,
        author,
        rsn_n,
        &l("Bounty Hunter"),
        categories,
        OFFSET,
    )
}
