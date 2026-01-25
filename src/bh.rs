use super::stats::process_stats_subsection;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 27;

    let categories: Vec<&str> = vec!["Hunter", "Rogue", "Legacy Hunter", "Legacy Rogue"];

    process_stats_subsection(source, &l("Bounty Hunter"), categories, OFFSET)
}
