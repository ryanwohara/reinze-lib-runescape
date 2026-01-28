use super::common::process_stats_subsection;
use crate::common::HiscoreName::Leagues;
use crate::common::Source;
use common::l;

pub fn lookup(mut source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![Leagues];

    source.query = format!("{} -l", source.query);

    process_stats_subsection(source, &l("Seasonal"), categories)
}
