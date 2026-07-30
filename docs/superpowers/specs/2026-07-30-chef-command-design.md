# `+chef` — cooking profit for raw fish

Date: 2026-07-30
Status: Approved (design discussed and accepted in-session)

## Purpose

`+degrime` answers "what does an hour of cleaning herbs pay right now?" from live
Grand Exchange prices. `+chef` answers the same question for Cooking, with two
differences that the skill forces:

- Cooking **burns** food below a certain level, and burnt fish give **no XP** and
  sell for nothing while still costing a raw fish. Profit and XP rates therefore
  both depend on the cook's level.
- The level is worth reporting on its own, so `+chef` also carries the
  `+herbi`-style goal block: XP to the next level, fish needed to get there, how
  long that takes, and what it pays along the way.

```
+chef                  ranked list of every cookable fish, best profit/hr first
+chef @shark           shark detail at your Cooking level
+chef Zezima @shark    shark detail at Zezima's Cooking level
+chef @shark ^70       shark detail as if level 70 — calculator, no hiscore lookup
+chef @shark #92       goal block targets level 92 instead of your next level
+chef -i @shark        ironman hiscores
```

## Command surface

New `src/chef.rs`, `mod chef;` in `lib.rs`, trigger `^chef$`, dispatched as
`"chef" => chef::lookup(source)`. The trigger is anchored because `lib.rs`'s
`commands_are_not_dispatched_twice` test requires each command to fire exactly
one trigger; `chef` is added to that test's list.

Flags come from the shared parser, so nothing new is parsed here:

| Input | Meaning | Source |
|---|---|---|
| `@fish` | which fish to detail | `flags.search` |
| `^N` | assume level (or raw XP) `N`, skip the hiscore lookup | `flags.start` |
| `#N` | goal target level | `flags.end` via `Listing::next_level` |
| `-i` `-u` `-h` … | account type | `flags.account_type` |
| leftover text | RSN | `strip_stats_parameters` |

**The fish is always selected with `@`, never by position.** The leftover query
text is unambiguously the RSN, which keeps `+chef` consistent with every other
stats-backed command and removes the "is `Shark` a fish or a player?" ambiguity
that positional matching would create. `strip_stats_parameters` already removes
`@shark` from the query, so the RSN falls out clean.

The `@` capture is `(\S+)` — a single token — so multi-word fish are selected by
any distinctive token (`@turtle`, `@crab`, `@manta`). Matching is exact, then
prefix, then substring, the same ladder `noburn.rs` already uses.

## Fish covered

Every fish with both a raw and a cooked Grand Exchange item — eleven in total.
Cooking levels and XP come from the wiki's burn-level tables (they agree with
`src/stats/cooking.rs`).

| Fish | Level | XP | Raw item | Cooked item |
|---|---|---|---|---|
| Tuna | 30 | 100 | Raw tuna | Tuna |
| Karambwan | 30 | 190 | Raw karambwan | Cooked karambwan |
| Lobster | 40 | 120 | Raw lobster | Lobster |
| Bass | 43 | 130 | Raw bass | Bass |
| Swordfish | 45 | 140 | Raw swordfish | Swordfish |
| Monkfish | 62 | 150 | Raw monkfish | Monkfish |
| Shark | 80 | 210 | Raw shark | Shark |
| Sea turtle | 82 | 211.3 | Raw sea turtle | Sea turtle |
| Anglerfish | 84 | 230 | Raw anglerfish | Anglerfish |
| Dark crab | 90 | 215 | Raw dark crab | Dark crab |
| Manta ray | 91 | 216.3 | Raw manta ray | Manta ray |

## Shared fish data: `src/fish.rs`

`noburn.rs` already owns a burn table for six of these fish, and its numbers have
drifted from the wiki. Rather than keep a second, differently-wrong copy in
`chef.rs`, the table moves to a new `src/fish.rs` that both commands read:
`noburn` renders it, `chef` does money and XP maths over it.

### The `Stop` enum

The current table stores each stop-burn level as a `u32` and uses `0` as a
sentinel for "N/A". That conflates three genuinely different situations, and the
wiki distinguishes them:

| Wiki cell | Meaning | Modelled as |
|---|---|---|
| `86` | stops burning at 86 | `Stop::Level(86)` |
| `—` | never stops burning below 99 | `Stop::Never` |
| `N/A` (Hosidius / gauntlet column) | never burns with that setup — the level would fall below the elite diary's own level 84 requirement | `Stop::NoBurn` |
| `N/A` (range column, non-gauntlet foods) | a range is no better than a fire | copy the fire value |

`Stop::Never` and `Stop::NoBurn` are opposites, and today `+noburn` prints "N/A"
for both. Replacing the sentinel with

```rust
enum Stop { Level(u32), Never, NoBurn }
```

fixes that display bug as a side effect: `Never` renders "N/A", `NoBurn` renders
"never" (exact wording chosen at implementation time).

### The table

Values are the wiki's current ones, which corrects seven stale entries in
`noburn.rs`: swordfish range 81→80, swordfish gauntlets 81→80, lobster
gauntlets+Hosidius-5% 61→60, monkfish gauntlets 87→86, shark Hosidius-10% 99→98,
anglerfish gauntlets 98→97, and anglerfish gauntlets+Hosidius-10% 88→87.

Gauntlet-affected fish:

| Fish | Fire | Range | Hos 5% | Hos 10% | Gaunt | Gaunt+5% | Gaunt+10% |
|---|---|---|---|---|---|---|---|
| Lobster | 74 | 74 | 70 | NoBurn | 64 | 60 | NoBurn |
| Swordfish | 86 | 80 | 76 | NoBurn | 80 | 76 | NoBurn |
| Monkfish | 92 | 90 | 86 | 82 | 86 | 82 | NoBurn |
| Shark | Never | Never | Never | 98 | 94 | 89 | 84 |
| Anglerfish | Never | Never | Never | Never | 97 | 93 | 87 |

Fish gauntlets do not affect (the gauntlet columns are absent, not zero):

| Fish | Fire | Range | Hos 5% | Hos 10% |
|---|---|---|---|---|
| Tuna | 63 | 63 | 59 | NoBurn |
| Karambwan | 99 | 99 | 93 | 87 |
| Bass | 79 | 79 | 75 | NoBurn |
| Sea turtle | Never | Never | Never | Never |
| Dark crab | Never | Never | Never | Never |
| Manta ray | Never | Never | Never | Never |

## Burn model

`+chef` reports four setups: **Fire**, **Range**, **Gauntlets** (gauntlets on a
normal range), and **Hosidius** (gauntlets plus 10% Hosidius favour, or plain
Hosidius 10% for fish gauntlets don't affect).

For the six fish gauntlets do not affect, the Gauntlets figure equals the Range
figure. Those fish are labelled rather than silently repeated, so the output
never implies gauntlets are doing something they aren't.

The game's real burn curve is not published — the wiki gives only the level at
which burning stops. Burn is therefore modelled as a straight line from an anchor
at the fish's cooking level down to zero at that setup's stop level:

```
burn(level, stop) =
    0.0                                            if stop is NoBurn
    0.0                                            if level >= stop
    MAX_BURN * (stop  - level) / (stop  - cook)    if stop is Level(stop)
    MAX_BURN * (100.0 - level) / (100.0 - cook)    if stop is Never
```

`MAX_BURN = 0.50`. This is the design's one invented number — an anchor chosen
because it lands near reported rates at the bottom of each fish's window, not a
figure the wiki publishes. It is a single named constant so it is cheap to
retune. `Stop::Never` interpolates towards a notional level 100 so those setups
never quite reach 0%, matching the wiki's "still burning at 99".

**Every burn figure and every number derived from one is printed with `~`.**
The command must not imply precision it does not have.

## Money model

Prices come from `get_item_db()` + `get_ge_data()`, read through degrime's
`price_of` helper (buy offer, falling back to sell offer).

Selling on the Grand Exchange costs a 2% convenience fee, rounded down per item
and capped at 5,000,000 gp (2% since 29 May 2025; it was 1% before). Nothing in
the repo models this today, and at ~20 gp on a cooked shark it is roughly 8% of
the profit, so `+chef` accounts for it:

```
tax(price)   = min(floor(price * 0.02), 5_000_000)
per_raw      = (1 - burn) * (cooked - tax(cooked)) - raw
profit_hourly = FISH_PER_HOUR * per_raw
```

Buying is untaxed — the fee falls on the seller — so the raw fish costs its
market price. Burnt fish are the difference between the two terms: they consume a
raw fish and return nothing.

`FISH_PER_HOUR = 1_300`, the wiki money-making guide's default cook rate. Profit
is signed: cooking can and often does lose money.

## XP and the goal block

Burning yields no XP, so expected XP per raw fish is `xp * (1 - burn)`, and the
number of fish a goal needs falls as the level rises and burning slows.

`fish_between(xp, target_xp, fish, setup)` walks **level bands** rather than
individual fish: burn is constant within a level, so each band needs
`ceil(remaining_in_band / xp_per_fish)` fish. This is the same idea as herbi's
`catches_between`, without its per-catch loop — a 200m-XP target would otherwise
iterate millions of times.

It returns `None` when the level is below the fish's cooking level.

The goal block is computed at the **Hosidius** setup (the best of the four) and
says so, because a fish count has to assume some burn rate.

## Output

Detail mode, three lines. The figures below are worked through the model at the
wiki's quoted shark prices (raw 732, cooked 991, tax 19), so they show what the
command actually prints rather than a sketch:

```
[Chef] Shark | Cooking 85 | 210 XP each | Raw 732 / Cooked 991 (-19 tax)
[Chef] Fire ~38% -162k/hr | Range ~38% -162k/hr | Gauntlets ~32% -94k/hr | Hosidius 0% 312k/hr
[Chef] 3.3m XP to 92 | 15,518 sharks | ~11.9h | 3.7m profit (Hosidius, 1,300/hr)
```

That spread is the point of the command: at level 85 a shark is worth 240 gp
cooked and taxed, so burning even a third of them turns a 312k/hr profit into a
162k/hr loss.

List mode, packed to `MAX_LINE_LEN` with the existing `pack_lines`, ranked by
profit/hr at the Hosidius setup (fish and figures illustrative — the real order
is whatever the live market says):

```
[Chef] Cooking 85 | Profit/hr: Anglerfish 1.6m | Karambwan 890k | Shark 312k |
       Dark crab 210k* | Manta ray 180k* | Monkfish 61k | Swordfish 40k | Tuna -11k
[Chef] * above your Cooking level
```

Fish above the caller's level are still listed — the market answer is useful
before you have the level — but marked, and the footnote line only appears when
at least one fish is marked. Colouring follows the house pattern: `source.l` for
the prefix, `c1` for labels, `c2` for values, `p` for parenthetical detail.

## Error handling

Each case returns a single prefixed line, matching degrime and herbi:

| Case | Output |
|---|---|
| `@` names no known fish | `'<query>' isn't a cookable fish - try +chef for the full list` |
| no price for the chosen fish | `No price for <fish>` |
| no prices at all | `No fish prices` |
| hiscore fetch fails | `No hiscores found` |
| no Cooking row | `No Cooking level found` |
| level below the fish's cooking level, detail mode | the level line plus `Requires <n> Cooking`, as herbi does for the hunter requirement |

## Testing

Everything except the two HTTP fetches is a pure function, so the tests need no
network — the same split degrime uses.

- `burn` — zero at and above the stop level; zero for `NoBurn`; `MAX_BURN` at the
  cooking level; interpolates towards 100 for `Never`.
- `tax` — 2% rounded down; the 5m cap; zero on a price under 50.
- `hourly` — burnt fish cost a raw fish and return nothing; a loss stays negative.
- `fish_between` — rounds up within a band; uses the cheaper rate after a level
  up; `None` below the cooking level; zero when the target is already met.
- `find_fish` — exact beats prefix beats substring; `turtle` finds Sea turtle;
  unknown and empty queries return `None`.
- table integrity — every fish's raw and cooked names are distinct and non-empty,
  levels and XP are positive, no duplicates (mirrors degrime's
  `every_herb_is_named_and_distinct`).

## Deliberate calls

- **`MAX_BURN = 0.50` is invented.** Documented in place, printed with `~`.
- **`noburn` output changes**: four corrected levels, five new fish, and "N/A"
  splitting into two distinct renderings. Accepted — the wiki is upstream truth.
- **Level and XP are duplicated** between `fish.rs` and `stats/cooking.rs`.
  Reaching into the stats enum would mean making it public and mapping variants
  for eleven fish; a self-contained table matches how `degrime.rs` carries its
  own herb data. The table-integrity test guards it.
- **Buy limits are ignored.** Raw fish have four-hour GE limits that a full hour
  at 1,300/hr can exceed. `+degrime` ignores limits too; adding them is a
  separate change if it turns out to matter.
