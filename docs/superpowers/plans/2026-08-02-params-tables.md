# Params tables migration — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move `Database.ini` into this repo and generate one Rust file per skill for the 17 non-combat skills, so `+params` reads compiled tables instead of re-parsing a file from another repository on every invocation.

**Architecture:** A committed Python generator reads the INI and emits `src/params/<skill>.rs`, each a sorted-as-in-file `&[(&str, &str)]` slice. `params.rs` dispatches a skill to its table, falling back to the INI — embedded with `include_str!` and parsed once into a `OnceLock` — for the six combat/Slayer sections that are not yet migrated. A Rust test asserts the generated tables and the INI agree in both directions.

**Tech Stack:** Rust (`rust-ini 0.21`, `std::sync::OnceLock`, no new crates). Python 3 standard library for the generator. No build script.

## Global Constraints

- **TWO REPOSITORIES.** `/home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape` (the plugin — this is where nearly all the work happens) and `.../rust-reinze` (the bot, which only loses a file). They are separate git repos; `rust-reinze` has no cargo dependency on the plugin.
- **Branches.** Plugin work is on `feat/params-tables`, already created, spec committed as `388bb93`. The `rust-reinze` change needs its own branch created off `main` before committing there.
- **The crate has 188 passing tests.** Run with `cd reinze-lib-runescape && cargo test`. No existing test may break.
- **`src/params/*.rs` are GENERATED.** Never hand-edit them. Every one carries an `@generated` header. If a value looks wrong, fix `lib/Database.ini` and regenerate.
- **Entries keep INI file order — do NOT sort.** `[Prayer]` is deliberately two blocks separated by a blank line (bones and ashes, then ensouled heads), each sorted within itself, so the section as a whole is not globally sorted. The drift test asserts order, so a global sort fails the suite.
- **Values are `&str`, never numbers.** Farming and Hunter carry composites (`Apple_tree=22-1199.5-8.5`, `Baby_impling=18/20`) and Woodcutting has `Bloodwood_(chop)=~30`. Nothing does arithmetic on them.
- **No XP value may change.** This is a storage migration. The drift test is what proves it.
- **The INI uses CRLF.** Preserve it when moving; `git mv` does.
- **Do not touch** the other files in `rust-reinze/lib/` — `bolts.ini`, `Ini.ini`, `rdrop.ini`, `patch.ini`, `plants.ini`, `item.ini` and the JSON files belong to other commands.
- **Commit style.** This repo uses conventional commits with a scope (`feat(params):`, `fix(chef):`). Bodies wrap at 72 columns and end with `Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>`.

**Verified API facts you will need:**

```rust
ini::Ini::load_from_str(&str) -> Result<Ini, ParseError>
Ini::sections(&self)  -> impl DoubleEndedIterator<Item = Option<&str>>
Ini::section(&self, Option<S>) -> Option<&Properties>
Properties::iter(&self) -> PropertyIter          // yields (&str, &str), duplicates included
```

**Current `params.rs` shape** (after the ranking change merged as `1e86aa8`): `lookup` resolves the skill via `common_skill`, loads the INI with `Ini::load_from_file("lib/Database.ini")`, selects the section, builds `let entries: Vec<(&str, &str)> = section.iter().collect();`, and passes that to `rank_matches(&entries, param)` which returns ranked pairs. `rank_matches` is `pub(crate) fn rank_matches<'a>(entries: &[(&'a str, &'a str)], query: &str) -> Vec<(&'a str, &'a str)>`.

---

### Task 1: The generator and its output

**Files:**
- Create: `reinze-lib-runescape/bin/gen-params.py`
- Create (generated): `reinze-lib-runescape/src/params/<skill>.rs` × 17

**Interfaces:**
- Consumes: `lib/Database.ini`, which at this point still lives in `rust-reinze`. Task 2 moves it.
- Produces: 17 files each exposing `pub const ENTRIES: &[(&str, &str)]`.

The generated files are inert this task — nothing declares them as modules yet, so `cargo` ignores them and the build stays green. That is deliberate: it makes this task independently reviewable.

- [ ] **Step 1: Write the generator**

Create `reinze-lib-runescape/bin/gen-params.py`:

```python
#!/usr/bin/env python3
"""Generate src/params/<skill>.rs from lib/Database.ini.

Offline generator, run by hand, output committed -- the same arrangement as
bin/cmb-xp.py producing src/npc/data.rs. Entries keep INI file order; the
Prayer section is deliberately two blocks and must not be re-sorted.
"""

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT = ROOT / "src" / "params"

# Served from the embedded INI until the npc/data.rs consolidation lands.
COMBAT = {"Attack", "Strength", "Defence", "Ranged", "Hitpoints", "Slayer"}


def read_sections(text: str) -> dict[str, list[tuple[str, str]]]:
    """Section name -> [(key, value)] in file order. Blank lines are skipped."""
    out: dict[str, list[tuple[str, str]]] = {}
    current = None
    for line in text.replace("\r\n", "\n").split("\n"):
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            current = stripped[1:-1]
            out[current] = []
        elif current is not None and stripped and "=" in stripped:
            key, value = stripped.split("=", 1)
            out[current].append((key.strip(), value.strip()))
    return out


def rust_string(value: str) -> str:
    """A Rust string literal body. Keys carry apostrophes and ampersands."""
    return value.replace("\\", "\\\\").replace('"', '\\"')


def main() -> int:
    ini_path = ROOT / "lib" / "Database.ini"
    if not ini_path.exists():
        # Task 1 runs before the file has moved into this repo.
        ini_path = ROOT.parent / "rust-reinze" / "lib" / "Database.ini"
    if not ini_path.exists():
        print(f"cannot find Database.ini (looked in {ROOT / 'lib'})", file=sys.stderr)
        return 1

    sections = read_sections(ini_path.read_text(encoding="utf-8"))
    OUT.mkdir(parents=True, exist_ok=True)

    written = []
    for name, entries in sections.items():
        if name in COMBAT:
            continue
        module = name.lower()
        lines = [
            "// @generated by bin/gen-params.py from lib/Database.ini -- do not edit.",
            f"// Section [{name}], {len(entries)} entries, in INI file order.",
            "",
            "pub const ENTRIES: &[(&str, &str)] = &[",
        ]
        for key, value in entries:
            lines.append(f'    ("{rust_string(key)}", "{rust_string(value)}"),')
        lines.append("];")
        lines.append("")
        (OUT / f"{module}.rs").write_text("\n".join(lines), encoding="utf-8")
        written.append((name, module, len(entries)))

    for name, module, count in sorted(written):
        print(f"  {module + '.rs':22} [{name}] {count} entries")
    print(f"{len(written)} files, {sum(c for _, _, c in written)} entries")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

- [ ] **Step 2: Run it**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape \
  && chmod +x bin/gen-params.py && python3 bin/gen-params.py
```

Expected: 17 files and 2508 entries. The per-skill counts must be exactly Agility 29, Construction 483, Cooking 154, Crafting 495, Farming 94, Firemaking 36, Fishing 56, Fletching 121, Herblore 110, Hunter 86, Magic 192, Mining 53, Prayer 73, Runecraft 35, Smithing 358, Thieving 96, Woodcutting 37. Paste the output.

If any count differs, stop — the INI has changed since this plan was written and the discrepancy must be understood before continuing.

- [ ] **Step 3: Check the output is well-formed Rust and faithful**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape && python3 - <<'EOF'
import pathlib, re
out = pathlib.Path("src/params")
files = sorted(out.glob("*.rs"))
print("files:", len(files))
total = 0
for f in files:
    text = f.read_text()
    assert text.startswith("// @generated"), f"{f.name} missing @generated header"
    rows = re.findall(r'^    \("(.*?)", "(.*?)"\),$', text, re.M)
    declared = int(re.search(r"(\d+) entries", text).group(1))
    assert len(rows) == declared, f"{f.name}: {len(rows)} rows vs {declared} declared"
    assert not [k for k, _ in rows if '"' in k or "\\" in k], f"{f.name}: unescaped key"
    assert not [v for _, v in rows if v == ""], f"{f.name}: empty value"
    total += len(rows)
print("total entries:", total)
EOF
```

Expected: `files: 17`, `total entries: 2508`, no assertion.

- [ ] **Step 4: Confirm Prayer's two-block order survived**

The Prayer section is not globally sorted. Verify the generated file preserves the INI's order rather than sorting it:

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape && python3 -c "
import re
rows = re.findall(r'^    \(\"(.*?)\", ', open('src/params/prayer.rs').read(), re.M)
print('prayer entries:', len(rows))
print('globally sorted?', rows == sorted(rows, key=str.lower))
print('first block ends / second begins:', rows[48:52])
"
```

Expected: 73 entries, `globally sorted? False`, and the boundary showing the last bones/ashes key followed by the first `Ensouled_*` key. If it reports `True`, the generator sorted and must be fixed.

- [ ] **Step 5: Confirm the build is untouched**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape && cargo test --quiet 2>&1 | tail -4
```

Expected: `test result: ok`, 188 passed. The new files are not declared as modules yet, so cargo ignores them entirely.

- [ ] **Step 6: Commit**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape \
  && git add bin/gen-params.py src/params \
  && git commit -m "$(cat <<'EOF'
feat(params): add the table generator and its output

bin/gen-params.py reads Database.ini and writes one file per non-combat
skill under src/params, each a slice of key and value string pairs in
INI file order. Offline generator with committed output, matching how
bin/cmb-xp.py produces src/npc/data.rs.

Values stay strings because Farming and Hunter pack several numbers
into one entry and Woodcutting carries an approximation.

Nothing declares these as modules yet, so the build ignores them. The
next commit moves the INI into this repo and wires them up.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)" && git show --stat HEAD | tail -5
```

Expected: 18 files changed (the generator plus 17 tables).

---

### Task 2: Move the INI into this repo and embed it

**Files:**
- Move: `rust-reinze/lib/Database.ini` → `reinze-lib-runescape/lib/Database.ini`
- Modify: `reinze-lib-runescape/src/params.rs`

**Interfaces:**
- Consumes: nothing from Task 1 — the generated tables stay unused until Task 3.
- Produces: `const DATABASE_INI: &str` and `fn database() -> &'static Ini` in `params.rs`, both used by Task 3.

Behaviour must be **identical** after this task. The only change is where the data comes from and how often it is parsed.

- [ ] **Step 1: Move the file**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/rust-reinze \
  && git mv lib/Database.ini /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape/lib/Database.ini 2>/dev/null \
  || { mkdir -p /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape/lib \
       && cp lib/Database.ini /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape/lib/Database.ini \
       && git rm --cached lib/Database.ini && rm lib/Database.ini; }
```

`git mv` cannot cross repositories, so the fallback copies then removes. Afterwards verify the copy is byte-identical to what was committed:

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/rust-reinze \
  && git show HEAD:lib/Database.ini | cmp - /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape/lib/Database.ini \
  && echo "byte-identical" \
  && python3 -c "
raw=open('/home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape/lib/Database.ini',newline='').read()
print('CRLF lines:', raw.count(chr(13)+chr(10)), '| bare LF:', raw.count(chr(10))-raw.count(chr(13)+chr(10)))
"
```

Expected: `byte-identical`, 5995 CRLF lines, 0 bare LF.

- [ ] **Step 2: Embed and cache it in `params.rs`**

Add near the top of `src/params.rs`, after the existing `use` lines:

```rust
use std::sync::OnceLock;

/// The XP database. Embedded rather than read at runtime: the old
/// `load_from_file` path resolved against the bot's working directory, which
/// is what kept this file in the other repository.
const DATABASE_INI: &str = include_str!("../lib/Database.ini");

/// Parsed once per process. The previous code re-read and re-parsed 116 KB on
/// every invocation of the command.
fn database() -> &'static Ini {
    static DB: OnceLock<Ini> = OnceLock::new();
    DB.get_or_init(|| Ini::load_from_str(DATABASE_INI).expect("embedded Database.ini must parse"))
}
```

Then replace this block in `lookup`:

```rust
    let database = Ini::load_from_file("lib/Database.ini").map_err(|e| {
        error!("Error loading Database.ini: {}", e);
        anyhow::anyhow!("Error loading Database.ini: {}", e)
    })?;

    let prefix = s.l(&capitalize(&skill));

    let section = match database.section(Some(capitalize(&skill))) {
```

with:

```rust
    let prefix = s.l(&capitalize(&skill));

    let section = match database().section(Some(capitalize(&skill))) {
```

`log::error` may now be unused in this file — if the compiler warns, remove the `use log::error;` import. Do not remove it if other code in the file still uses it.

- [ ] **Step 3: Build and test**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape && cargo test --quiet 2>&1 | tail -4
```

Expected: `test result: ok`, 188 passed, and no warnings about an unused import or unused variable.

- [ ] **Step 4: Prove behaviour is unchanged**

The suite never exercises the data path, so verify by hand with a temporary test. Add to the `mod tests` block:

```rust
    #[test]
    fn ctl_sample_lookups() {
        for (skill, query) in [
            ("Smithing", "cannonball"),
            ("Crafting", "amulet"),
            ("Magic", "teleport"),
            ("Prayer", "bones"),
            ("Attack", "dragon"),
            ("Slayer", "kalphite"),
        ] {
            let props = database().section(Some(skill)).expect("section");
            let entries: Vec<(&str, &str)> = props.iter().collect();
            let top: Vec<&str> = rank_matches(&entries, query)
                .into_iter()
                .take(5)
                .map(|(k, _)| k)
                .collect();
            println!("[{skill}] {query:?} -> {} matches, top 5: {top:?}", entries.len());
        }
    }
```

Run it, paste the output, then DELETE the test and confirm with `grep -c ctl_sample_lookups src/params.rs` run on its own (it exits 1 on zero matches, so do not chain it with `&&`):

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape \
  && cargo test --quiet ctl_sample_lookups -- --nocapture 2>&1 | tail -12
```

Keep this output — Task 3 must reproduce it exactly.

- [ ] **Step 5: Commit in this repo**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape \
  && git add lib/Database.ini src/params.rs \
  && git commit -m "$(cat <<'EOF'
feat(params): own Database.ini and embed it

The file moves here from the bot repo. It is embedded with include_str
rather than read from disk, because load_from_file resolved against the
bot's working directory and that is precisely what stopped the data
moving to the repository that consumes it.

Parsing now happens once per process behind a OnceLock instead of on
every invocation of the command.

No behaviour changes; the same section is read for every skill. The
generated tables added in the previous commit are still unused.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)" && git show --stat HEAD | tail -4
```

- [ ] **Step 6: Commit the removal in the bot repo**

`rust-reinze` is on `main`; branch first.

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/rust-reinze \
  && git checkout -b chore/drop-database-ini \
  && git add -u lib/Database.ini \
  && git commit -m "$(cat <<'EOF'
chore: remove Database.ini, now owned by reinze-lib-runescape

The only consumer is the params command in the plugin crate, which now
embeds the file at compile time. Nothing in this repository reads it.

The other files under lib/ are unaffected; they belong to other
commands.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)" && git show --stat HEAD | tail -3 && cargo test --quiet 2>&1 | tail -3
```

Expected: one file changed (a deletion), and `rust-reinze`'s own 17 tests still pass.

---

### Task 3: Wire the generated tables and guard against drift

**Files:**
- Modify: `reinze-lib-runescape/src/params.rs`

**Interfaces:**
- Consumes: the 17 `ENTRIES` slices from Task 1, and `DATABASE_INI` / `database()` from Task 2.
- Produces: nothing further. This is the last task.

- [ ] **Step 1: Declare the modules and the dispatch**

Add to `src/params.rs`, above `lookup`:

```rust
mod agility;
mod construction;
mod cooking;
mod crafting;
mod farming;
mod firemaking;
mod fishing;
mod fletching;
mod herblore;
mod hunter;
mod magic;
mod mining;
mod prayer;
mod runecraft;
mod smithing;
mod thieving;
mod woodcutting;

/// Sections still served from the embedded INI, pending the npc/data.rs
/// consolidation. Everything else has a generated table.
const INI_SECTIONS: &[&str] = &[
    "Attack",
    "Defence",
    "Hitpoints",
    "Ranged",
    "Slayer",
    "Strength",
];

/// The generated table for a capitalised skill name, or None if that skill is
/// still served from the embedded INI.
fn table_for(skill: &str) -> Option<&'static [(&'static str, &'static str)]> {
    Some(match skill {
        "Agility" => agility::ENTRIES,
        "Construction" => construction::ENTRIES,
        "Cooking" => cooking::ENTRIES,
        "Crafting" => crafting::ENTRIES,
        "Farming" => farming::ENTRIES,
        "Firemaking" => firemaking::ENTRIES,
        "Fishing" => fishing::ENTRIES,
        "Fletching" => fletching::ENTRIES,
        "Herblore" => herblore::ENTRIES,
        "Hunter" => hunter::ENTRIES,
        "Magic" => magic::ENTRIES,
        "Mining" => mining::ENTRIES,
        "Prayer" => prayer::ENTRIES,
        "Runecraft" => runecraft::ENTRIES,
        "Smithing" => smithing::ENTRIES,
        "Thieving" => thieving::ENTRIES,
        "Woodcutting" => woodcutting::ENTRIES,
        _ => return None,
    })
}
```

- [ ] **Step 2: Use the table, falling back to the INI**

In `lookup`, this is the exact block as it stands after Task 2 — four statements between the skill guard and the `rank_matches` call:

```rust
    let prefix = s.l(&capitalize(&skill));

    let section = match database().section(Some(capitalize(&skill))) {
        Some(section) => section,
        _ => return Ok(vec![format!("{} {}", prefix, s.c1("No results found"))]),
    };

    let entries: Vec<(&str, &str)> = section.iter().collect();
```

Replace all of it with:

```rust
    let name = capitalize(&skill);
    let prefix = s.l(&name);

    let entries: Vec<(&str, &str)> = match table_for(&name) {
        Some(table) => table.to_vec(),
        None => match database().section(Some(name.clone())) {
            Some(section) => section.iter().collect(),
            None => return Ok(vec![format!("{} {}", prefix, s.c1("No results found"))]),
        },
    };
```

The `rank_matches(&entries, param)` call below it is unchanged. Two details worth not getting wrong: `Ini::section` is generic over `Into<SectionKey>`, so pass an owned `name.clone()` exactly as the current code passes `capitalize(&skill)` — a `&String` will not coerce. And both match arms yield `'static` data, since the tables are consts and `database()` returns `&'static Ini`, so the `Vec` borrows nothing local and no lifetime annotation is needed.

- [ ] **Step 3: Write the drift test**

Add to the `mod tests` block. This is the correctness gate for the whole migration:

```rust
    #[test]
    fn generated_tables_match_the_ini() {
        let ini = Ini::load_from_str(DATABASE_INI).expect("embedded ini parses");
        let mut checked = 0;

        for section in ini.sections() {
            let Some(name) = section else { continue };
            if INI_SECTIONS.contains(&name) {
                assert!(
                    table_for(name).is_none(),
                    "[{name}] is listed as INI-served but also has a generated table"
                );
                continue;
            }

            let table = table_for(name).unwrap_or_else(|| {
                panic!("[{name}] has no generated table; run bin/gen-params.py and add it to table_for")
            });
            let from_ini: Vec<(&str, &str)> = ini
                .section(Some(name))
                .expect("section exists")
                .iter()
                .collect();

            assert_eq!(
                table.len(),
                from_ini.len(),
                "[{name}] has {} generated entries but {} in the INI; regenerate",
                table.len(),
                from_ini.len()
            );
            for (i, (generated, ini_entry)) in table.iter().zip(from_ini.iter()).enumerate() {
                assert_eq!(
                    generated, ini_entry,
                    "[{name}] entry {i} differs; regenerate"
                );
            }
            checked += 1;
        }

        assert_eq!(checked, 17, "expected 17 generated sections, checked {checked}");
    }
```

It asserts in both directions: every generated table matches its section entry for entry and in order, and every non-combat section in the INI has a table. Forgetting to regenerate, or adding a skill without wiring it into `table_for`, fails here with the skill named.

- [ ] **Step 4: Run the suite**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape && cargo test --quiet 2>&1 | tail -4
```

Expected: `test result: ok`, 189 passed (188 plus the drift test).

- [ ] **Step 5: Prove output is unchanged**

Re-add the temporary probe from Task 2 Step 4, adapted to go through the new dispatch so it exercises the tables rather than the INI:

```rust
    #[test]
    fn ctl_sample_lookups() {
        for (skill, query) in [
            ("Smithing", "cannonball"),
            ("Crafting", "amulet"),
            ("Magic", "teleport"),
            ("Prayer", "bones"),
            ("Attack", "dragon"),
            ("Slayer", "kalphite"),
        ] {
            let entries: Vec<(&str, &str)> = match table_for(skill) {
                Some(table) => table.to_vec(),
                None => database().section(Some(skill)).expect("section").iter().collect(),
            };
            let top: Vec<&str> = rank_matches(&entries, query)
                .into_iter()
                .take(5)
                .map(|(k, _)| k)
                .collect();
            println!("[{skill}] {query:?} -> {} matches, top 5: {top:?}", entries.len());
        }
    }
```

Run with `cargo test --quiet ctl_sample_lookups -- --nocapture`. The output must be **character-for-character identical** to Task 2 Step 4's. Paste both side by side. The first four now come from generated tables and the last two from the INI, so identical output is what proves the migration changed nothing.

Then DELETE the test and confirm it is gone with `grep -c ctl_sample_lookups src/params.rs` run on its own (it exits 1 on zero matches, so do not chain it with `&&`).

- [ ] **Step 6: Confirm no generated file was hand-edited**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape \
  && python3 bin/gen-params.py > /dev/null && git status --short src/params
```

Expected: no output — regenerating produces byte-identical files, proving nothing under `src/params/` was touched by hand.

- [ ] **Step 7: Commit**

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape \
  && git add src/params.rs \
  && git commit -m "$(cat <<'EOF'
feat(params): read generated tables instead of the INI

The seventeen non-combat skills now resolve through table_for to a
compiled slice. Attack, Strength, Defence, Ranged, Hitpoints and Slayer
still come from the embedded INI, pending the consolidation with
npc/data.rs, so table_for returns None for them and the lookup falls
back.

A test parses the embedded INI and asserts in both directions: every
generated table matches its section entry for entry and in order, and
every non-combat section has a table. Forgetting to regenerate, or
adding a skill without wiring it up, fails with the skill named rather
than drifting silently.

Sample lookups across four generated skills and two INI-served ones
produce identical output to before the change.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>
EOF
)" && git show --stat HEAD | tail -3
```

---

## Final verification

Run every command and paste the output.

```bash
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/reinze-lib-runescape \
  && git log --oneline -4 && git status --short && cargo test --quiet 2>&1 | tail -4
cd /home/rohara/.agent-deck/multi-repo-worktrees/788b004e/rust-reinze \
  && git log --oneline -2 && git status --short && ls lib/
```

- [ ] `reinze-lib-runescape` shows three `feat(params):` commits on top of `388bb93`.
- [ ] `cargo test` passes with 189 tests, including `generated_tables_match_the_ini`.
- [ ] `reinze-lib-runescape/git status --short` shows no `ctl_sample_lookups` leftovers and no modified files under `src/params/`.
- [ ] `rust-reinze` shows one `chore:` commit on branch `chore/drop-database-ini`, `lib/Database.ini` gone, and the other seven files under `lib/` still present.
- [ ] `rust-reinze`'s own tests still pass.
- [ ] `python3 bin/gen-params.py` followed by `git status --short src/params` produces no output.
- [ ] No file under `src/params/` lacks the `@generated` header.
