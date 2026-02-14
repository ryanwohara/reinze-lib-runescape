use crate::npc::data::{Npc, NpcMetadata};
use crate::stats::skill::Skill;
use common::commas;
use common::source::Source;

pub mod data;

pub fn lookup(s: &Source) -> Result<Vec<String>, ()> {
    let result = Npc::lookup(&s.query);
    let prefix = s.l("NPC");
    let not_found = vec![prefix.as_str(), s.c1("Not found").as_str()].join(" ");

    if result == Npc::None {
        let list = Npc::search(&s.query);
        return if list.is_empty() {
            Ok(vec![not_found])
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
                    .map(|result| s.c2(result))
                    .collect::<Vec<String>>();

                output.push(
                    vec![
                        prefix.to_string(),
                        s.p("Partial Matches"),
                        formatted.join(&s.c1(", ")),
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

            Ok(returnable)
        };
    }

    let npc = NpcMetadata::from(&result);

    let members = if npc.members { "P2P" } else { "F2P" };
    let hp = format!("{}♥", &commas(npc.hitpoints as f64, "d"));
    let attack = s.c1(&vec!["Att:", &s.c2(&commas(npc.attack as f64, "d"))].join(""));
    let defence = s.c1(&vec!["Def:", &s.c2(&commas(npc.defence as f64, "d"))].join(""));
    let magic = s.c1(&vec!["Mage:", &s.c2(&commas(npc.magic as f64, "d"))].join(""));
    let ranged = s.c1(&vec!["Range:", &s.c2(&commas(npc.ranged as f64, "d"))].join(""));
    let stab = s.c1(&vec!["Stab:", &s.c2(&commas(npc.stab as f64, "d"))].join(""));
    let slash = s.c1(&vec!["Slash:", &s.c2(&commas(npc.slash as f64, "d"))].join(""));
    let crush = s.c1(&vec!["Crush:", &s.c2(&commas(npc.crush as f64, "d"))].join(""));
    let magic_def = s.c1(&vec!["Magic:", &s.c2(&commas(npc.magic_def as f64, "d"))].join(""));
    let light_ranged = s.c1(&vec![
        "Light Ranged:",
        &s.c2(&commas(npc.light_ranged as f64, "d")),
    ]
    .join(""));
    let std_ranged =
        s.c1(&vec!["Med Ranged:", &s.c2(&commas(npc.std_ranged as f64, "d"))].join(""));
    let heavy_ranged = s.c1(&vec![
        "Heavy Ranged:",
        &s.c2(&commas(npc.heavy_ranged as f64, "d")),
    ]
    .join(""));

    let combat_xp = s.c1(&vec!["CombatXP:", &s.c2(&commas(npc.combat_xp as f64, "d"))].join(""));
    let hitpoints_xp = s.c1(&vec!["HpXp:", &s.c2(&commas(npc.hitpoints_xp as f64, "d"))].join(""));
    let slayer_xp = s.c1(&vec!["XP:", &s.c2(&commas(npc.slayer_xp as f64, "d"))].join(""));
    let slayer_req = s.c1(&vec!["Req:", &s.c2(npc.slayer_req.to_string().as_str())].join(""));

    let weakness = if !npc.weakness.is_empty() {
        npc.weakness
    } else {
        "No elemental weakness".to_string()
    };

    let first_output = vec![
        prefix,
        s.l(&npc.name),
        s.p(members),
        s.p(&hp),
        s.p(&combat_xp),
        s.p(&hitpoints_xp),
        s.c1("|"),
        attack,
        defence,
        magic,
        ranged,
    ]
    .join(" ");

    let defensive_stats = vec![
        "Defensive Stats:".to_string(),
        s.p(&vec![
            stab,
            slash,
            crush,
            magic_def,
            light_ranged,
            std_ranged,
            heavy_ranged,
        ]
        .join(&s.c1(" | "))),
    ]
    .join(" ");

    let second_output = vec![s.c2(&weakness), defensive_stats].join(&s.c1(" | "));

    if npc.slayer_masters.is_empty() {
        Ok(vec![first_output, second_output])
    } else {
        let third_output = vec![
            s.l("Slayer"),
            s.p(&slayer_req),
            s.p(&slayer_xp),
            s.c1("Assigned by:"),
            s.c2(&npc.slayer_masters),
            s.l(&npc.slayer_categories),
        ]
        .join(" ");

        Ok(vec![first_output, second_output, third_output])
    }
}
