use super::stats::process_stats_subsection;
use crate::common::Source;
use common::l;

pub fn lookup(mut source: Source) -> Result<Vec<String>, ()> {
    const OFFSET: isize = 25;

    let categories: Vec<&str> = vec!["Grid Points"];

    source.query = format!("{} -t", source.query);

    process_stats_subsection(source, &l("Grid Master"), categories, OFFSET)
}
