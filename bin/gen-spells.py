#!/usr/bin/env python3
"""Generate src/spell/data.rs from the OSRS wiki.

Offline generator, run by hand, output committed -- the same arrangement as
bin/cmb-xp.py and bin/gen-params.py.

Source is every page transcluding {{Infobox Spell}}, reached through the
transclusion API. Deliberately NOT List_of_spells: that page holds 197 spells
against this route's 224, and six of its Standard-spellbook XP values disagree
with the spells' own pages. See the design doc for the measurements.

    python3 bin/gen-spells.py
"""

import pathlib
import re
import sys

import requests

ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT = ROOT / "src" / "spell" / "data.rs"
MAGIC = ROOT / "src" / "stats" / "magic.rs"

# Hardcoded in Skill::defaults(); the run aborts if any stops existing, since
# the generated file would then not compile.
MAGIC_DEFAULTS = [
    "VarrockTeleport", "CamelotTeleport", "ArdougneTeleport",
    "HighLevelAlchemy", "MagicImbue",
]

API = "https://oldschool.runescape.wiki/api.php"
HEADERS = {"User-Agent": "Reinze - https://reinze.com/"}
TEMPLATE = "Template:Infobox Spell"

# The wiki writes these inconsistently cased. Anything outside these sets is a
# new value worth looking at rather than passing through.
SPELLBOOKS = {"normal": "Normal", "ancient": "Ancient", "lunar": "Lunar",
              "arceuus": "Arceuus", "all": "All"}
KINDS = {"combat": "Combat", "teleport": "Teleport", "utility": "Utility"}

INFOBOX = re.compile(r"\{\{Infobox Spell(.*?)\n\}\}", re.S)
RUNEREQ = re.compile(r"\{\{RuneReq\|(.*?)\}\}")
# <ref>..</ref>, <ref/> and {{Refn|..}} glue footnotes onto numbers.
FOOTNOTE = re.compile(r"<ref[^>]*>.*?</ref>|<ref[^>]*/>|\{\{Refn\|.*?\}\}", re.S)


def api(**params):
    params.setdefault("format", "json")
    return requests.get(API, params=params, headers=HEADERS, timeout=60).json()


def spell_pages() -> list[str]:
    """Every page transcluding the infobox."""
    titles, cont = [], {}
    while True:
        response = api(action="query", list="embeddedin", eititle=TEMPLATE,
                       eilimit="500", **cont)
        titles += [p["title"] for p in response["query"]["embeddedin"]]
        if "continue" not in response:
            return titles
        cont = response["continue"]


def page_text(titles: list[str]) -> dict[str, str]:
    """Wikitext for each title, fetched 50 at a time."""
    out = {}
    for i in range(0, len(titles), 50):
        response = api(action="query", prop="revisions", rvprop="content",
                       rvslots="main", titles="|".join(titles[i:i + 50]))
        for page in response["query"]["pages"].values():
            try:
                out[page["title"]] = page["revisions"][0]["slots"]["main"]["*"]
            except (KeyError, IndexError):
                pass
    return out


def field(box: str, key: str) -> str:
    """One infobox field, or "" when absent or blank.

    A blank field is the trap here: `\\|\\s*name\\s*=\\s*(.*)` happily returns
    the *next* line, so a spell with no name comes back called "|image =".
    A value that starts with a pipe is that overrun, not a value.
    """
    match = re.search(r"\|\s*" + re.escape(key) + r"\s*=\s*(.*)", box)
    if not match:
        return ""
    value = FOOTNOTE.sub("", match.group(1)).strip()
    return "" if value.startswith("|") else value


def number(text: str, allow_float: bool = False):
    cleaned = re.sub(r"[^\d.]", "", text)
    if not cleaned:
        return None
    try:
        return float(cleaned) if allow_float else int(float(cleaned))
    except ValueError:
        return None


def runes(cost: str) -> list[tuple[int, str]]:
    """[(count, rune)] in wiki order. Empty means the spell is free."""
    match = RUNEREQ.search(cost)
    if not match:
        return []
    out = []
    for part in match.group(1).split("|"):
        if "=" not in part:
            continue
        rune, count = part.split("=", 1)
        amount = number(count)
        if amount:
            out.append((amount, rune.strip()))
    return out


def parse(pages: dict[str, str]) -> tuple[list[dict], list[str]]:
    spells, skipped = {}, []
    for title, text in sorted(pages.items()):
        boxes = INFOBOX.findall(text)
        if not boxes:
            skipped.append(title)
            continue
        for box in boxes:
            name = field(box, "name") or title
            level = number(field(box, "level"))
            xp = number(field(box, "exp"), allow_float=True)
            if level is None or xp is None:
                skipped.append(title)
                continue
            book = field(box, "spellbook").lower()
            kind = field(box, "type").lower()
            if book not in SPELLBOOKS:
                print(f"  unknown spellbook {book!r} on {title}", file=sys.stderr)
            if kind not in KINDS:
                print(f"  unknown type {kind!r} on {title}", file=sys.stderr)
            spells[name] = {
                "name": name,
                "level": level,
                "xp": xp,
                "members": field(box, "members").lower().startswith("y"),
                "spellbook": SPELLBOOKS.get(book, "Normal"),
                "kind": KINDS.get(kind, "Utility"),
                "damage": number(field(box, "damage")),
                "runes": runes(field(box, "cost")),
            }
    ordered = sorted(spells.values(), key=lambda s: (s["level"], s["name"]))
    return ordered, skipped


def rust_string(value: str) -> str:
    return value.replace("\\", "\\\\").replace('"', '\\"')


def variant(name: str) -> str:
    """The enum identifier for a spell name.

    Same transformation bin/cmb-xp.py uses for NPCs. Verified against the
    current spell set to produce 224 distinct, valid Rust identifiers.
    """
    return re.sub(r"[\[\](){}*.',\s!&\\/%\-]", "", name)


def rust_number(value: float) -> str:
    return f"{value:.1f}" if value != int(value) else f"{int(value)}.0"


def emit(spells: list[dict]) -> str:
    lines = [
        "// @generated by bin/gen-spells.py from the OSRS wiki -- do not edit.",
        f"// {len(spells)} spells transcluding {{{{Infobox Spell}}}}, by level then name.",
        "",
        "/// A spell as the wiki's `{{Infobox Spell}}` describes it.",
        "pub struct Spell {",
        "    pub name: &'static str,",
        "    pub level: u32,",
        "    pub xp: f64,",
        "    pub members: bool,",
        "    /// Normal, Ancient, Lunar, Arceuus or All.",
        "    pub spellbook: &'static str,",
        "    /// Combat, Teleport or Utility.",
        "    pub kind: &'static str,",
        "    /// Max hit, for the combat spells that have one.",
        "    pub damage: Option<u32>,",
        "    /// (count, rune) in wiki order. Empty for the free teleports.",
        "    pub runes: &'static [(u32, &'static str)],",
        "}",
        "",
        "pub const SPELLS: &[Spell] = &[",
    ]
    for spell in spells:
        runes_lit = ", ".join(
            f'({count}, "{rust_string(rune)}")' for count, rune in spell["runes"]
        )
        damage = f"Some({spell['damage']})" if spell["damage"] is not None else "None"
        lines += [
            "    Spell {",
            f'        name: "{rust_string(spell["name"])}",',
            f"        level: {spell['level']},",
            f"        xp: {rust_number(spell['xp'])},",
            f"        members: {str(spell['members']).lower()},",
            f'        spellbook: "{spell["spellbook"]}",',
            f'        kind: "{spell["kind"]}",',
            f"        damage: {damage},",
            f"        runes: &[{runes_lit}],",
            "    },",
        ]
    lines += ["];", ""]
    return "\n".join(lines)


MAGIC_HEAD = '''// @generated by bin/gen-spells.py from the OSRS wiki -- do not edit.
// {count} spells, by level then name. Levels and XP come from each spell's own
// {{{{Infobox Spell}}}}; the hand-maintained version of this file was built from
// List_of_spells, which is stale -- see the spell command design doc.

use crate::stats::skill::{{Detail, Details, IntoString, Multipliers, Skill}};
use regex::Regex;
use std::ops::Add;

pub enum Magic {{
'''

MAGIC_DETAIL = '''}

impl Detail for Magic {
    fn multipliers(&self) -> Vec<Multipliers> {
        vec![]
    }

    fn name(&self) -> String {
        if let Details::Magic(obj) = self.details() {
            return obj.name;
        }

        "".to_string()
    }

    fn level(&self) -> u32 {
        if let Details::Magic(obj) = self.details() {
            return obj.level;
        }

        0
    }

    fn xp(&self) -> f64 {
        if let Details::Magic(obj) = self.details() {
            return obj.xp as f64;
        }

        0.0
    }
}

impl Skill for Magic {
    fn all() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
'''

MAGIC_TAIL = '''        Details::Magic(MagicDetails {
            name: details.0.to_owned(),
            level: details.1,
            xp: details.2,
            members: details.3,
        })
    }

    fn search<T>(query: T) -> Vec<Self>
    where
        T: ToString,
        Self: Sized,
    {
        let mut all = Self::all();

        let q = query.to_string().to_lowercase();

        if let Ok(pattern) = Regex::new(q.as_str()) {
            let mut index = 0;
            all.retain(|activity| {
                if pattern
                    .captures(activity.name().to_lowercase().as_str())
                    .iter()
                    .count()
                    > 0
                    && index < 10
                {
                    index = index.add(1);

                    return true;
                }

                return false;
            });
        } else {
            return vec![];
        }

        all
    }
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct MagicDetails {
    pub name: String,
    pub level: u32,
    pub xp: f64,
    pub members: bool,
}

impl IntoString for MagicDetails {
    fn to_string(&self, s: &crate::stats::skill::Source, xp_difference: f64) -> String {
        format!(
            "{}: {}",
            s.c1(self.name.as_str()),
            s.c2(common::commas_from_string(
                format!("{}", (xp_difference / self.xp).ceil()).as_str(),
                "d"
            )
            .as_str())
        )
    }
}
'''


def emit_magic(spells: list[dict]) -> str:
    """src/stats/magic.rs -- the -mage skill table, same spells as data.rs."""
    out = [MAGIC_HEAD.format(count=len(spells))]
    for spell in spells:
        out.append(f"    {variant(spell['name'])},\n")
    out.append(MAGIC_DETAIL)
    for spell in spells:
        out.append(f"            Self::{variant(spell['name'])},\n")
    out.append("        ]\n    }\n\n    fn defaults() -> Vec<Details> {\n        vec![\n")
    for name in MAGIC_DEFAULTS:
        out.append(f"            Self::{name},\n")
    out.append("        ]\n        .iter()\n        .map(|x| x.details())\n"
               "        .collect()\n    }\n\n"
               "    fn details(&self) -> Details {\n        let details = match self {\n")
    for spell in spells:
        out.append(
            f'            Self::{variant(spell["name"])} => '
            f'("{rust_string(spell["name"])}", {spell["level"]}, '
            f'{rust_number(spell["xp"])}, {str(spell["members"]).lower()}),\n'
        )
    out.append("        };\n\n")
    out.append(MAGIC_TAIL)
    return "".join(out)


def main() -> int:
    titles = spell_pages()
    print(f"{len(titles)} pages transclude {TEMPLATE}")
    spells, skipped = parse(page_text(titles))
    if not spells:
        print("no spells parsed", file=sys.stderr)
        return 1

    # Page count deliberately does not have to equal spell count -- redirects
    # and documentation transclude the template too.
    if skipped:
        print(f"skipped {len(skipped)} page(s) with no usable infobox: "
              f"{', '.join(sorted(skipped)[:5])}")

    # Enum identifiers must be distinct and must still contain the five
    # Skill::defaults() hardcodes, or src/stats/magic.rs will not compile.
    variants = [variant(s["name"]) for s in spells]
    duplicates = sorted({v for v in variants if variants.count(v) > 1})
    if duplicates:
        print(f"duplicate enum variants: {', '.join(duplicates)}", file=sys.stderr)
        return 1
    lost = [d for d in MAGIC_DEFAULTS if d not in variants]
    if lost:
        print(f"Skill::defaults() references missing spells: {', '.join(lost)}",
              file=sys.stderr)
        return 1

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(emit(spells), encoding="utf-8")
    MAGIC.write_text(emit_magic(spells), encoding="utf-8")

    free = sum(1 for s in spells if not s["runes"])
    print(f"  {OUT.relative_to(ROOT)}: {len(spells)} spells "
          f"({free} free, {len(spells) - free} with runes)")
    print(f"  {MAGIC.relative_to(ROOT)}: {len(spells)} variants")
    for book in sorted({s["spellbook"] for s in spells}):
        print(f"    {book:<10} {sum(1 for s in spells if s['spellbook'] == book)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
