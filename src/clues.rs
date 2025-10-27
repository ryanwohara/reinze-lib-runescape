use super::stats::process_stats_subsection;
use common::l;

pub fn lookup(query: &str, author: &str, rsn_n: &str) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 31;

    let categories: Vec<&str> = vec![
        "All", "Beginner", "Easy", "Medium", "Hard", "Elite", "Master",
    ];

    process_stats_subsection(query, author, rsn_n, &l("Clues"), categories, OFFSET)
}
