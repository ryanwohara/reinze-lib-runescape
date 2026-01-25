use super::stats::process_stats_subsection;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 32;

    let categories: Vec<&str> = vec![
        "All", "Beginner", "Easy", "Medium", "Hard", "Elite", "Master",
    ];

    process_stats_subsection(source, &l("Clues"), categories, OFFSET)
}
