# `-pickpocket` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `-pickpocket` command that ranks the wiki's six pickpocketing money makers by profit per hour at current Grand Exchange prices, and details one — with its requirements — when the caller names it.

**Architecture:** A new `src/pickpocket.rs` holds the six methods as a static table: rate per hour, Thieving XP, and the expected loot per pickpocket that the wiki's money making guides publish. Profit is priced from `lib/ge.json` on every invocation. Two values that are not GE prices — the easy and master casket averages — are constants written by a refresher script.

**Tech Stack:** Rust 2024 edition, `anyhow` for errors, the workspace-local `common` crate for IRC colouring, no new dependencies. One Node script for refreshing constants, run by hand.

**Design spec:** `docs/superpowers/specs/2026-08-05-pickpocket-command-design.md`

## Global Constraints

- **No new dependencies.** Everything needed is already in `Cargo.toml`.
- **`cargo test` must pass after every task**, and `cargo build --release` must be warning-free for `pickpocket.rs`.
- **Prices use the `high` value only**, exactly as `src/prices.rs` does — *not* the `price_of` helper in `common.rs`, which falls back to `low`.
- **Coins are worth 1 gp** and are never looked up or taxed.
- **GE tax is 2%**, rounded down per item, capped at 5,000,000. It applies to items sold on the GE. It does **not** apply to coins, nor to clue caskets, whose source value already accounts for it.
- **Expected quantities are pre-evaluated `f64` literals** with the wiki's original expression in a comment. Nothing parses expressions at runtime.
- **Rogue equipment is assumed** — the wiki's quantities already include its doubling, and the requirement lines say so.
- Colour discipline: `source.l` for the prefix, `c1` for labels, `c2` for values, `p` for parentheticals. Signed gp uses the `GREEN`/`RED` convention from `src/chef.rs`.
- **Commit after every task** using `type(scope): subject`.

## File Structure

| File | Responsibility |
|---|---|
| `src/pickpocket.rs` (create) | The six methods, their loot tables, the profit maths, and both output modes. |
| `scripts/gen-clue-values.js` (create) | Refreshes the two casket constants from the wiki. Run by hand. |
| `src/lib.rs` (modify) | `mod pickpocket;`, two triggers, dispatch arm, help entry, trigger test. |
| `README.md` (modify) | `-pickpocket` entry under Economy. |

---

### Task 1: The method table

**Files:**
- Create: `src/pickpocket.rs`
- Modify: `src/lib.rs` (add `mod pickpocket;`, alphabetically after `mod plant;`)

**Interfaces:**
- Consumes: nothing.
- Produces: `pub enum Value { Ge, Coins, EasyCaskets(f64), MasterCaskets(f64), CrystalShard }`; `pub struct Loot { pub item: &'static str, pub qty: f64, pub value: Value, pub per_hour: bool }`; `pub struct Method { pub name, pub rate_name: &'static str, pub thieving: u32, pub rate: f64, pub xp: f64, pub inputs: &'static [Loot], pub outputs: &'static [Loot], pub requirements: &'static [&'static str] }`; `pub const METHODS: [Method; 6]`; `pub fn find_method(query: &str) -> Option<&'static Method>`; `pub const EASY_CASKET_GP: f64`; `pub const MASTER_CASKET_GP: f64`.

**Background.** Every quantity below is the *expected* amount per pickpocket, already folding in drop rates and the rogue-equipment doubling — they come from the wiki's `{{Mmgtable}}` templates. A few entries are per hour instead of per pickpocket (stamina potions, Shadow Veil's cosmic runes); those carry `per_hour: true`. H.A.M. members is the one method whose rate is not pickpockets: it is 18 easy clues an hour, at 50 pickpockets per clue.

- [ ] **Step 1: Write the failing test**

Create `src/pickpocket.rs` containing only this test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn there_are_six_methods_each_usable() {
        assert_eq!(METHODS.len(), 6);

        for method in METHODS.iter() {
            assert!(!method.name.is_empty(), "a method has no name");
            assert!(method.thieving >= 1, "{} has no Thieving level", method.name);
            assert!(method.rate > 0.0, "{} has no rate", method.name);
            assert!(method.xp > 0.0, "{} has no xp", method.name);
            assert!(
                !method.outputs.is_empty(),
                "{} has nothing to show for it",
                method.name
            );
            assert!(
                !method.requirements.is_empty(),
                "{} lists no requirements",
                method.name
            );
        }
    }

    #[test]
    fn the_table_is_ordered_by_thieving_level() {
        for pair in METHODS.windows(2) {
            assert!(
                pair[0].thieving <= pair[1].thieving,
                "{} (level {}) is listed after {} (level {})",
                pair[1].name,
                pair[1].thieving,
                pair[0].name,
                pair[0].thieving
            );
        }
    }

    #[test]
    fn methods_are_found_by_exact_prefix_or_substring() {
        assert_eq!(find_method("Paladins").map(|m| m.name), Some("Paladins"));
        assert_eq!(find_method("paladins").map(|m| m.name), Some("Paladins"));
        // Prefix.
        assert_eq!(find_method("vyre").map(|m| m.name), Some("Vyres"));
        assert_eq!(find_method("elv").map(|m| m.name), Some("Elves"));
        // Substring, which is how the multi-word methods are reachable: the
        // `@` flag captures a single token.
        assert_eq!(
            find_method("knight").map(|m| m.name),
            Some("Knights of Ardougne")
        );
        assert_eq!(
            find_method("farmer").map(|m| m.name),
            Some("Master farmers")
        );
        assert_eq!(find_method("ham").map(|m| m.name), Some("H.A.M. members"));
    }

    #[test]
    fn unknown_and_empty_queries_are_not_found() {
        assert!(find_method("guards").is_none());
        assert!(find_method("").is_none());
        assert!(find_method("   ").is_none());
    }

    #[test]
    fn the_thieving_levels_match_the_wiki() {
        assert_eq!(find_method("ham").unwrap().thieving, 15);
        assert_eq!(find_method("farmer").unwrap().thieving, 38);
        assert_eq!(find_method("knight").unwrap().thieving, 55);
        assert_eq!(find_method("paladin").unwrap().thieving, 70);
        assert_eq!(find_method("vyre").unwrap().thieving, 82);
        assert_eq!(find_method("elv").unwrap().thieving, 85);
    }

    #[test]
    fn hourly_loot_is_marked_separately_from_per_pickpocket_loot() {
        // Shadow Veil's cosmic runes are 300 an hour, not 300 per vyre.
        let vyres = find_method("vyre").expect("vyres are in the table");
        let cosmics = vyres
            .inputs
            .iter()
            .find(|loot| loot.item == "Cosmic rune")
            .expect("vyres burn cosmic runes");

        assert!(cosmics.per_hour, "cosmic runes are an hourly cost");
        assert_eq!(cosmics.qty, 300.0);

        // Dodgy necklaces are consumed per pickpocket.
        let necklaces = vyres
            .inputs
            .iter()
            .find(|loot| loot.item == "Dodgy necklace")
            .expect("vyres burn dodgy necklaces");
        assert!(!necklaces.per_hour);
    }

    #[test]
    fn coins_and_caskets_are_valued_without_a_ge_lookup() {
        let knights = find_method("knight").expect("knights are in the table");
        assert!(matches!(knights.outputs[0].value, Value::Coins));

        let ham = find_method("ham").expect("H.A.M. is in the table");
        assert!(
            ham.outputs
                .iter()
                .any(|loot| matches!(loot.value, Value::EasyCaskets(_))),
            "H.A.M. pays in easy caskets"
        );
    }

    #[test]
    fn master_farmers_carry_the_full_seed_table() {
        let farmers = find_method("farmer").expect("master farmers are in the table");

        assert_eq!(farmers.outputs.len(), 18, "18 seed types drop");
        assert!(farmers.inputs.is_empty(), "master farmers cost nothing");
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Add `mod pickpocket;` to `src/lib.rs` so the file compiles, then run: `cargo test --lib pickpocket::`
Expected: FAIL — `METHODS`, `Value`, `find_method` do not exist.

- [ ] **Step 3: Write the implementation**

Put this above the test module in `src/pickpocket.rs`:

```rust
/// Average value of a reward casket, in gp.
///
/// Not a Grand Exchange price: a casket's worth comes from its reward table -
/// 283 drop rows for easy, a 170-term expression for master - so the wiki
/// evaluates those and the results are baked in here. Generated by
/// `scripts/gen-clue-values.js`; re-run it to refresh.
pub const EASY_CASKET_GP: f64 = 9_773.0;
pub const MASTER_CASKET_GP: f64 = 108_946.0;

/// How one line of loot is priced.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    /// Looked up in the item database by name.
    Ge,
    /// Coins are worth one gp each, and selling them is not a thing.
    Coins,
    /// This many easy reward caskets.
    EasyCaskets(f64),
    /// This many master reward caskets.
    MasterCaskets(f64),
    /// Crystal shards do not trade. The wiki values one as the margin a
    /// divine super combat potion carries over a plain one, over 2.5 shards.
    CrystalShard,
}

/// One line of a method's inputs or outputs.
pub struct Loot {
    pub item: &'static str,
    /// Expected amount per pickpocket, unless `per_hour` is set. Already
    /// includes drop rates and the rogue-equipment doubling.
    pub qty: f64,
    pub value: Value,
    /// Counted once an hour rather than once per pickpocket.
    pub per_hour: bool,
}

impl Loot {
    const fn ge(item: &'static str, qty: f64) -> Self {
        Self { item, qty, value: Value::Ge, per_hour: false }
    }

    const fn ge_hourly(item: &'static str, qty: f64) -> Self {
        Self { item, qty, value: Value::Ge, per_hour: true }
    }

    const fn coins(qty: f64) -> Self {
        Self { item: "Coins", qty, value: Value::Coins, per_hour: false }
    }

    const fn coins_hourly(qty: f64) -> Self {
        Self { item: "Coins", qty, value: Value::Coins, per_hour: true }
    }
}

/// One pickpocketing money maker, as the wiki's guides describe it.
pub struct Method {
    pub name: &'static str,
    pub thieving: u32,
    /// Units an hour. Usually pickpockets - see `rate_name`.
    pub rate: f64,
    /// What `rate` counts, for the output line.
    pub rate_name: &'static str,
    /// Thieving XP per unit.
    pub xp: f64,
    pub inputs: &'static [Loot],
    pub outputs: &'static [Loot],
    /// Shown when the method is named, most important first.
    pub requirements: &'static [&'static str],
}

const HAM_INPUTS: [Loot; 2] = [
    // Teleport charges and runes, per clue.
    Loot::coins(2_500.0),
    Loot::ge_hourly("Stamina potion(4)", 5.0),
];
const HAM_OUTPUTS: [Loot; 2] = [
    Loot { item: "Clue scroll (easy)", qty: 1.0, value: Value::EasyCaskets(3.0), per_hour: false },
    Loot { item: "Clue scroll (master)", qty: 0.02, value: Value::MasterCaskets(6.0), per_hour: false },
];
const HAM_REQS: [&str; 4] = [
    "15 Thieving (92 recommended)",
    "Various quests, for the clue steps",
    "Hard or elite Ardougne diary",
    "H.A.M. robes below 93 Thieving",
];

const FARMER_OUTPUTS: [Loot; 18] = [
    Loot::ge("Barley seed", 0.72222222),        // 2 * 6.5 * 1/18
    Loot::ge("Hammerstone seed", 0.55555556),   // 2 * 5 * 1/18
    Loot::ge("Jute seed", 0.41493776),          // 2 * 5 * 1/24.1
    Loot::ge("Asgarnian seed", 0.29288703),     // 2 * 3.5 * 1/23.9
    Loot::ge("Yanillian seed", 0.19390582),     // 2 * 3.5 * 1/36.1
    Loot::ge("Krandorian seed", 0.09695291),    // 2 * 3.5 * 1/72.2
    Loot::ge("Wildblood seed", 0.02816901),     // 2 * 2 * 1/142
    Loot::ge("Watermelon seed", 0.01058201),    // 2 * 1/189
    Loot::ge("Snape grass seed", 0.00769231),   // 2 * 1/260
    Loot::ge("Ranarr seed", 0.00754973),        // 2 * 1/264.91
    Loot::ge("Toadflax seed", 0.00451467),      // 2 * 1/443
    Loot::ge("Avantoe seed", 0.00211193),       // 2 * 1/947
    Loot::ge("Kwuarm seed", 0.00143988),        // 2 * 1/1389
    Loot::ge("Snapdragon seed", 0.00107852),    // 2 * 1/1854.4
    Loot::ge("Cadantine seed", 0.00067204),     // 2 * 1/2976
    Loot::ge("Lantadyme seed", 0.00047996),     // 2 * 1/4167
    Loot::ge("Dwarf weed seed", 0.00028802),    // 2 * 1/6944
    Loot::ge("Torstol seed", 0.0002157),        // 2 * 1/9271.98
];
const FARMER_REQS: [&str; 4] = [
    "38 Thieving (94+ recommended)",
    "50 Thieving and Agility for rogue equipment",
    "Hard Ardougne diary, or a thieving cape",
    "85 Farming raises the rare seed rates",
];

const KNIGHT_INPUTS: [Loot; 2] = [
    Loot::ge("Cosmic rune", 0.1),
    Loot::ge("Jug of wine", 0.005),
];
// 50 coins, and 50 again with rogue equipment.
const KNIGHT_OUTPUTS: [Loot; 1] = [Loot::coins(100.0)];
const KNIGHT_REQS: [&str; 4] = [
    "55 Thieving (95+ recommended)",
    "Plague City, if suiciding",
    "Medium Ardougne diary",
    "47 Magic and A Kingdom Divided for Shadow Veil",
];

const PALADIN_INPUTS: [Loot; 2] = [
    // Food at the baker stall.
    Loot::coins_hourly(5_000.0),
    Loot::ge("Dodgy necklace", 0.0072275), // 1/10 * .25 * (1 - .7109)
];
const PALADIN_OUTPUTS: [Loot; 2] = [
    Loot::coins(160.0), // 80 * 2
    Loot::ge("Chaos rune", 4.0),
];
const PALADIN_REQS: [&str; 4] = [
    "70 Thieving (99 recommended)",
    "50 Agility for rogue equipment",
    "Medium Ardougne diary",
    "Thieving cape and dodgy necklaces recommended",
];

const VYRE_INPUTS: [Loot; 2] = [
    Loot::ge("Dodgy necklace", 0.00838312), // 1/10 * .25 * (1-.15) * (1-.6055)
    Loot::ge_hourly("Cosmic rune", 300.0),
];
const VYRE_OUTPUTS: [Loot; 8] = [
    Loot::ge("Blood shard", 0.0004),              // 2 * 1/5000
    Loot::coins(414.94318182),                    // (230 + 272.5) * 109/132
    Loot::ge("Death rune", 0.24242424),           // 2 * 2 * 8/132
    Loot::ge("Blood rune", 0.12121212),           // 2 * 4 * 2/132
    Loot::ge("Blood pint", 0.09090909),           // 2 * 1 * 6/132
    Loot::ge("Uncut ruby", 0.07575758),           // 2 * 1 * 5/132
    Loot::ge("Diamond", 0.01515152),              // 2 * 1 * 1/132
    Loot::ge("Cooked mystery meat", 0.01515152),  // 2 * 1 * 1/132
];
const VYRE_REQS: [&str; 4] = [
    "82 Thieving (99 recommended)",
    "Sins of the Father",
    "50 Agility for rogue equipment",
    "47 Magic and A Kingdom Divided for Shadow Veil",
];

const ELF_INPUTS: [Loot; 2] = [
    Loot::ge("Dodgy necklace", 0.02888889), // 13/450
    Loot::ge_hourly("Cosmic rune", 300.0),
];
const ELF_OUTPUTS: [Loot; 8] = [
    Loot::ge("Enhanced crystal teleport seed", 0.00195313), // 2 * 1/1024
    Loot { item: "Crystal shard", qty: 0.05714286, value: Value::CrystalShard, per_hour: false }, // 2 * 1/35
    Loot::coins(516.796875),                // 2 * (280+350)/2 * 105/128
    Loot::ge("Death rune", 0.25),           // 2 * 2 * 8/128
    Loot::ge("Nature rune", 0.234375),      // 2 * 3 * 5/128
    Loot::ge("Fire orb", 0.03125),          // 2 * 1 * 2/128
    Loot::ge("Diamond", 0.015625),          // 2 * 1 * 1/128
    Loot::ge("Gold ore", 0.015625),         // 2 * 1 * 1/128
];
const ELF_REQS: [&str; 4] = [
    "85 Thieving (99 recommended)",
    "Song of the Elves",
    "50 Agility for rogue equipment",
    "47 Magic and A Kingdom Divided for Shadow Veil",
];

/// Every pickpocketing money maker the wiki keeps a guide for, in Thieving
/// level order. Rates, XP and loot are the guides'.
pub const METHODS: [Method; 6] = [
    Method {
        name: "H.A.M. members",
        thieving: 15,
        rate: 18.0,
        rate_name: "easy clues",
        xp: 1_110.0, // 22.2 per pickpocket, 50 pickpockets a clue
        inputs: &HAM_INPUTS,
        outputs: &HAM_OUTPUTS,
        requirements: &HAM_REQS,
    },
    Method {
        name: "Master farmers",
        thieving: 38,
        rate: 3_000.0,
        rate_name: "pickpockets",
        xp: 43.0,
        inputs: &[],
        outputs: &FARMER_OUTPUTS,
        requirements: &FARMER_REQS,
    },
    Method {
        name: "Knights of Ardougne",
        thieving: 55,
        rate: 3_000.0,
        rate_name: "pickpockets",
        xp: 84.3,
        inputs: &KNIGHT_INPUTS,
        outputs: &KNIGHT_OUTPUTS,
        requirements: &KNIGHT_REQS,
    },
    Method {
        name: "Paladins",
        thieving: 70,
        rate: 1_145.0,
        rate_name: "pickpockets",
        xp: 151.8,
        inputs: &PALADIN_INPUTS,
        outputs: &PALADIN_OUTPUTS,
        requirements: &PALADIN_REQS,
    },
    Method {
        name: "Vyres",
        thieving: 82,
        rate: 720.0,
        rate_name: "pickpockets",
        xp: 306.9,
        inputs: &VYRE_INPUTS,
        outputs: &VYRE_OUTPUTS,
        requirements: &VYRE_REQS,
    },
    Method {
        name: "Elves",
        thieving: 85,
        rate: 560.0,
        rate_name: "pickpockets",
        xp: 353.3,
        inputs: &ELF_INPUTS,
        outputs: &ELF_OUTPUTS,
        requirements: &ELF_REQS,
    },
];

/// Match a query against the method names, ignoring case and surrounding
/// space. Exact wins over a prefix, which wins over a substring - the
/// multi-word methods are only reachable by substring, because the `@` flag
/// captures a single token.
pub fn find_method(query: &str) -> Option<&'static Method> {
    let query = query.trim().to_lowercase();

    if query.is_empty() {
        return None;
    }

    // "H.A.M. members" should answer to "ham", so punctuation is ignored on
    // both sides of the comparison.
    let plain = |s: &str| s.to_lowercase().replace(['.', '-', '\''], "");
    let needle = plain(&query);

    METHODS
        .iter()
        .find(|method| plain(method.name) == needle)
        .or_else(|| METHODS.iter().find(|method| plain(method.name).starts_with(&needle)))
        .or_else(|| METHODS.iter().find(|method| plain(method.name).contains(&needle)))
}
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --lib pickpocket::`
Expected: PASS, 8 tests.

Run: `cargo test`
Expected: the suite's previous count plus 8, 0 failures. Record the number you see.

- [ ] **Step 5: Commit**

```bash
git add src/pickpocket.rs src/lib.rs
git commit -m "$(cat <<'EOF'
feat(pickpocket): the six pickpocketing money makers

Rates, Thieving XP and expected loot per pickpocket, taken from the
wiki's money making guides. The quantities already fold in drop rates and
the rogue-equipment doubling, so the wiki's expression is kept in a
comment beside each evaluated figure.
EOF
)"
```

---

### Task 2: Pricing and profit

**Files:**
- Modify: `src/pickpocket.rs`

**Interfaces:**
- Consumes: `Value`, `Loot`, `Method`, `METHODS`, `find_method`, `EASY_CASKET_GP`, `MASTER_CASKET_GP` from Task 1; `crate::common::{get_ge_data, get_item_db}`; `crate::items::{Mapping, Price}`.
- Produces: `enum Side { Input, Output }`; `fn ge_high(items: &[Mapping], ge: &HashMap<u32, Price>, name: &str) -> Option<u32>`; `fn tax(price: f64) -> f64`; `fn value_of(loot: &Loot, side: Side, items: &[Mapping], ge: &HashMap<u32, Price>) -> Option<f64>`; `pub struct Hourly { pub outputs: i64, pub inputs: i64, pub profit: i64, pub unpriced: bool }`; `fn hourly(method: &Method, items: &[Mapping], ge: &HashMap<u32, Price>) -> Hourly`.

**Background.** `src/prices.rs` — which backs `-price` — reads `lib/ge.json` and uses each item's `high` value, with no fallback. The `price_of` helper in `common.rs` is different: it falls back to `low`. This command matches `-price`, because that is the figure a caller can check for themselves.

**The tax only applies to one side.** You pay the full asking price for an input and hand over 2% of what you get for an output, so `value_of` takes a `Side`. Taxing inputs as well would quietly overstate every method's profit.

- [ ] **Step 1: Write the failing test**

Add to `src/pickpocket.rs`'s existing test module:

```rust
    use crate::items::{Mapping, Price};
    use std::collections::HashMap;

    /// A two-item database: one that has traded, one that has not.
    fn stub_market() -> (Vec<Mapping>, HashMap<u32, Price>) {
        let items = vec![
            Mapping { id: 1, name: "Chaos rune".to_string(), members: true,
                      lowalch: None, highalch: None, limit: None, value: None, total: None },
            Mapping { id: 2, name: "Blood shard".to_string(), members: true,
                      lowalch: None, highalch: None, limit: None, value: None, total: None },
        ];
        let mut ge = HashMap::new();
        ge.insert(1, Price { high: Some(100), low: Some(90) });
        // No buy offer: -price prints 0 for this, so we do too.
        ge.insert(2, Price { high: None, low: Some(5_000_000) });

        (items, ge)
    }

    #[test]
    fn a_price_is_the_high_value_matching_the_price_command() {
        let (items, ge) = stub_market();

        assert_eq!(ge_high(&items, &ge, "Chaos rune"), Some(100));
        assert_eq!(ge_high(&items, &ge, "chaos RUNE"), Some(100));
    }

    #[test]
    fn an_item_with_no_buy_offer_does_not_fall_back_to_the_sell_offer() {
        // `price_of` in common.rs would answer 5,000,000 here. -price says 0,
        // and this command matches -price.
        let (items, ge) = stub_market();

        assert_eq!(ge_high(&items, &ge, "Blood shard"), None);
    }

    #[test]
    fn an_unknown_item_has_no_price() {
        let (items, ge) = stub_market();

        assert_eq!(ge_high(&items, &ge, "Santa hat"), None);
    }

    #[test]
    fn tax_is_two_percent_rounded_down_and_capped() {
        assert_eq!(tax(100.0), 2.0);
        assert_eq!(tax(99.0), 1.0);
        assert_eq!(tax(49.0), 0.0);
        assert_eq!(tax(1_000_000_000.0), 5_000_000.0);
    }

    #[test]
    fn coins_are_worth_one_each_and_are_never_taxed() {
        let (items, ge) = stub_market();
        let coins = Loot::coins(100.0);

        assert_eq!(value_of(&coins, Side::Output, &items, &ge), Some(1.0));
    }

    #[test]
    fn caskets_are_valued_from_the_generated_constants() {
        let (items, ge) = stub_market();
        let easy = Loot { item: "Clue scroll (easy)", qty: 1.0,
                          value: Value::EasyCaskets(3.0), per_hour: false };

        assert_eq!(
            value_of(&easy, Side::Output, &items, &ge),
            Some(3.0 * EASY_CASKET_GP)
        );
    }

    #[test]
    fn an_output_pays_the_tax_and_an_input_does_not() {
        // You hand over 2% of what you sell, and pay the full ask for what you
        // buy. Taxing both sides would overstate every method's profit.
        let (items, ge) = stub_market();
        let rune = Loot::ge("Chaos rune", 1.0);

        assert_eq!(value_of(&rune, Side::Output, &items, &ge), Some(98.0));
        assert_eq!(value_of(&rune, Side::Input, &items, &ge), Some(100.0));
    }

    #[test]
    fn an_hour_is_outputs_less_inputs() {
        let (items, ge) = stub_market();
        // Knights: 3,000 pickpockets x 100 coins, less cosmic runes and wine
        // that this stub market cannot price.
        let knights = find_method("knight").expect("knights are in the table");
        let hour = hourly(knights, &items, &ge);

        assert_eq!(hour.outputs, 300_000);
        assert_eq!(hour.inputs, 0, "neither input is priced by the stub");
        assert_eq!(hour.profit, hour.outputs - hour.inputs);
        assert!(hour.unpriced, "unpriced inputs must be flagged");
    }

    #[test]
    fn hourly_loot_is_counted_once_not_once_per_pickpocket() {
        let (mut items, mut ge) = stub_market();
        items.push(Mapping { id: 3, name: "Dodgy necklace".to_string(), members: true,
                             lowalch: None, highalch: None, limit: None, value: None, total: None });
        ge.insert(3, Price { high: Some(1_000), low: Some(1_000) });

        let vyres = find_method("vyre").expect("vyres are in the table");
        let hour = hourly(vyres, &items, &ge);

        // 720 pickpockets x 0.00838312 necklaces x 1,000 gp = 6,036, untaxed
        // because it is a cost. If the 300 hourly cosmic runes were counted
        // per pickpocket instead, the total would be orders of magnitude
        // larger.
        assert_eq!(hour.inputs, 6_036);
    }

    #[test]
    fn a_method_whose_items_all_price_is_not_flagged() {
        let (mut items, mut ge) = stub_market();
        items.push(Mapping { id: 4, name: "Jug of wine".to_string(), members: false,
                             lowalch: None, highalch: None, limit: None, value: None, total: None });
        ge.insert(4, Price { high: Some(50), low: Some(50) });
        items.push(Mapping { id: 5, name: "Cosmic rune".to_string(), members: true,
                             lowalch: None, highalch: None, limit: None, value: None, total: None });
        ge.insert(5, Price { high: Some(200), low: Some(200) });

        let knights = find_method("knight").expect("knights are in the table");
        let hour = hourly(knights, &items, &ge);

        assert!(!hour.unpriced, "every knight item prices here");
        // 3,000 x (0.1 x 200 + 0.005 x 50) = 60,750.
        assert_eq!(hour.inputs, 60_750);
    }
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --lib pickpocket::`
Expected: FAIL — `ge_high`, `tax`, `value_of`, `hourly` do not exist.

- [ ] **Step 3: Write the implementation**

Add to `src/pickpocket.rs`, above the test module. Extend the file's imports to:

```rust
use crate::items::{Mapping, Price};
use std::collections::HashMap;
```

```rust
/// The Grand Exchange takes 2% of a sale, rounded down, capped per item.
const GE_TAX_PERCENT: f64 = 0.02;
const GE_TAX_CAP: f64 = 5_000_000.0;

/// Fish an item's buy price out of the item database and price list.
///
/// This is the `high` value with no fallback, which is what `-price` reports.
/// `common::price_of` deliberately differs - it falls back to `low` - so a
/// figure here can be checked with `-price` and agree.
fn ge_high(items: &[Mapping], ge: &HashMap<u32, Price>, name: &str) -> Option<u32> {
    let id = items
        .iter()
        .find(|item| item.name.eq_ignore_ascii_case(name))?
        .id;

    ge.get(&id)?.high
}

fn tax(price: f64) -> f64 {
    (price * GE_TAX_PERCENT).floor().min(GE_TAX_CAP)
}

/// Which side of the ledger a line sits on. The Grand Exchange takes its cut
/// when you sell, so an output nets less than its price while an input costs
/// exactly its price.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Side {
    Input,
    Output,
}

/// What one of an item is worth, net of tax where tax applies. `None` when the
/// item cannot be priced at all.
fn value_of(
    loot: &Loot,
    side: Side,
    items: &[Mapping],
    ge: &HashMap<u32, Price>,
) -> Option<f64> {
    let net = |gross: f64| match side {
        Side::Output => gross - tax(gross),
        Side::Input => gross,
    };

    match loot.value {
        // Coins are not sold, so no lookup and no tax.
        Value::Coins => Some(1.0),
        // The casket averages already account for tax.
        Value::EasyCaskets(n) => Some(n * EASY_CASKET_GP),
        Value::MasterCaskets(n) => Some(n * MASTER_CASKET_GP),
        Value::CrystalShard => {
            let divine = ge_high(items, ge, "Divine super combat potion(4)")? as f64;
            let plain = ge_high(items, ge, "Super combat potion(4)")? as f64;

            Some(net((divine - plain) * 2.5))
        }
        Value::Ge => Some(net(ge_high(items, ge, loot.item)? as f64)),
    }
}

/// An hour of a method, in gp.
#[derive(Debug, PartialEq)]
pub struct Hourly {
    pub outputs: i64,
    pub inputs: i64,
    pub profit: i64,
    /// At least one item had no buy price, so the totals understate reality.
    pub unpriced: bool,
}

/// Total one side of the ledger. Per-pickpocket loot scales with the rate;
/// hourly loot is counted once.
fn total(
    loot: &[Loot],
    side: Side,
    rate: f64,
    items: &[Mapping],
    ge: &HashMap<u32, Price>,
) -> (f64, bool) {
    let mut gp = 0.0;
    let mut unpriced = false;

    for line in loot {
        match value_of(line, side, items, ge) {
            Some(value) => {
                let amount = if line.per_hour { line.qty } else { line.qty * rate };
                gp += amount * value;
            }
            None => unpriced = true,
        }
    }

    (gp, unpriced)
}

fn hourly(method: &Method, items: &[Mapping], ge: &HashMap<u32, Price>) -> Hourly {
    let (outputs, out_missing) = total(method.outputs, Side::Output, method.rate, items, ge);
    let (inputs, in_missing) = total(method.inputs, Side::Input, method.rate, items, ge);

    Hourly {
        outputs: outputs.round() as i64,
        inputs: inputs.round() as i64,
        profit: (outputs - inputs).round() as i64,
        unpriced: out_missing || in_missing,
    }
}
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --lib pickpocket::`
Expected: PASS, 18 tests.

Run: `cargo test`
Expected: Task 1's total plus 10, 0 failures.

- [ ] **Step 5: Commit**

```bash
git add src/pickpocket.rs
git commit -m "$(cat <<'EOF'
feat(pickpocket): price a method's hour

Outputs less inputs, with per-pickpocket loot scaled by the rate and
hourly loot counted once. Prices are the high value with no fallback,
matching -price rather than common::price_of, so a caller can check any
figure themselves. An item with no buy price contributes nothing and
flags the method rather than silently understating it.
EOF
)"
```

---

### Task 3: Output and wiring

**Files:**
- Modify: `src/pickpocket.rs`
- Modify: `src/lib.rs` (triggers, dispatch, help, trigger test)
- Modify: `README.md`

**Interfaces:**
- Consumes: everything from Tasks 1 and 2, plus `crate::common::{collect_hiscores, get_ge_data, get_item_db, short_gp, Entry, HiscoreName, Listing, MAX_SKILL_LEVEL, level_to_xp, xp_to_level}`, `crate::stats::{stats_parameters, strip_stats_parameters, level_display, StatsFlags}`, `crate::track::{MAX_LINE_LEN, pack_lines}`, `common::commas`.
- Produces: `pub fn lookup(source: Source) -> Result<Vec<String>>`.

`level_display` is `pub(crate)` in `stats.rs`, so it is reachable without a visibility change.

- [ ] **Step 1: Write the failing test**

Add to `src/pickpocket.rs`'s test module:

```rust
    #[test]
    fn profit_is_green_and_loss_is_red() {
        assert_eq!(gp(1_222_670), format!("{}1.2m", GREEN));
        assert_eq!(gp(-5_000), format!("{}-5.0k", RED));
        assert_eq!(gp(0), format!("{}0", GREEN));
    }

    #[test]
    fn a_rate_names_its_unit() {
        let ham = find_method("ham").expect("H.A.M. is in the table");
        let knights = find_method("knight").expect("knights are in the table");

        // H.A.M.'s rate is clues an hour, not pickpockets, so it must say so.
        assert_eq!(rate_line(ham), "18 easy clues/hr");
        assert_eq!(rate_line(knights), "3,000 pickpockets/hr");
    }

    #[test]
    fn experience_per_hour_is_the_rate_times_the_xp_each() {
        let knights = find_method("knight").expect("knights are in the table");

        // 3,000 x 84.3
        assert_eq!(xp_per_hour(knights).round() as i64, 252_900);
    }
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --lib pickpocket::`
Expected: FAIL — `gp`, `GREEN`, `RED`, `rate_line`, `xp_per_hour` do not exist.

- [ ] **Step 3: Write the display helpers**

Add to `src/pickpocket.rs`. Extend the imports at the top of the file to:

```rust
use anyhow::Result;
use common::commas;
use common::source::Source;

use crate::common::{
    Entry, HiscoreName, Listing, MAX_SKILL_LEVEL, collect_hiscores, get_ge_data, get_item_db,
    level_to_xp, short_gp, xp_to_level,
};
use crate::items::{Mapping, Price};
use crate::stats::{StatsFlags, level_display, stats_parameters, strip_stats_parameters};
use crate::track::{MAX_LINE_LEN, pack_lines};
use std::collections::HashMap;
```

```rust
/// Signed gp is coloured by sign rather than by the c1/c2 palette: those two
/// colours are per-user configurable and carry no profit/loss meaning.
const GREEN: &str = "\x0303";
const RED: &str = "\x0304";

fn gp(amount: i64) -> String {
    format!(
        "{}{}",
        if amount < 0 { RED } else { GREEN },
        short_gp(amount)
    )
}

/// `3,000 pickpockets/hr`. H.A.M.'s unit is easy clues, so the unit is carried
/// on the method rather than assumed.
fn rate_line(method: &Method) -> String {
    format!("{} {}/hr", commas(method.rate, "d"), method.rate_name)
}

fn xp_per_hour(method: &Method) -> f64 {
    method.rate * method.xp
}
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `cargo test --lib pickpocket::`
Expected: PASS, 21 tests.

- [ ] **Step 5: Write `lookup` and the two output modes**

Add to `src/pickpocket.rs`:

```rust
/// The Thieving listing to work from: the player's, or one conjured from `^N`.
fn thieving(source: &Source, prefix: &str, flags: &StatsFlags) -> Result<Listing, Vec<String>> {
    if flags.start > 0 {
        let xp = if flags.start > MAX_SKILL_LEVEL {
            flags.start
        } else {
            level_to_xp(flags.start)
        };

        return Ok(Listing::Entry(Entry {
            name: HiscoreName::Thieving,
            level: xp_to_level(xp),
            xp,
            rank: 0,
        }));
    }

    let joined: String = strip_stats_parameters(&source.query)
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    let hiscores = match collect_hiscores(&joined, source, flags) {
        Ok(hiscores) => hiscores,
        Err(_) => {
            return Err(vec![format!(
                "{} {}",
                prefix,
                source.c1("No hiscores found")
            )]);
        }
    };

    match hiscores.skill("Thieving") {
        Some(listing) => Ok(listing),
        None => Err(vec![format!(
            "{} {}",
            prefix,
            source.c1("No Thieving level found")
        )]),
    }
}

/// Every method ranked by profit, most profitable first. Methods above the
/// caller's level are kept - the market answer is useful before the level is -
/// but marked.
fn ranked(
    source: &Source,
    prefix: &str,
    level_string: &str,
    level: u32,
    items: &[Mapping],
    ge: &HashMap<u32, Price>,
) -> Vec<String> {
    let mut ranked: Vec<(&Method, Hourly)> = METHODS
        .iter()
        .map(|method| (method, hourly(method, items, ge)))
        .collect();

    ranked.sort_by(|(_, a), (_, b)| b.profit.cmp(&a.profit));

    let locked = ranked.iter().any(|(method, _)| level < method.thieving);
    let thin = ranked.iter().any(|(_, hour)| hour.unpriced);

    let parts: Vec<String> = ranked
        .iter()
        .map(|(method, hour)| {
            format!(
                "{} {}{}",
                source.c1(method.name),
                gp(hour.profit),
                if level < method.thieving {
                    source.c1("*")
                } else {
                    String::new()
                }
            )
        })
        .collect();

    let mut lines = pack_lines(
        &format!("{} {} {}", prefix, level_string, source.c1("Profit/hr:")),
        &parts,
        &source.c1(" | "),
        MAX_LINE_LEN,
    );

    let mut notes: Vec<String> = Vec::new();
    if locked {
        notes.push("* above your Thieving level".to_string());
    }
    if thin {
        notes.push("some items have no buy price".to_string());
    }
    if !notes.is_empty() {
        lines.push(format!("{} {}", prefix, source.p(&notes.join(" | "))));
    }

    lines
}

/// One method, with what it takes to do it.
fn detail(
    source: &Source,
    prefix: &str,
    level_string: &str,
    level: u32,
    method: &Method,
    items: &[Mapping],
    ge: &HashMap<u32, Price>,
) -> Vec<String> {
    let hour = hourly(method, items, ge);

    let header = vec![
        source.c2(method.name),
        level_string.to_string(),
        source.c1(&rate_line(method)),
        vec![
            source.c2(&commas(method.xp, ".1f")),
            source.c1("XP each"),
            source.p(&format!("{} XP/hr", short_gp(xp_per_hour(method).round() as i64))),
        ]
        .join(" "),
    ]
    .join(&source.c1(" | "));

    let money = vec![
        vec![source.c1("Profit/hr"), gp(hour.profit)].join(" "),
        vec![
            source.c1("Outputs"),
            source.c2(&commas(hour.outputs as f64, "d")),
        ]
        .join(" "),
        vec![
            source.c1("Inputs"),
            source.c2(&commas(hour.inputs as f64, "d")),
        ]
        .join(" "),
    ]
    .join(&source.c1(" | "));

    let mut lines = vec![
        format!("{} {}", prefix, header),
        format!("{} {}", prefix, money),
        format!(
            "{} {} {}",
            prefix,
            source.c1("Requires"),
            method
                .requirements
                .iter()
                .map(|req| source.c2(req))
                .collect::<Vec<String>>()
                .join(&source.c1(" | "))
        ),
    ];

    if level < method.thieving {
        lines.push(format!(
            "{} {}",
            prefix,
            source.c1(&format!("Requires {} Thieving", method.thieving))
        ));
    }

    if hour.unpriced {
        lines.push(format!(
            "{} {}",
            prefix,
            source.p("some items have no buy price, so the totals understate it")
        ));
    }

    lines
}

pub fn lookup(source: Source) -> Result<Vec<String>> {
    let prefix = source.l("Pickpocket");
    let flags = stats_parameters(&source.query);

    let items = get_item_db()?;
    let ge = get_ge_data()?;

    let listing = match thieving(&source, &prefix, &flags) {
        Ok(listing) => listing,
        Err(lines) => return Ok(lines),
    };

    let level = listing.actual_level();
    let (reported, virtual_level) = level_display(listing.level(), level);
    let level_string = vec![
        source.c1("Thieving"),
        source.c2(&reported.to_string()),
        virtual_level.map_or(String::new(), |lvl| source.p(&lvl.to_string())),
    ]
    .join(" ")
    .trim_end()
    .to_string();

    if flags.search.is_empty() {
        return Ok(ranked(&source, &prefix, &level_string, level, &items, &ge));
    }

    let method = match find_method(&flags.search) {
        Some(method) => method,
        None => {
            return Ok(vec![format!(
                "{} {}",
                prefix,
                source.c1(&format!(
                    "'{}' isn't a pickpocketing method - try -pickpocket for the full list",
                    flags.search
                ))
            )]);
        }
    };

    Ok(detail(
        &source,
        &prefix,
        &level_string,
        level,
        method,
        &items,
        &ge,
    ))
}
```

- [ ] **Step 6: Wire the command into `lib.rs`**

Add two lines to `TRIGGERS`, after `patch`:

```
^pickpocket\d*$
^pp\d*$
```

Both anchored, and both carry `\d*` so `-pickpocket2` reaches the plugin to select a second linked RSN. Neither collides with an existing pattern — `pp` is not matched by `(pvparena|pvp|arena)\d*`, `params?`, `patch`, `payment|plants?` or `pray(er)?`.

Add the dispatch arm after the `"patch"` arm:

```rust
"pickpocket" | "pp" => pickpocket::lookup(source),
```

Add to the `"help"` list, between `patch` and `players`:

```
pickpocket[N]
```

Add `"pickpocket"`, `"pickpocket2"` and `"pp"` to the `commands_are_not_dispatched_twice` test's command list, after `"patch"`.

- [ ] **Step 7: Run the full suite**

Run: `cargo test`
Expected: Task 2's total plus 3, 0 failures.

Run: `cargo build --release 2>&1 | grep -E "^(warning|error)"`
Expected: no output.

- [ ] **Step 8: Document it in the README**

Add under `### Economy`, after the `-money` entry:

```markdown
- `-pickpocket[N] [@method] [^level] [RSN]` — Profit per hour for the six
  pickpocketing money makers, ranked, with methods above your Thieving level
  marked. `@method` details one — its rate, XP, and what it requires. Profit is
  green and losses red, priced from the same Grand Exchange values `-price`
  reports and net of the 2% tax. Rogue equipment is assumed, since the wiki's
  loot rates include it (alias `-pp`).
```

- [ ] **Step 9: Commit**

```bash
git add src/pickpocket.rs src/lib.rs README.md
git commit -m "$(cat <<'EOF'
feat(pickpocket): -pickpocket command

Ranks the six pickpocketing money makers by profit per hour and details
one with its requirements when named with @method. Profit prints green
and losses red; a method with an unpriced item says so rather than
quietly reporting a total that is too low.
EOF
)"
```

---

### Task 4: The casket value refresher

**Files:**
- Create: `scripts/gen-clue-values.js`
- Modify: `src/pickpocket.rs` (only if the script finds different values)

**Interfaces:**
- Consumes: `EASY_CASKET_GP` and `MASTER_CASKET_GP` in `src/pickpocket.rs`, which it rewrites in place.
- Produces: nothing other code depends on.

**Background.** A reward casket's value comes from its drop table, not the Grand Exchange — 283 rows for easy, a 170-term expression for master. Rather than port those, the wiki evaluates its own `{{EasyClueValue}}` and `{{MasterClueValue}}` templates against current prices and this script writes the results into the Rust source. It is run by hand, not on a schedule, and `node` is already available.

- [ ] **Step 1: Write the script**

Create `scripts/gen-clue-values.js`:

```javascript
// Refreshes the casket value constants in src/pickpocket.rs from the OSRS
// wiki. A casket's worth comes from its reward table rather than the Grand
// Exchange, so the wiki evaluates it and we bake in the result.
//
//   node scripts/gen-clue-values.js
//
// Fails loudly rather than writing a bad value.

const fs = require('fs');
const path = require('path');

const API = 'https://oldschool.runescape.wiki/api.php';
const UA = { headers: { 'User-Agent': 'Reinze.com' } };
const TARGET = path.join(__dirname, '..', 'src', 'pickpocket.rs');

async function expand(template) {
  const url = `${API}?action=parse&text=${encodeURIComponent(template)}` +
    `&contentmodel=wikitext&prop=text&format=json`;
  const res = await fetch(url, UA);
  if (!res.ok) throw new Error(`${template}: HTTP ${res.status}`);

  const json = await res.json();
  const text = (json.parse?.text?.['*'] || '').replace(/<[^>]+>/g, '').trim();
  const value = parseFloat(text.replace(/,/g, ''));

  if (!isFinite(value) || value <= 0) {
    throw new Error(`${template} did not evaluate to a number: ${text.slice(0, 80)}`);
  }
  return Math.round(value);
}

(async () => {
  const easy = await expand('{{EasyClueValue}}');
  const master = await expand('{{MasterClueValue}}');

  // A casket is worth more than a coin and less than a bank. If either lands
  // outside that, the template changed shape and a human should look.
  for (const [name, value] of [['easy', easy], ['master', master]]) {
    if (value < 100 || value > 50_000_000) {
      throw new Error(`${name} casket value looks wrong: ${value}`);
    }
  }

  const source = fs.readFileSync(TARGET, 'utf8');
  const updated = source
    .replace(/pub const EASY_CASKET_GP: f64 = [\d_.]+;/,
             `pub const EASY_CASKET_GP: f64 = ${easy.toLocaleString('en-US').replace(/,/g, '_')}.0;`)
    .replace(/pub const MASTER_CASKET_GP: f64 = [\d_.]+;/,
             `pub const MASTER_CASKET_GP: f64 = ${master.toLocaleString('en-US').replace(/,/g, '_')}.0;`);

  if (updated === source) {
    console.log(`no change: easy ${easy}, master ${master}`);
    return;
  }

  fs.writeFileSync(TARGET, updated);
  console.log(`updated: easy ${easy}, master ${master}`);
})();
```

- [ ] **Step 2: Run it**

Run: `node scripts/gen-clue-values.js`
Expected: either `no change: easy 9773, master 108946`, or `updated: ...` with values near those. Anything wildly different means the wiki changed and needs a human look — say so rather than committing it blindly.

- [ ] **Step 3: Confirm the source still builds**

Run: `cargo test --lib pickpocket::`
Expected: PASS. If the script rewrote the constants, the casket test still passes because it compares against the constant rather than a literal.

Run: `git diff src/pickpocket.rs`
Expected: either no diff, or only the two constant lines.

- [ ] **Step 4: Commit**

```bash
git add scripts/gen-clue-values.js src/pickpocket.rs
git commit -m "$(cat <<'EOF'
chore(pickpocket): script to refresh the casket values

A casket's worth comes from its reward table - 283 rows for easy, a
170-term expression for master - so the wiki evaluates its own templates
and this writes the results into the constants. Run by hand.
EOF
)"
```

---

## Manual verification

`cargo test` covers the maths but not the price-backed paths, which need
`lib/ge.json` and `lib/item_db.json` present — that is how the plugin runs in
production, not how it runs in a checkout.

| Command | Expect |
|---|---|
| `-pickpocket` | one or two lines, six methods, most profitable first, `*` on any above your level |
| `-pickpocket @vyres` | three lines: header, money, requirements |
| `-pickpocket @ham` | resolves to H.A.M. members, and its rate reads `18 easy clues/hr` |
| `-pickpocket @knight` | resolves to Knights of Ardougne |
| `-pickpocket @guards` | the "isn't a pickpocketing method" line |
| `-pickpocket @elves ^40` | the "Requires 85 Thieving" line, no hiscore lookup |
| `-pp` | same as `-pickpocket` |
| `-pickpocket2` | uses the second linked RSN |

**Cross-check the figures against the wiki once prices are wired up.** At the
time of writing the guides report: elves 4,529,773, vyres 2,514,262, master
farmers 1,222,670, H.A.M. members 699,643, paladins 644,457, Knights of Ardougne
262,755. These move with prices, so expect drift, not equality — but an order of
magnitude apart means a quantity or a unit is wrong.

## Notes for the implementer

- **Do not swap `ge_high` for `common::price_of`.** They differ deliberately:
  `price_of` falls back to the sell offer, and this command matches `-price`,
  which does not. A test pins this.
- **The spec listed a `No prices` error case; this plan handles it differently
  and on purpose.** Rather than a separate line for "nothing priced at all",
  every method carries an `unpriced` flag and the output says so in a footnote.
  That covers the total failure and the far more likely partial one — a single
  item with no recent buy offer — with one mechanism instead of two.
- **Do not "fix" the loot quantities.** They are the wiki's expected values,
  already including drop rates and rogue-equipment doubling. The original
  expression is in a comment beside each one.
- **`commas(value, "d")`** formats an integer with thousands separators and
  keeps a minus sign; `commas(value, ".1f")` gives one decimal.
- **`pack_lines`** measures bytes including colour codes, so the `GREEN`/`RED`
  prefixes are accounted for automatically.
- **`Mapping` and `Price`** are declared in `src/items.rs`; `Price` has `high`
  and `low`, both `Option<u32>`.
