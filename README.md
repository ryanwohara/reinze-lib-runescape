# reinze-lib-runescape

RuneScape command plugin for the [reinze](https://reinze.com) IRC bot. It builds
to a shared library (`.so`) that the `rust-reinze` host loads dynamically at
runtime, exposing Old School RuneScape lookups — hiscores, calculators, Grand
Exchange prices, and reference data — as IRC commands.

Output from a `-command` is private (messaged to the caller); output from a
`+command` is public. A trailing `[N]` on a command uses the caller's Nth stored
RSN (see `-rsn`). Most player lookups accept an explicit RSN, or fall back to the
caller's stored RSN — or IRC nick — when none is given.

## Commands

### Accounts & Stats

- `-rsn[N] set <RSN> | show | del | list` — Manage the RuneScape names stored for
  you and used as the default lookup target elsewhere. `set` saves/updates slot
  [N] (default 0); `show`/`del` display or remove it; `list` shows every slot.
- `-stats[N] [flags] [RSN]` — Report a player's combat level and every skill's
  level in one line (aliases `-overall`, `-total`). Account-type flags: `-i`
  ironman, `-u` ultimate, `-h` hardcore, `-d` deadman, `-l` leagues, `-t`
  tournament, `-1` one-defence, `-sk` skiller, `-fs` fresh start. Display flags:
  `-s` sort by XP to next level, `-o` sort by XP but show levels, `-r` rank,
  `-e`/`-x` XP. A comparison (`< <= > >= =`) filters to matching skills.
- `-combat[N] [flags] [RSN]` — Show total combat level and style, each combat
  skill's level/XP, and the XP each needs to raise combat by one (alias `-cmb`).
- `-track[N] [RSN] [@duration] [^name ...]` — Diff a player's live hiscores
  against a saved snapshot, reporting per-skill and activity changes. No duration
  diffs the most recent snapshot; `@3d`/`@1w`/`@12h`/`@2w3d` diffs that far back.
  Records a fresh snapshot each call. `^name` narrows the report to specific rows
  and may be repeated — `^mining`, `^mine` and `^att` go through the skill
  aliases, `^zulrah`/`^cox`/`^clue` match any hiscore row by substring. A
  requested row that did not change reports its current standing instead. A
  short token matches the first row whose name contains it, so `^king` finds
  Cooking rather than King Black Dragon — spell more of the name to fix it
  (`^dragon`, not `^black dragon`; a token is a single non-space run, so
  spaces don't work). An unresolvable token is reported inline as
  `no match for 'x'` beside the columns that did resolve; if every `^name`
  token is unresolvable, the command short-circuits and records no snapshot.
- `-players` — Report OSRS and RS3 players online (and each side's share), the
  combined total, and total registered accounts, live from Jagex.

### Calculators

- `-(skill)[N] [#goal] [^input XP] [@method] [RSN]` — For one skill, report level,
  XP, rank, and XP/percent to the next level, plus suggested training methods and
  quantities. `#N` targets a level/XP, `^N` simulates a starting level/XP, `@term`
  filters methods. One command per skill (e.g. `-attack`/`-att`, `-mining`/`-mine`,
  `-construction`/`-con`, `-sailing`/`-sail`).
- `-lvl (level) | (start)-(end)` — Convert a level (capped at 126) to the XP
  required, or report the XP difference between two levels.
- `-xp (amount)` — Convert an XP amount (0–200,000,000; `k`/`m`/`b` suffixes) to
  the corresponding level (aliases `-exp`, `-experience`).
- `-params (skill) (search term)` — Look up actions/items for a skill in the XP
  database (up to 10 matches with their XP) (alias `-param`).
- `-combat-est (99a 90s 70d 10h 1p 1r 1m ...)` — Estimate a combat level from
  manually supplied skill levels; output mirrors `-combat` (aliases `-combatest`,
  `-cmbest`, `-cmb-est`).
- `-herbi[N] [#goal] [^level] [RSN]` — How many herbiboars stand between a
  player's Hunter level and their next level or `#goal`, plus the XP per catch
  and the estimated time and gp along the way (alias `-herbiboar`). XP per catch
  scales with Hunter level (1,770 at 74 → 2,461 at 99) and is re-rated as you
  level. Requires 80 Hunter, or 74 with a super hunter potion. `^level` computes
  from a hypothetical level or raw XP instead of a hiscores lookup; time and gp
  are estimates (~60 catches and ~400k gp per hour with a herb sack).

### Bosses & Minigames

- `-kc[N] [RSN] [@boss]` — Boss and raid kill counts with rank; `@name` filters
  (aliases `-boss`, `-bosses`).
- `-clues[N] [RSN] [@tier]` — Completed clue scroll counts and rank per tier
  (alias `-clue`).
- `-colosseum[N] [RSN]` — Fortis Colosseum Glory score and rank (alias `-colo`).
- `-collectionlog[N] [RSN]` — Collection Log count and rank (aliases `-coll`,
  `-collection`).
- `-lms[N] [RSN]` — Last Man Standing score and rank.
- `-pvparena[N] [RSN]` — PVP Arena score and rank (aliases `-pvp`, `-arena`).
- `-rifts[N] [RSN]` — Guardians of the Rift score and rank (alias `-rift`).
- `-sw[N] [RSN]` — Soul Wars Zeal score and rank (aliases `-swar`, `-soulw`,
  `-soulwar`, `-soulwars`, `-zeal`).
- `-bh[N] [RSN]` — Bounty Hunter score and rank across Hunter, Rogue, Legacy
  Hunter, and Legacy Rogue (aliases `-bounty`, `-bhunter`, `-bountyhunter`).
- `-leagues[N] [RSN]` — Current-season Leagues score and rank (alias `-league`).
- `-grid[N] [RSN]` — Grid Master (Tournament-mode) score and rank.
- `-salvage (shipwreck)` — A Shipwreck Salvage wreck's Salvaging level, XP,
  average lifespan, and map locations (alias `-salvages`).
- `-task[N] [RSN] @(count) (monster)` — Total Slayer/Combat/Hitpoints XP for
  killing a monster `@count` times, and a projected Slayer level if the RSN has
  Slayer XP. Unknown monsters return suggestions.

### Economy

- `-ge (item)` — Live Grand Exchange price(s). Supports regex/abbreviation search,
  comma-separated queries, and quantity prefixes (`5 rune scimitar`), with a
  running total.
- `-price (item)` — Like `-ge` but from a cached snapshot; up to 10 items.
- `-alch (item)` — High and low alchemy values, optionally times a quantity
  (alias `-alchemy`); up to 10 items.
- `-money [-l (limit)]` — High-alch "money maker" items ranked by profit after GE
  buy price and a nature rune (top 15); `-l 100k` filters by GE buy limit (aliases
  `-mp`, `-profit`, `-printer`, `-moneyprinter`, `-profitprinter`).
- `-bolts (bolt)` — The special effect of an enchanted crossbow bolt tip, by gem
  or ability name (alias `-bolt`).

### Reference

- `-boost (skill)` — Food, potion, and stew boosts for a skill (alias `-boosts`).
- `-plant (plant)` — A growable's Farming level, growth time, XP, and protection
  payment (aliases `-plants`, `-payment`).
- `-patch (patch type)` — Locations of a farming patch type (allotment, herb,
  tree, hops, coral, spirit, …).
- `-fairy (code | location)` — A fairy ring's inverse, by three-letter code or
  location name.
- `-npc (name)` — An NPC/monster's HP, XP, combat bonuses, weakness, and slayer
  info; partial matches on a miss.
- `-noburn (fish)` — The Cooking level to stop burning a fish, across fire/range
  and with Hosidius and gauntlet bonuses; all fish if none given (alias `-burn`).
- `-wiki (query)` — A link to the OSRS Wiki search results for a query.
- `-togw` — The world(s) currently marked as a Tears of Guthix world, live.

### Fun & Social

- `+congrats [nick] (level) (skill)` — Public celebratory message for a skill
  level or XP milestone (aliases `+gz`, `+grats`, `+gratz`, `+congratz`,
  `+congratulations`). Accepts skill names and `combat`/`cmb`/`cmbt` (range
  4–126).

## Building

```sh
cargo build --release
```

This produces `target/release/libreinze_lib_runescape.so`. Install it into the
`rust-reinze` host's `plugins/` directory. Install **atomically** — build to a
temp file on the same filesystem, then `mv`/rename it into place — so the host
never loads a partially written library.
