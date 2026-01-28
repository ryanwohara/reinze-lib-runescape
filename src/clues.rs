use super::common::process_stats_subsection;
use crate::common::HiscoreName::{
    ClueScrollAll, ClueScrollBeginner, ClueScrollEasy, ClueScrollElite, ClueScrollHard,
    ClueScrollMaster, ClueScrollMedium,
};
use crate::common::Source;
use common::l;

pub fn lookup(source: Source) -> Result<Vec<String>, ()> {
    let categories = vec![
        ClueScrollAll,
        ClueScrollBeginner,
        ClueScrollEasy,
        ClueScrollMedium,
        ClueScrollHard,
        ClueScrollElite,
        ClueScrollMaster,
    ];

    process_stats_subsection(source, &l("Clues"), categories)
}
