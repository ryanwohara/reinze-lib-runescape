# `-pickpocket` — profit rates for the six pickpocketing methods

Date: 2026-08-05
Status: Approved (design discussed and accepted in-session)

## Purpose

The wiki keeps six money making guides for pickpocketing. `-pickpocket` ranks
them by profit per hour at current prices, and details one — including what it
takes to do it — when the caller names it:

```
-pickpocket                 all six ranked by profit/hr, marked above your level
-pickpocket @vyres          vyres detail, with requirements
-pickpocket Zezima @elves   elves at Zezima's Thieving level
-pickpocket @elves ^85      as if Thieving 85 — no hiscore lookup
-pickpocket -i @paladins    ironman hiscores
```

## Command surface

New `src/pickpocket.rs`, `mod pickpocket;` in `lib.rs`, triggers `^pickpocket\d*$`
and `^pp\d*$`, dispatched as `"pickpocket" | "pp"`. Both anchored, and both added
to `commands_are_not_dispatched_twice` so the one-trigger-per-command rule holds.
`\d*` is present because the command does a hiscore lookup, so `-pickpocket2`
must reach the plugin to select a second linked RSN.

Flags come from the shared parser, exactly as `+chef` gets them:

| Input | Meaning | Source |
|---|---|---|
| `@method` | which method to detail | `flags.search` |
| `^N` | assume Thieving level (or raw XP) `N` | `flags.start` |
| `-i` `-u` `-h` … | account type | `flags.account_type` |
| leftover text | RSN | `strip_stats_parameters` |

The method is always selected with `@`, never by position, so the leftover query
text is unambiguously the RSN. This matches `+chef` and every other stats-backed
command. Matching is exact, then prefix, then substring, so `@vyre`, `@knight`
and `@ham` all resolve.

## The six methods

Rates, XP and loot come from the wiki's `{{Mmgtable}}` templates, which already
fold drop rates and rogue-equipment doubling into their expected quantities.

| Method | Thieving | Rate/hr | XP each | Quests |
|---|---|---|---|---|
| H.A.M. members | 15 | 18 easy clues | 1,110 (22.2 × 50) | Various, for clue steps |
| Master farmers | 38 | 3,000 | 43 | None |
| Knights of Ardougne | 55 | 3,000 | 84.3 | Plague City, if suiciding |
| Paladins | 70 | 1,145 | 151.8 | None |
| Vyres | 82 | 720 | 306.9 | Sins of the Father |
| Elves | 85 | 560 | 353.3 | Song of the Elves |

H.A.M. is the one method whose rate is not pickpockets: it is easy clues per
hour, with 50 pickpockets per clue. The unit is labelled in the output rather
than left to be assumed.

### Loot and costs

Per pickpocket unless marked hourly.

**H.A.M. members** — in: 2,500 coins per clue (teleports and runes), 5 stamina
potions per hour. Out: 1 easy clue valued at 3 caskets, 0.02 master clues valued
at 6 caskets.

**Master farmers** — no inputs. Out: 18 seed types, from `2 × 6.5 × 1/18` barley
down to `2 × 1/9271.98` torstol.

**Knights of Ardougne** — in: 0.1 cosmic runes, 0.005 jugs of wine. Out: 100
coins (50 base, 50 again with rogue equipment).

**Paladins** — in: 5,000 coins per hour (baker stall food), dodgy necklace
charges at `1/10 × 0.25 × (1 − 0.7109)`. Out: 160 coins, 4 chaos runes.

**Vyres** — in: dodgy necklace at `1/10 × 0.25 × (1 − 0.15) × (1 − 0.6055)`, 300
cosmic runes per hour. Out: blood shard `2 × 1/5000`, `(230 + 272.5) × 109/132`
coins, death runes, blood runes, blood pints, uncut rubies, diamonds, cooked
mystery meat.

**Elves** — in: dodgy necklace `13/450`, 300 cosmic runes per hour. Out: enhanced
crystal teleport seed `2 × 1/1024`, crystal shards `2 × 1/35`, `2 × (280+350)/2 ×
105/128` coins, death runes, nature runes, fire orbs, diamonds, gold ore.

Quantities are stored as evaluated `f64` constants with the wiki's expression in
a comment beside them. There is no reason to carry an expression evaluator into
the command when the inputs never change at runtime.

## Profit

```
profit/hr = (per-pickpocket outputs × rate + hourly outputs)
          − (per-pickpocket inputs  × rate + hourly inputs)
```

Prices come from `lib/ge.json` using the **`high` value only, exactly as `-price`
does**. This differs from the `price_of` helper `+chef` and `+degrime` share,
which falls back to `low` when nothing has bought recently; `-pickpocket` matches
`-price` because that is the figure a caller can check for themselves.

Coins are valued at 1 rather than looked up. Sold items pay the 2% Grand Exchange
tax, capped at 5,000,000 per item; coins do not, since they are not sold. This
matches both the wiki's own guides, which quote outputs after tax, and `+chef`.

Two outputs are not plain GE lookups:

- **Crystal shards** use the wiki's valuation, `(Divine super combat potion(4) −
  Super combat potion(4)) × 2.5`, which prices live from two tradeable items.
- **Clue scrolls** use generated constants (below).

An item with no `high` price contributes nothing **and marks the method**.
`-price` prints `0` for such an item, so contributing nothing matches it — but
silently understating a profit figure is worse than saying the data is thin, so
the method carries a marker and a footnote.

### Clue values

Casket value is not a GE price: it comes from the reward tables, 283 drop rows
for easy and a 170-term expression for master. Rather than port those tables for
the least profitable of six methods, the wiki evaluates them and the results are
baked in:

```rust
/// Average casket values. Generated by scripts/gen-clue-values.js — re-run it
/// to refresh. Sourced from the wiki's {{EasyClueValue}} and {{MasterClueValue}},
/// which price the full reward tables at the time of generation.
const EASY_CASKET_GP: u32 = 9_773;
const MASTER_CASKET_GP: u32 = 108_946;
```

`scripts/gen-clue-values.js` asks the wiki API to expand both templates and
rewrites the two constants in place, the same pattern `gen-plant.js` uses for the
farming table. Every other priced item stays live on every invocation.

### The model reproduces the wiki

Worked against the wiki's own rendered figures, which is what makes the loot
tables above trustworthy rather than transcribed hopefully:

| Method | Model | Wiki |
|---|---|---|
| H.A.M. members | 18 × (3 × 9,773 + 0.02 × 6 × 108,946) = 763,062 out | 763,068 out |
| Knights | 3,000 × 100 coins = 300,000 out | 300,000 out |
| Paladins | 1,145 × 160 coins = 183,200, remainder ≈ 104 gp/chaos rune | 659,520 out |
| Master farmers | 43 × 3,000 = 129,000 Thieving XP/hr | 129,000 XP/hr |

The residual differences are the wiki's own rounding. Implementation should
re-check each method against the rendered guide once prices are wired up.

For reference, the wiki's profit figures at the time of writing: elves 4,529,773,
vyres 2,514,262, master farmers 1,222,670, H.A.M. members 699,643, paladins
644,457, Knights of Ardougne 262,755.

## Output

List mode, packed to `MAX_LINE_LEN` with `pack_lines`, ranked by profit:

```
[Pickpocket] Thieving 74 | Profit/hr: Elves 4.5m* | Vyres 2.5m*
             Master farmers 1.2m | H.A.M. members 700k | Paladins 644k | Knights 263k
[Pickpocket] * above your Thieving level
```

Those are the wiki's current figures, so the ordering is real rather than
illustrative — though it moves with prices, which is the point of computing it.

Detail mode:

```
[Pickpocket] Vyres | Thieving 82 | 720/hr | 306.9 XP each (221k XP/hr)
[Pickpocket] Profit/hr 2.5m | Outputs 2.56m | Inputs 44.5k
[Pickpocket] Requires 82 Thieving, Sins of the Father | 50 Agility for rogue equipment | 47 Magic for Shadow Veil
```

Profit is coloured green and losses red, reusing the convention `+chef`
established. Methods above the caller's level are listed but marked — the market
answer is useful before the level is.

## Error handling

Each case returns a single prefixed line, matching `+chef` and `+degrime`:

| Case | Output |
|---|---|
| `@` names no known method | `'<query>' isn't a pickpocketing method - try -pickpocket for the full list` |
| no prices at all | `No prices` |
| hiscore fetch fails | `No hiscores found` |
| no Thieving row | `No Thieving level found` |
| level below the method's requirement, detail mode | the level line plus `Requires <n> Thieving` |

## Testing

Everything except the price load is pure, so the tests need no network — the same
split `+chef` and `+degrime` use.

- per-hour maths — outputs minus inputs; hourly inputs counted once, not per
  pickpocket; a method with no inputs still prices.
- tax — 2% rounded down on items, none on coins.
- `find_method` — exact beats prefix beats substring; `ham`, `knight`, `vyre`
  resolve; unknown and empty return `None`.
- level gating — a method above the caller's level is marked, not dropped.
- unpriced items — an item with no `high` contributes zero and sets the marker.
- table integrity — six methods, each with a name, a Thieving level, a positive
  rate and at least one output.

## Deliberate calls

- **Two clue constants are generated, not computed.** Porting 450 drop rows and
  the casket roll model for one method is disproportionate; the refresher script
  makes the staleness a one-command fix.
- **`-price` semantics, not `price_of`.** A caller checking a figure will use
  `-price`, so the numbers should agree.
- **Expected quantities are pre-evaluated.** The wiki's expressions live in
  comments; nothing parses them at runtime.
- **Rogue equipment is assumed**, because the wiki's quantities already include
  its doubling. The requirement line says so.
