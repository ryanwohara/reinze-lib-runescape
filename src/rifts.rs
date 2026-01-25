use super::stats::process_stats_subsection;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 42;

    let categories: Vec<&str> = vec!["Closed"];

    process_stats_subsection(source, &l("Guardians of the Rift"), categories, OFFSET)
}
