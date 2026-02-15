use super::common::process_stats_subsection;
use crate::common::HiscoreName::Gridmaster;
use common::source::Source;

pub fn lookup(mut source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![Gridmaster];

    source.query = format!("{} -t", source.query);

    process_stats_subsection(source, "Grid Master", categories)
}
