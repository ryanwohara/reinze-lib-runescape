use super::common::process_stats_subsection;
use crate::common::HiscoreName::GotrRifts;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![GotrRifts];

    process_stats_subsection(source, &l("Guardians of the Rift"), categories)
}
