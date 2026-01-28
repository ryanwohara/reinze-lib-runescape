use super::common::process_stats_subsection;
use crate::common::HiscoreName::CollectionsLogged;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![CollectionsLogged];

    process_stats_subsection(source, &l("Collection Log"), categories)
}
