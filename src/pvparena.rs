use super::common::process_stats_subsection;
use crate::common::HiscoreName::PvpArena;
use anyhow::Result;
use common::source::Source;

pub fn lookup(s: Source) -> Result<Vec<String>> {
    let categories = vec![PvpArena];

    // `process_stats_subsection` applies the caller's colors to this label.
    process_stats_subsection(s, "PVP Arena", categories)
}
