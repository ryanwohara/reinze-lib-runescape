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
use ::common::{PluginContext, to_str_or_default};
use regex::Regex;
use std::ffi::CString;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn exported(context: *const PluginContext) -> *mut c_char {
    unsafe {
        let nil = CString::new("").unwrap().into_raw();

        let mut command = to_str_or_default((*context).cmd);
        let query = to_str_or_default((*context).param);
        let author = to_str_or_default((*context).author);
        let color = (*context).color;

        let re = Regex::new(r"^([a-zA-Z]+)(\d+)$").unwrap();
        let cmd = command.to_string();
        let re_match = match re.captures(&cmd) {
            Some(captures) => vec![captures],
            None => vec![],
        };

        let mut rsn_n = "0";

        if re_match.len() > 0 {
            command = re_match[0].get(1).unwrap().as_str().to_string();
            rsn_n = re_match[0].get(2).unwrap().as_str();
        }

        let source = Source::create(
            rsn_n,
            Author::create(author, color),
            &command.to_string(),
            &query,
        );

        match match command.as_str() {
            "alch" | "alchemy" => alch::lookup(&source),
            "bolt" | "bolts" => bolts::lookup(&source),
            "bh" | "bounty" | "bhunter" | "bountyhunter" => bh::lookup(source),
            "boost" | "boosts" => boost::lookup(&source),
            "boss" | "bosses" | "kc" => bosses::lookup(source),
            "clue" | "clues" => clues::lookup(source),
            "combat" | "cmb" => stats::combat(source),
            "combatest" | "cmbest" | "cmb-est" | "combat-est" => combat_est::estimate(source),
            "colo" | "colosseum" => colosseum::lookup(source),
            "coll" | "collection" | "collectionlog" => collectionlog::lookup(source),
            "congratulations" | "congratulation" | "congrats" | "congratz" | "grats" | "gratz"
            | "gz" => grats::get(&source),
            "experience" | "xperience" | "exp" | "xp" => xp::lookup(&source),
            "fairy" => fairy::lookup(source),
            "ge" => ge::lookup(&source),
            "grid" => gridmaster::lookup(source),
            "level" | "lvl" => level::lookup(&source),
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
            | "smith" | "mining" | "mine" | "herblore" | "herb" | "agility" | "agil"
            | "thieving" | "thief" | "slayer" | "slay" | "farming" | "farm" | "runecraft"
            | "rc" | "hunter" | "hunt" | "construction" | "con" | "sail" | "sailing" => {
                stats::lookup(source)
            }
            "payment" | "plant" | "plants" => plant::lookup(&source),
            "pvparena" | "pvp" | "arena" => pvparena::lookup(source),
            "rift" | "rifts" => rifts::lookup(source),
            "rsn" => rsn::process(source),
            "salvage" | "salvages" => salvage::lookup(&source),
            "sw" | "swar" | "soulw" | "soulwar" | "soulwars" | "zeal" => soulwars::lookup(source),
            "togw" => tog::world(),
            "wiki" => wiki::query(&source),
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
}
