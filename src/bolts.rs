use common::{c1, c2, l, p};

pub fn lookup(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Bolts");

    if query.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, c1("No query provided"))]);
    }

    for bolt in Bolt::all() {
        let details = bolt.details();

        if details
            .bolt
            .to_ascii_lowercase()
            .contains(&query.to_ascii_lowercase())
            || details
                .name
                .to_ascii_lowercase()
                .contains(&query.to_ascii_lowercase())
        {
            return Ok(vec![format!("{} {}", prefix, details.to_string(),)]);
        }
    }

    Ok(vec![format!("{}: {}", prefix, c1("No results found"))])
}

enum Bolt {
    Opal,
    Sapphire,
    Jade,
    Pearl,
    Emerald,
    Topaz,
    Ruby,
    Diamond,
    Dragonstone,
    Onyx,
}

impl Bolt {
    fn all() -> Vec<Self> {
        vec![
            Self::Opal,
            Self::Sapphire,
            Self::Jade,
            Self::Pearl,
            Self::Emerald,
            Self::Topaz,
            Self::Ruby,
            Self::Diamond,
            Self::Dragonstone,
            Self::Onyx,
        ]
    }

    fn details(&self) -> BoltDetails {
        match self {
            Bolt::Opal => BoltDetails::from(
                "Opal",
                "Lucky Lightning",
                "Chance of a lighting bolt striking your opponent and causing extra damage.",
            ),
            Bolt::Sapphire => BoltDetails::from(
                "Sapphire",
                "Clear Mind",
                "Chance of lowering target's Prayer points and giving part of them to the attacker. Only works in PvP.",
            ),
            Bolt::Jade => BoltDetails::from(
                "Jade",
                "Earth's Fury",
                "Chance of knocking their target to the ground, although agile opponents may resist this.",
            ),
            Bolt::Pearl => BoltDetails::from(
                "Pearl",
                "Sea Curse",
                "Chance of waterfall hitting oppenent. This ability is negated by opponents wielding water staves but increased by opponents wearing fiery garments.",
            ),
            Bolt::Emerald => BoltDetails::from(
                "Emerald",
                "Magical Poison",
                "Chance of poisoning opponent. Damage of 5, with poison.",
            ),
            Bolt::Topaz => BoltDetails::from(
                "Topaz",
                "Down to Earth",
                "Chance of lowering opponents Magic level. Only works in PvP.",
            ),
            Bolt::Ruby => BoltDetails::from(
                "Ruby",
                "Blood Forfeit",
                "Chance of removing 10% of attacker's Hitpoints and 20% of their target's Hitpoints.",
            ),
            Bolt::Diamond => BoltDetails::from(
                "Diamond",
                "Armour Piercing",
                "Chance of negating a sizable portion of opponent's Defence bonuses againts Ranged attacks.",
            ),
            Bolt::Dragonstone => BoltDetails::from(
                "Dragonstone",
                "Dragon's Breath",
                "Chance of inflicting a dragon's breath hit against oppenent, unless they have an AntiDragon Shield, have used an AntiFire Potion, or are some type of fiery beast.",
            ),
            Bolt::Onyx => BoltDetails::from(
                "Onyx",
                "Life Leech",
                "Chance of doing extra damage of healing the attacker's Hitpoints by 25% of the damage dealt. This does not work on the undead, as they have no life to leech.",
            ),
        }
    }
}

struct BoltDetails {
    bolt: String,
    name: String,
    effect: String,
}

impl BoltDetails {
    fn from(bolt: &str, name: &str, effect: &str) -> Self {
        Self {
            bolt: bolt.to_string(),
            name: name.to_string(),
            effect: effect.to_string(),
        }
    }

    fn to_string(&self) -> String {
        vec![p(&self.bolt), c1(&self.name), c1("|"), c2(&self.effect)].join(" ")
    }
}
