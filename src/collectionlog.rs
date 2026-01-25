use super::stats::process_stats_subsection;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 44;

    let categories: Vec<&str> = vec!["Entries"];

    process_stats_subsection(source, &l("Collection Log"), categories, OFFSET)
}
