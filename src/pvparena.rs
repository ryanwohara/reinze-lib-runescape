use super::stats::process_stats_subsection;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 40;

    let categories: Vec<&str> = vec!["Kills"];

    process_stats_subsection(source, &l("PVP Arena"), categories, OFFSET)
}
