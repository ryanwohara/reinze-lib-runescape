use anyhow::Result;
use common::source::Source;
use std::fmt;
use std::str::FromStr;

pub fn patch(s: &Source) -> Result<Vec<String>> {
    let prefix = s.l("Patch");
    let patch: Patch = s.query.parse().unwrap_or(Patch::None);

    let locations = patch
        .locations()
        .iter()
        .map(|location| s.c2(&location))
        .collect();

    let output = format!(
        "{} {}{}",
        prefix,
        format_patch(&patch.to_string(), s),
        s.not_found(locations)
    );

    Ok(vec![output])
}

fn format_patch(patch: &str, s: &Source) -> String {
    if !patch.is_empty() {
        format!("{} ", s.p(&patch))
    } else {
        "".to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Patch {
    Allotment,
    Flower,
    Herb,
    Bush,
    Tree,
    Fruit,
    Hops,
    Spirit,
    Belladonna,
    Calquat,
    Mushroom,
    Celastrus,
    Redwood,
    Crystal,
    Seaweed,
    Grape,
    Hespori,
    Anima,
    Cactus,
    Hardwood,
    Coral,
    None,
}

impl Patch {
    fn all() -> Vec<Self> {
        vec![
            Self::Allotment,
            Self::Flower,
            Self::Herb,
            Self::Bush,
            Self::Tree,
            Self::Fruit,
            Self::Hops,
            Self::Spirit,
            Self::Belladonna,
            Self::Calquat,
            Self::Mushroom,
            Self::Celastrus,
            Self::Redwood,
            Self::Crystal,
            Self::Seaweed,
            Self::Grape,
            Self::Hespori,
            Self::Anima,
            Self::Cactus,
            Self::Hardwood,
            Self::Coral,
            Self::None,
        ]
    }

    fn locations(&self) -> &'static [&'static str] {
        match self {
            Self::Allotment => &[
                "Falador South",
                "Port Phasmatys",
                "Harmony Island",
                "Catherby North",
                "Ardougne North",
                "Farming Guild",
                "Hosidius South-west",
                "Prifddinas North",
                "Civitas illa Fortis West",
            ],
            Self::Flower => &[
                "Falador South",
                "Port Phasmatys",
                "Catherby North",
                "Ardougne North",
                "Farming Guild",
                "Hosidius South-west",
                "Prifddinas North",
                "Civitas illa Fortis West",
                "Kastori",
            ],
            Self::Herb => &[
                "Falador South",
                "Troll Stronghold Rooftop",
                "Port Phasmatys",
                "Catherby North",
                "Ardougne North",
                "Farming Guild",
                "Weiss",
                "Harmony Island",
                "Hosidius South-west",
                "Civitas illa Fortis West",
            ],
            Self::Bush => &[
                "Champions' Guild",
                "Rimmington",
                "Ardougne South",
                "Etceteria South-west",
                "Farming Guild",
            ],
            Self::Tree => &[
                "Lumbridge West",
                "Varrock Castle",
                "Falador Park",
                "Taverley",
                "Gnome Stronghold",
                "Farming Guild",
                "Nemus Retreat",
            ],
            Self::Fruit => &[
                "Catherby East",
                "Tree Gnome Maze West",
                "Brimhaven North",
                "Gnome Stronghold",
                "Lletya",
                "Farming Guild",
                "Kastori",
            ],
            Self::Hops => &[
                "Lumbridge North",
                "McGrubor's Woods North",
                "Yanille",
                "Entrana",
                "Aldarin",
            ],
            Self::Spirit => &[
                "Etceteria South-east",
                "Port Sarim East",
                "Brimhaven East",
                "Farming Guild",
                "Hosidius South-west",
            ],
            Self::Belladonna => &["Draynor Village Manor", "Auburnvale"],
            Self::Calquat => &[
                "Kastori",
                "Tai Bwo Wannai North",
                "The Great Conch (Summer Shore)",
            ],
            Self::Mushroom => &["Canifis"],
            Self::Celastrus => &["Farming Guild"],
            Self::Redwood => &["Farming Guild"],
            Self::Crystal => &["Prifddinas North"],
            Self::Seaweed => &["Underwater (Fossil Island)"],
            Self::Grape => &["Hosidius Vinery"],
            Self::Hespori => &["Farming Guild"],
            Self::Anima => &["Farming Guild"],
            Self::Cactus => &["Al Kharid", "Farming Guild"],
            Self::Hardwood => &[
                "Fossil Island (Mushroom Forest)",
                "Locus Oasis (Varlamore)",
                "Anglers' Retreat",
            ],
            Self::Coral => &["Coral Nurseries (East. Great Conch)"],
            Self::None => &[],
        }
    }
}

impl FromStr for Patch {
    type Err = ();

    fn from_str(query: &str) -> Result<Self, Self::Err> {
        Self::all()
            .into_iter()
            .find(|patch| {
                patch
                    .to_string()
                    .to_lowercase()
                    .contains(&query.to_lowercase())
            })
            .ok_or(())
    }
}

impl fmt::Display for Patch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Allotment => "Allotment",
            Self::Flower => "Flower",
            Self::Herb => "Herb",
            Self::Bush => "Bush",
            Self::Tree => "Tree",
            Self::Fruit => "Fruit",
            Self::Hops => "Hops",
            Self::Spirit => "Spirit",
            Self::Belladonna => "Belladonna",
            Self::Calquat => "Calquat",
            Self::Mushroom => "Mushroom",
            Self::Celastrus => "Celastrus",
            Self::Redwood => "Redwood",
            Self::Crystal => "Crystal",
            Self::Seaweed => "Seaweed",
            Self::Grape => "Grape",
            Self::Hespori => "Hespori",
            Self::Anima => "Anima",
            Self::Cactus => "Cactus",
            Self::Hardwood => "Hardwood",
            Self::Coral => "Coral",
            Self::None => "",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every patch type, so a new one cannot be added without the checks below
    /// seeing it.
    const ALL: [Patch; 21] = [
        Patch::Allotment,
        Patch::Flower,
        Patch::Herb,
        Patch::Bush,
        Patch::Tree,
        Patch::Fruit,
        Patch::Hops,
        Patch::Spirit,
        Patch::Belladonna,
        Patch::Calquat,
        Patch::Mushroom,
        Patch::Celastrus,
        Patch::Redwood,
        Patch::Crystal,
        Patch::Seaweed,
        Patch::Grape,
        Patch::Hespori,
        Patch::Anima,
        Patch::Cactus,
        Patch::Hardwood,
        Patch::Coral,
    ];

    #[test]
    fn no_patch_lists_the_same_location_twice() {
        for patch in ALL {
            let locations = patch.locations();
            let mut unique = locations.to_vec();
            unique.sort();
            unique.dedup();

            assert_eq!(
                locations.len(),
                unique.len(),
                "{} lists a location twice: {:?}",
                patch,
                locations
            );
        }
    }

    /// Varlamore has one herb patch, at Ortus Farm west of Civitas illa Fortis.
    /// It was listed twice - once under each name - so a herb run counted a
    /// patch that does not exist.
    #[test]
    fn varlamore_has_exactly_one_herb_patch() {
        let herbs = Patch::Herb.locations();

        assert!(herbs.contains(&"Civitas illa Fortis West"));
        assert!(
            !herbs.contains(&"Varlamore"),
            "the Varlamore herb patch is the Civitas illa Fortis one: {:?}",
            herbs
        );
    }

    /// The three Ortus Farm patches share a name, so a rename has to move all
    /// of them together.
    #[test]
    fn ortus_farm_is_named_the_same_way_across_its_three_patches() {
        for patch in [Patch::Allotment, Patch::Flower, Patch::Herb] {
            assert!(
                patch.locations().contains(&"Civitas illa Fortis West"),
                "{} is missing the Ortus Farm patch",
                patch
            );
        }
    }

    /// Prifddinas grows two allotments and a flower patch beside its crystal
    /// tree, but no herb - so it belongs on an allotment run and not a herb
    /// one. All three share the crystal patch's name, since they sit together.
    #[test]
    fn prifddinas_has_allotment_and_flower_patches_but_no_herb() {
        for patch in [Patch::Allotment, Patch::Flower, Patch::Crystal] {
            assert!(
                patch.locations().contains(&"Prifddinas North"),
                "{} is missing the Prifddinas patch: {:?}",
                patch,
                patch.locations()
            );
        }

        let herbs = Patch::Herb.locations();
        assert!(
            !herbs.iter().any(|location| location.contains("Prifddinas")),
            "Prifddinas has no herb patch: {:?}",
            herbs
        );
    }

    /// Kastori holds three patches - calquat, fruit tree, and a flower patch
    /// that, unusually, has no allotment beside it. A run that stops short of
    /// any of them misses a patch.
    #[test]
    fn kastori_holds_a_calquat_fruit_and_flower_patch() {
        for patch in [Patch::Calquat, Patch::Fruit, Patch::Flower] {
            assert!(
                patch.locations().contains(&"Kastori"),
                "{} is missing the Kastori patch: {:?}",
                patch,
                patch.locations()
            );
        }
    }

    /// Auburn Valley holds two patches in two different settlements: the tree
    /// patch at Nemus Retreat and the belladonna patch at Auburnvale itself.
    /// Naming both "Auburnvale" sent players to the wrong one.
    #[test]
    fn auburn_valley_names_its_two_patches_apart() {
        let trees = Patch::Tree.locations();
        let belladonna = Patch::Belladonna.locations();

        assert!(trees.contains(&"Nemus Retreat"), "trees: {:?}", trees);
        assert!(
            !trees.contains(&"Auburnvale"),
            "the tree patch is at Nemus Retreat, not Auburnvale: {:?}",
            trees
        );
        assert!(
            belladonna.contains(&"Auburnvale"),
            "belladonna: {:?}",
            belladonna
        );
    }

    #[test]
    fn every_patch_type_names_at_least_one_location() {
        for patch in ALL {
            assert!(
                !patch.locations().is_empty(),
                "{} has nowhere to farm",
                patch
            );
        }
    }
}
