use super::common::process_stats_subsection;
use crate::common::HiscoreName::Gridmaster;
use crate::common::Source;
use common::l;

pub fn lookup(mut source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![Gridmaster];

    source.query = format!("{} -t", source.query);

    process_stats_subsection(source, &l("Grid Master"), categories)
}
