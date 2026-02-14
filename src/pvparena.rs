use super::common::process_stats_subsection;
use crate::common::HiscoreName::PvpArena;
use common::l;
use common::source::Source;

pub fn lookup(s: Source) -> Result<Vec<String>, ()> {
    let categories = vec![PvpArena];

    process_stats_subsection(s, &l("PVP Arena"), categories)
}
