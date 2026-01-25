use super::stats::process_stats_subsection;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 26;

    let categories: Vec<&str> = vec!["League Points"];

    let query = format!("{} -l", source.query);

    process_stats_subsection(source, &l("Seasonal"), categories, OFFSET)
}
