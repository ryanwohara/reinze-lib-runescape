mod bosses;
mod common;
mod ge;
mod items;
mod level;
mod params;
mod patch;
mod players;
mod prices;
mod stats;
mod xp;

#[no_mangle]
pub extern "C" fn exported(command: &str, query: &str, author: &str) -> Result<Vec<String>, ()> {
    match command {
        "boss" => bosses::bosses(query),
        "experience" | "exp" | "xp" => xp::xp(query),
        "ge" => ge::ge(query),
        "level" | "lvl" => level::level(query),
        "param" | "params" => params::params(query),
        "patch" => patch::patch(query),
        "players" => players::players(),
        "price" => prices::prices(query),
        // "overall" | "stats" | "total" | "attack" | "att" | "defence" | "def" | "strength"
        // | "str" | "hitpoints" | "hp" | "ranged" | "range" | "prayer" | "pray" | "magic"
        // | "mage" | "cooking" | "cook" | "woodcutting" | "wc" | "fletching" | "fletch"
        // | "fishing" | "fish" | "firemaking" | "fm" | "crafting" | "craft" | "smithing"
        // | "smith" | "mining" | "mine" | "herblore" | "herb" | "agility" | "agil" | "thieving"
        // | "thief" | "slayer" | "slay" | "farming" | "farm" | "runecraft" | "rc" | "hunter"
        // | "hunt" | "construction" | "con" => stats::stats(command, query, author),
        "help" => Ok("boss
ge
level
xp
params
players
price"
            //stats"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        "" => Ok("boss
ge
level
lvl
params
patch
players
price
xp
exp
experience"
            // stats
            // overall
            // total
            // attack
            // att
            // defence
            // def
            // strength
            // str
            // hitpoints
            // hp
            // ranged
            // range
            // prayer
            // pray
            // magic
            // mage
            // cooking
            // cook
            // woodcutting
            // wc
            // fletching
            // fletch
            // fishing
            // fish
            // firemaking
            // fm
            // crafting
            // craft
            // smithing
            // smith
            // mining
            // mine
            // herblore
            // herb
            // agility
            // agil
            // thieving
            // thief
            // slayer
            // slay
            // farming
            // farm
            // runecraft
            // rc
            // hunter
            // hunt
            // construction
            // con"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        _ => Ok(vec![]),
    }
}
