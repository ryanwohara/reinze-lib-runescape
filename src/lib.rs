extern crate core;

mod alch;
mod bh;
mod boost;
mod bosses;
mod clues;
mod collectionlog;
mod colosseum;
mod common;
mod ge;
mod grats;
mod items;
mod leagues;
mod level;
mod lms;
mod money;
mod noburn;
mod params;
mod patch;
mod plant;
mod players;
mod prices;
mod pvparena;
mod rifts;
mod rsn;
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
        "alch" | "alchemy" => alch::lookup(query),
        "bh" | "bounty" | "bhunter" | "bountyhunter" => bh::lookup(query, author, rsn_n),
        "boost" | "boosts" => boost::boost(query),
        "boss" | "bosses" | "kc" => bosses::lookup(query, author, rsn_n),
        "clue" | "clues" => clues::lookup(query, author, rsn_n),
        "colo" | "colosseum" => colosseum::lookup(query, author, rsn_n),
        "coll" | "collection" | "collectionlog" => collectionlog::lookup(query, author, rsn_n),
        "congratulations" | "congratulation" | "congrats" | "congratz" | "grats" | "gratz"
        | "gz" => grats::get(query, author),
        "experience" | "xperience" | "exp" | "xp" => xp::xp(query),
        "ge" => ge::ge(query),
        "level" | "lvl" => level::level(query),
        "league" | "leagues" => leagues::lookup(query, author, rsn_n),
        "lms" | "lmstanding" | "lmanstanding" | "lastmstanding" | "lastmanstanding" => {
            lms::lookup(query, author, rsn_n)
        }
        "mp" | "money" | "moneyprinter" | "profit" | "printer" | "profitprinter" => money::printer(query),
        "noburn" | "burn" => noburn::noburn(query),
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
        "rsn" => rsn::process(query, author, rsn_n),
        "sw" | "swar" | "soulw" | "soulwar" | "soulwars" | "zeal" => {
            soulwars::lookup(query, author, rsn_n)
        }
        "wiki" => wiki::wiki(query),
        "help" => Ok(r"alchemy
bh[N]
boost
clues[N]
colosseum[N]
collectionlog[N]
combat[N]
congrats
ge
kc[N]
level
leagues[N]
lms[N]
money
noburn
params
plant
players
price
pvparena[N]
rifts[N]
rsn[N]
sw[N]
stats[N]
wiki
xp"
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()),
        "" => Ok(r"alch(emy)?$
b(ounty)?h(unter)?\d*
boost
boss\d*
kc\d*
clues?\d*
co?mb(at)?\d*
colo(sseum)?\d*
coll(ection(log)?)?\d*
((con)?grat[sz]?(ulations?)?|gz)
^ge
l(ast)?m(an)?s(tanding)?\d*
level
leagues?\d*
lvl
mp|money|moneyprinter|profit|printer|profitprinter
(no)?burn
params?
patch
plants?
players
price
(pvparena|pvp|arena)\d*
rifts?\d*
rsn\d*
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
con(struction)?\d*$
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
