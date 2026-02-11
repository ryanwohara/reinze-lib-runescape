use super::common::process_stats_subsection;
use crate::common::HiscoreName::{
    BountyHunterHunter, BountyHunterLegacyHunter, BountyHunterLegacyRogue, BountyHunterRogue,
};
use common::l;
use common::source::Source;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![
        BountyHunterHunter,
        BountyHunterRogue,
        BountyHunterLegacyHunter,
        BountyHunterLegacyRogue,
    ];

    process_stats_subsection(source, &l("Bounty Hunter"), categories)
}
