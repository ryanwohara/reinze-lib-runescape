use super::common::process_stats_subsection;
use crate::common::HiscoreName::GotrRifts;
use common::l;
use common::source::Source;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![GotrRifts];

    process_stats_subsection(source, "Guardians of the Rift", categories)
}
