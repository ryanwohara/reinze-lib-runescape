use super::common::process_stats_subsection;
use crate::common::HiscoreName::SoulWarsZeal;
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![SoulWarsZeal];

    process_stats_subsection(source, &l("Soul Wars"), categories)
}
