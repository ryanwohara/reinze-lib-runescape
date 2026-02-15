use super::common::process_stats_subsection;
use crate::common::HiscoreName::ColosseumGlory;
use common::source::Source;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![ColosseumGlory];

    process_stats_subsection(source, "Colosseum", categories)
}
