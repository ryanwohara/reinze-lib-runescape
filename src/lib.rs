extern crate core;

mod alch;
mod bh;
mod bolts;
mod boost;
mod bosses;
mod clues;
mod collectionlog;
mod colosseum;
mod combat_est;
mod common;
mod fairy;
mod ge;
mod grats;
mod gridmaster;
mod items;
mod leagues;
mod level;
mod lms;
mod money;
mod noburn;
mod npc;
mod params;
mod patch;
mod plant;
mod players;
mod prices;
mod pvparena;
mod rifts;
mod rsn;
mod salvage;
mod soulwars;
mod stats;
mod tog;
mod wiki;
mod xp;

use ::common::author::Author;
use ::common::source::Source;
use regex::Regex;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn exported(
    cmd: *const c_char,
    raw_query: *const c_char,
    raw_author: *const c_char,
) -> *mut c_char {
    let nil = CString::new("").unwrap().into_raw(); // using unwrap() here is safe because we know the string is valid UTF-8

    if cmd.is_null() || raw_query.is_null() || raw_author.is_null() {
        return nil;
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

    let source = Source::create(rsn_n, Author::create(author), command, query);

    match match command {
        "alch" | "alchemy" => alch::lookup(query),
        "bolt" | "bolts" => bolts::lookup(query),
        "bh" | "bounty" | "bhunter" | "bountyhunter" => bh::lookup(source),
        "boost" | "boosts" => boost::boost(query),
        "boss" | "bosses" | "kc" => bosses::lookup(source),
        "clue" | "clues" => clues::lookup(source),
        "combat" | "cmb" => stats::combat(source),
        "combatest" | "cmbest" | "cmb-est" | "combat-est" => combat_est::estimate(source),
        "colo" | "colosseum" => colosseum::lookup(source),
        "coll" | "collection" | "collectionlog" => collectionlog::lookup(source),
        "congratulations" | "congratulation" | "congrats" | "congratz" | "grats" | "gratz"
        | "gz" => grats::get(query, author),
        "experience" | "xperience" | "exp" | "xp" => xp::xp(query),
        "fairy" => fairy::lookup(source),
        "ge" => ge::ge(query),
        "grid" => gridmaster::lookup(source),
        "level" | "lvl" => level::level(query),
        "league" | "leagues" => leagues::lookup(source),
        "lms" | "lmstanding" | "lmanstanding" | "lastmstanding" | "lastmanstanding" => {
            lms::lookup(source)
        }
        "mp" | "money" | "moneyprinter" | "profit" | "printer" | "profitprinter" => {
            money::printer(&source)
        }
        "noburn" | "burn" => noburn::noburn(&source),
        "npc" => npc::lookup(&source),
        "param" | "params" => params::lookup(&source),
        "patch" => patch::patch(&source),
        "players" => players::lookup(&source),
        "price" => prices::lookup(&source),
        "overall" | "stats" | "total" | "attack" | "att" | "defence" | "def" | "strength"
        | "str" | "hitpoints" | "hp" | "ranged" | "range" | "prayer" | "pray" | "magic"
        | "mage" | "cooking" | "cook" | "woodcutting" | "wc" | "fletching" | "fletch"
        | "fishing" | "fish" | "firemaking" | "fm" | "crafting" | "craft" | "smithing"
        | "smith" | "mining" | "mine" | "herblore" | "herb" | "agility" | "agil" | "thieving"
        | "thief" | "slayer" | "slay" | "farming" | "farm" | "runecraft" | "rc" | "hunter"
        | "hunt" | "construction" | "con" | "sail" | "sailing" => stats::lookup(source),
        "payment" | "plant" | "plants" => plant::lookup(&source),
        "pvparena" | "pvp" | "arena" => pvparena::lookup(source),
        "rift" | "rifts" => rifts::lookup(source),
        "rsn" => rsn::process(source),
        "salvage" | "salvages" => salvage::lookup(query),
        "sw" | "swar" | "soulw" | "soulwar" | "soulwars" | "zeal" => soulwars::lookup(source),
        "togw" => tog::world(),
        "wiki" => wiki::wiki(query),
        "help" => Ok(r"alchemy
bolts
bh[N]
boost
clues[N]
colosseum[N]
collectionlog[N]
combat[N]
combat-est
congrats
fairy
ge
grid[N]
kc[N]
level
leagues[N]
lms[N]
money
noburn
npc
params
plant
players
price
pvparena[N]
rifts[N]
rsn[N]
salvage
stats[N]
sw[N]
togw
wiki
xp"
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()),
        "" => Ok(r"alch(emy)?$
bolts?
b(ounty)?h(unter)?\d*
boost
boss\d*
fairy
kc\d*
clues?\d*
co?mb(at)?\d*$
co?mb(at)?-?est
colo(sseum)?\d*$
coll(ection(log)?)?\d*
((con)?grat[sz]?(ulations?)?|gz)
^ge
^grid
l(ast)?m(an)?s(tanding)?\d*
level
leagues?\d*
lvl
mp|money|moneyprinter|profit|printer|profitprinter
(no)?burn
npc
params?
patch
payment|plants?
players
price
(pvparena|pvp|arena)\d*
rifts?\d*
rsn\d*
s(oul)?w(ar)?s?\d*
zeal\d*
e?xp(erience)?
salvages?
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
sail(ing)?\d*
togw
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
