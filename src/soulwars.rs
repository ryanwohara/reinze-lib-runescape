use super::common::process_stats_subsection;
use anyhow::Result;
use crate::common::HiscoreName::SoulWarsZeal;
use common::source::Source;

pub fn lookup(source: Source) -> Result<Vec<String>> {
    let categories = vec![SoulWarsZeal];

    process_stats_subsection(source, "Soul Wars", categories)
}
