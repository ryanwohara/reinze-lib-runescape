mod bh;
mod bosses;
mod clues;
mod common;
mod ge;
mod items;
mod level;
mod lms;
mod params;
mod patch;
mod plant;
mod players;
mod prices;
mod pvparena;
mod rifts;
mod soulwars;
mod stats;
mod wiki;
mod xp;

use regex::Regex;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn exported(
    cmd: *const c_char,
    raw_query: *const c_char,
    raw_author: *const c_char,
) -> *mut c_char {
    let nil = CString::new("").unwrap().into_raw();

    if cmd.is_null() || raw_query.is_null() || raw_author.is_null() {
        return nil; // using unwrap() here is safe because we know the string is valid UTF-8
    }

    let unsafe_cmd = unsafe { CStr::from_ptr(cmd) };
    let unsafe_query = unsafe { CStr::from_ptr(raw_query) };
    let unsafe_author = unsafe { CStr::from_ptr(raw_author) };

    let mut command = match unsafe_cmd.to_str() {
        Ok(command) => command,
        Err(_) => return nil,
    };

    let query = match unsafe_query.to_str() {
        Ok(query) => query,
        Err(_) => return nil,
    };

    let author = match unsafe_author.to_str() {
        Ok(author) => author,
        Err(_) => return nil,
    };

    let re = Regex::new(r"^([a-zA-Z]+)(\d+)$").unwrap();
    let re_match = match re.captures(command) {
        Some(captures) => vec![captures],
        None => vec![],
    };

    let mut rsn_n = "0";

    if re_match.len() > 0 {
        command = re_match[0].get(1).unwrap().as_str();
        rsn_n = re_match[0].get(2).unwrap().as_str();
    }

    match match command {
        "bh" | "bounty" | "bhunter" | "bountyhunter" => bh::lookup(query, author, rsn_n),
        "boss" | "bosses" => bosses::lookup(query, author, rsn_n),
        "clue" | "clues" => clues::lookup(query, author, rsn_n),
        "experience" | "xperience" | "exp" | "xp" => xp::xp(query),
        "ge" => ge::ge(query),
        "level" | "lvl" => level::level(query),
        "lms" | "lmstanding" | "lmanstanding" | "lastmstanding" | "lastmanstanding" => {
            lms::lookup(query, author, rsn_n)
        }
        "param" | "params" => params::params(query),
        "patch" => patch::patch(query),
        "players" => players::players(),
        "price" => prices::prices(query),
        "overall" | "stats" | "total" | "combat" | "cmb" | "attack" | "att" | "defence" | "def"
        | "strength" | "str" | "hitpoints" | "hp" | "ranged" | "range" | "prayer" | "pray"
        | "magic" | "mage" | "cooking" | "cook" | "woodcutting" | "wc" | "fletching" | "fletch"
        | "fishing" | "fish" | "firemaking" | "fm" | "crafting" | "craft" | "smithing"
        | "smith" | "mining" | "mine" | "herblore" | "herb" | "agility" | "agil" | "thieving"
        | "thief" | "slayer" | "slay" | "farming" | "farm" | "runecraft" | "rc" | "hunter"
        | "hunt" | "construction" | "con" => stats::stats(command, query, author, rsn_n),
        "plant" | "plants" => plant::lookup(query),
        "pvparena" | "pvp" | "arena" => pvparena::lookup(query, author, rsn_n),
        "rift" | "rifts" => rifts::lookup(query, author, rsn_n),
        "sw" | "swar" | "soulw" | "soulwar" | "soulwars" | "zeal" => {
            soulwars::lookup(query, author, rsn_n)
        }
        "wiki" => wiki::wiki(query),
        "help" => Ok(r"bh[N]
boss[N]
clues[N]
combat[N]
ge
level
lms[N]
xp
params
plant
players
price
pvparena[N]
rifts[N]
sw[N]
stats[N]
wiki"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        "" => Ok(r"b(ounty)?h(unter)?\d*
boss\d*
clues?\d*
co?mb(at)?\d*
ge
l(ast)?m(an)?s(tanding)?\d*
level
lvl
params?
patch
plants?
players
price
(pvparena|pvp|arena)\d*
rifts?\d*
s(oul)?w(ar)?s?\d*
zeal\d*
e?xp(erience)?
stats
overall
total
att(ack)?
def(ence)?
str(ength)?
h(it)?p(oints)?
ranged?
pray(er)?
mag(e|ic)
cook(ing)?
w(ood)?c(utting)?
fletch(ing)?
fish(ing)?
f(ire)?m(aking)?
craft(ing)?
smith(ing)?
min(e|ing)
herb(lore)?
agil(ity)?
thie(f|ving)
slay(er)?
farm(ing)?
r(une)?c(raft)?
hunt(er)?
con(struction)?
wiki"
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
        _ => Ok(vec![]),
    } {
        Ok(output) => match CString::new(output.join("\n")) {
            Ok(output) => output.into_raw(),
            Err(_) => nil,
        },
        Err(_) => nil,
    }
}
