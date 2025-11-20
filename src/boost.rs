use crate::common::skill;
use common::{c1, c2, l, not_found, p};

pub fn boost(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Boost");
    let skill = skill(query);
    let mut found_params: Vec<String> = vec![];

    for (action, boost) in get_modifiers(&skill) {
        found_params.push(format!("{} {}", c1(&action), c2(&boost)));
    }

    let output = format!(
        "{} {}{}",
        prefix,
        format_skill(&skill),
        not_found(found_params)
    );

    Ok(vec![output])
}

fn format_skill(skill: &str) -> String {
    if skill.len() > 0 {
        format!("{} ", p(&skill))
    } else {
        "".to_string()
    }
}

fn get_modifiers(skill: &str) -> Vec<(String, String)> {
    match skill {
        "Agility" => vec![
            ("Elven dawn", "+1"),
            ("Sunbeam Ale", "+1"),
            ("Agility potion/mix", "+3"),
            ("Summer pie", "+5"),
            ("Yellow spicy stew", "-5/+5"),
        ],
        "Attack" => vec![
            ("Bandit's brew", "+1"),
            ("Cup of Tea", "+2"),
            ("Jangerberries", "+2"),
            ("Kebab", "+1-3"),
            ("Poison chalice", "+1/+4"),
            ("Attack/Combat potion/mix", "+3-12"),
            ("Super attack/mix", "+5-19"),
            ("Super combat potion", "+5-19"),
            ("Smouldering heart", "+?-20"),
            ("Zamorak brew", "+2-21"),
            ("Red spicy stew", "-5/+5"),
        ],
        "Construction" => vec![
            ("Cup of Tea", "+1-3"),
            ("Crystal saw (stackable)", "+3"),
            ("Orange spicy stew", "-5/+5"),
        ],
        "Cooking" => vec![
            ("Chef's Delight", "+1-5"),
            ("Mature Chef's Delight", "+2-6"),
            ("Orange spicy stew", "-5/+5"),
        ],
        "Crafting" => vec![
            ("Poison chalice", "+0-1"),
            ("Mushroom pie", "+4"),
            ("Orange spicy stew", "-5/+5"),
        ],
        "Defence" => vec![
            ("Defence potion/mix", "+3-12"),
            ("Super defence/mix", "+5-19 "),
            ("Smouldering heart", "+?-20"),
            ("Saradomin brew", "+2-21"),
            ("Draynor Manor cabbage", "+1"),
            ("Red spicy stew", "-5/+5"),
            ("Excalibur", "+8"),
        ],
        "Farming" => vec![
            ("Cider", "+1"),
            ("Ixcoztic white", "+1"),
            ("Mature cider", "+2"),
            ("Garden pie", "+3"),
            ("Brown spicy stew", "-5/+5"),
        ],
        "Firemaking" => vec![("Chilhuac red", "+1"), ("Orange spicy stew", "-5/+5")],
        "Fishing" => vec![
            ("Dragon/Infernal/Crystal harpoon", "+3"),
            ("Fishing potion/mix", "+3"),
            ("Fish pie", "+3"),
            ("Admiral pie", "+5"),
            ("Brown spicy stew", "-5/+5"),
            ("Fishing guild", "+7"),
        ],
        "Fletching" => vec![("Dragonfruit pie", "+4"), ("Orange spicy stew", "-5/+5")],
        "Herblore" => vec![
            ("Greenman's ale", "+1"),
            ("Chichilihui rosé", "+1"),
            ("Mature greenman's ale", "+2"),
            ("Botanical pie", "+4"),
            ("Brown spicy stew", "-5/+5"),
        ],
        "Hitpoints" => vec![
            ("Bloody bracer", "+2"),
            ("Guthix rest", "+5"),
            ("Elidinis Statuette", "+7"),
            ("Saradomin brew", "+3-16"),
            ("Smouldering pile of flesh", "+?-18"),
            ("Anglerfish", "+3-22"),
        ],
        "Hunter" => vec![
            ("Blackbird red", "+1"),
            ("Trapper's tipple", "+2"),
            ("Hunter potion/mix", "+3"),
            ("Yellow spicy stew", "-5/+5"),
        ],
        "Magic" => vec![
            ("Steamforge brew", "+1"),
            ("Talking to Oldak", "-2/+2"),
            ("Wizard's mind bomb", "+2-3"),
            ("Magic essence potion/mix", "+3"),
            ("Mature Wizard's mind bomb", "+3-4"),
            ("Battlemage potion", "+4"),
            ("Magic potion/mix", "+5"),
            ("Red spicy stew", "-5/+5"),
            ("Ancient brew", "+2-6"),
            ("Forgotten brew", "+3-10"),
            ("Imbued Heart", "+1-10"),
            ("Saturated Heart", "+4-13"),
            ("Smoulering heart", "+?-11"),
        ],
        "Mining" => vec![
            ("Braindeath 'rum'", "+1"),
            ("Dwarven stout", "+1"),
            ("Steamforge brew", "+1"),
            ("Mature dwarven stout", "+2"),
            ("Dragon/Infernal/3rd age/Crystal pickaxe", "+3"),
            ("Celestial signet", "+4"),
            ("Brown spicy stew", "-5/+5"),
            ("Mining guild", "+7"),
        ],
        "Prayer" => vec![
            ("Monastery altar", "+2"),
            ("Nature Grotto", "+2"),
            ("Seers' Village altar", "+2"),
            ("Yellow spicy stew", "-5/+5"),
            ("GWD altars", "+0-11"),
            ("Ancient mace", "+1-42"),
            ("Eldritch nightmare staff", "+1-60"),
        ],
        "Ranged" => vec![
            ("Wild pie", "+4"),
            ("Lizardkicker", "+4"),
            ("Ranging potion/mix", "+4-13"),
            ("Bastion potion", "+4-13"),
            ("Red spicy stew", "-5/+5"),
            ("Smouldering heart", "+?-14"),
        ],
        "Runecraft" => vec![
            ("Metztonalli white", "+1"),
            ("Talking to Oldak", "-2/+2"),
            ("Orange spicy stew", "-5/+5"),
        ],
        "Sailing" => vec![
            ("Kraken ink stout", "+1"),
            ("Perildance bitter", "+1"),
            ("Trawler's trust", "+2"),
            ("Whirlpool surprise", "+3"),
            ("Horizon's lure", "+4"),
            ("Yellow spicy stew", "-5/+5"),
        ],
        "Slayer" => vec![
            ("Imperial rosé", "+1"),
            ("Slayer's respite", "+2"),
            ("Mature slayer's respite", "+4"),
            ("Yellow spicy stew", "-5/+5"),
            ("Wild pie", "+5"),
        ],
        "Smithing" => vec![
            ("Dwarven stout", "+1"),
            ("Mature dwarven stout", "+2"),
            ("Kovac's grog", "+4"),
            ("Orange spicy stew", "-5/+5"),
        ],
        "Strength" => vec![
            ("Jangerberries", "+1"),
            ("Beer", "+0-3"),
            ("Brandy", "+1-5"),
            ("Sun-shine", "+1-5"),
            ("Moon-lite", "+1-5"),
            ("Sunbeam ale", "+1"),
            ("Kebab", "+1-3"),
            ("Braindeath 'rum'", "+3"),
            ("Poison chalice", "+1/+4"),
            ("Red spicy stew", "-5/+5"),
            ("Short green guy/Premade sgg", "+1-5"),
            ("Grog", "+1-5"),
            ("Karamjan rum", "+1-5"),
            ("Blood pint", "+2-5"),
            ("Wizard blizzard/Blurberry special/Drunk dragon", "+1-6"),
            ("Keg of beer", "+2-11"),
            ("Strength potion/mix", "+3-12"),
            ("Combat potion/mix", "+3-12"),
            ("Zamorak Brew", "+2-13"),
            ("Super strength/mix", "+5-19"),
            ("Super combat potion", "+5-19"),
            ("Dragon battleaxe", "+10-21"),
            ("Smouldering heart", "+?-20"),
        ],
        "Thieving" => vec![
            ("Poison chalice", "+1"),
            ("Bandit's brew", "+1"),
            ("Spring sq'irk juice", "+1"),
            ("Autumn sq'irk juice", "+2"),
            ("Summer sq'irk juice", "+3"),
            ("Yellow spicy stew", "-5/+5"),
        ],
        "Woodcutting" => vec![
            ("Eclipse red", "+1"),
            ("Axeman's folly", "+1"),
            ("Mature axeman's folly", "+2"),
            ("Dragon/Infernal/3rd Age/Crystal axe/felling axe", "+3"),
            ("Brown spicy stew", "-5/+5"),
            ("WC Guild", "+7"),
            ("Forestry", "+0-10"),
        ],
        _ => vec![],
    }
    .into_iter()
    .map(|(name, boost)| (name.to_string(), boost.to_string()))
    .collect::<Vec<(String, String)>>()
}
