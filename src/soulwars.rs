use super::stats::process_stats_subsection;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 41;

    let categories: Vec<&str> = vec!["Points"];

    process_stats_subsection(source, &l("Soul Wars"), categories, OFFSET)
}
