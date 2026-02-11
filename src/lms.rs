use super::common::process_stats_subsection;
use crate::common::HiscoreName::LMS;
use common::l;
use common::source::Source;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![LMS];

    process_stats_subsection(source, &l("Last Man Standing"), categories)
}
