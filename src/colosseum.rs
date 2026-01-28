use super::common::process_stats_subsection;
use crate::common::HiscoreName::ColosseumGlory;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![ColosseumGlory];

    process_stats_subsection(source, &l("Colosseum"), categories)
}
