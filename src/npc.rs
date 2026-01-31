use crate::npc::data::{Npc, NpcMetadata};
use common::{c1, c2, commas, l, p};

mod data;

pub fn lookup(query: &str) -> Result<Vec<String>, ()> {
    let result = Npc::lookup(query);
    let prefix = l("NPC");
    let not_found = vec![prefix.as_str(), c1("Not found").as_str()].join(" ");

    if result == Npc::None {
        let list = Npc::search(query);
        if list.is_empty() {
            return Ok(vec![not_found]);
        } else {
            let found = list
                .iter()
                .map(|npc| NpcMetadata::from(npc).name)
                .collect::<Vec<String>>();

            if found.is_empty() {
                return Ok(vec![not_found]);
            }

            let mut chunks = found.chunks(10);
            let mut output = vec![];
            while let Some(results) = chunks.next() {
                let formatted = results
                    .iter()
                    .map(|result| c2(result))
                    .collect::<Vec<String>>();

                output.push(
                    vec![
                        prefix.to_string(),
                        p("Partial Matches"),
                        formatted.join(&c1(", ")),
                    ]
                    .join(" "),
                );
            }

            let mut top = output.chunks(3);
            let mut returnable = vec![];

            if let Some(chunks) = top.next() {
                for chunk in chunks {
                    returnable.push(chunk.to_string());
                }
            }

            return Ok(returnable);
        }
    }

    let npc = NpcMetadata::from(&result);

    let members = if npc.members { "P2P" } else { "F2P" };
    let hp = format!("{}♥", &commas(npc.hitpoints as f64, "d"));
    let attack = c1(&vec!["Att:", &c2(&commas(npc.attack as f64, "d"))].join(""));
    let defence = c1(&vec!["Def:", &c2(&commas(npc.defence as f64, "d"))].join(""));
    let magic = c1(&vec!["Mage:", &c2(&commas(npc.magic as f64, "d"))].join(""));
    let ranged = c1(&vec!["Range:", &c2(&commas(npc.ranged as f64, "d"))].join(""));
    let stab = c1(&vec!["Stab:", &c2(&commas(npc.stab as f64, "d"))].join(""));
    let slash = c1(&vec!["Slash:", &c2(&commas(npc.slash as f64, "d"))].join(""));
    let crush = c1(&vec!["Crush:", &c2(&commas(npc.crush as f64, "d"))].join(""));
    let magic_def = c1(&vec!["Magic:", &c2(&commas(npc.magic_def as f64, "d"))].join(""));
    let light_ranged =
        c1(&vec!["Light Ranged:", &c2(&commas(npc.light_ranged as f64, "d"))].join(""));
    let std_ranged = c1(&vec!["Med Ranged:", &c2(&commas(npc.std_ranged as f64, "d"))].join(""));
    let heavy_ranged =
        c1(&vec!["Heavy Ranged:", &c2(&commas(npc.heavy_ranged as f64, "d"))].join(""));

    let combat_xp = c1(&vec!["CombatXP:", &c2(&commas(npc.combat_xp as f64, "d"))].join(""));
    let hitpoints_xp = c1(&vec!["HpXp:", &c2(&commas(npc.hitpoints_xp as f64, "d"))].join(""));

    let weakness = if !npc.weakness.is_empty() {
        npc.weakness
    } else {
        "No elemental weakness".to_string()
    };

    let first_output = vec![
        prefix,
        l(&npc.name),
        p(members),
        p(&hp),
        p(&combat_xp),
        p(&hitpoints_xp),
        c1("|"),
        attack,
        defence,
        magic,
        ranged,
    ]
    .join(" ");

    let defensive_stats = vec![
        "Defensive Stats:".to_string(),
        p(&vec![
            stab,
            slash,
            crush,
            magic_def,
            light_ranged,
            std_ranged,
            heavy_ranged,
        ]
        .join(&c1(" | "))),
    ]
    .join(" ");

    let second_output = vec![c2(&weakness), defensive_stats].join(&c1(" | "));

    println!("{}", first_output);
    println!("{}", second_output);

    Ok(vec![first_output, second_output])
}
