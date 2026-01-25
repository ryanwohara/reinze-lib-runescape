use super::stats::process_stats_subsection;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 43;

    let categories: Vec<&str> = vec!["Glory"];

    process_stats_subsection(source, &l("Colosseum"), categories, OFFSET)
}
