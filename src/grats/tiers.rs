use crate::common::skills;

/// One rung of a congratulation ladder: a badge and the interchangeable
/// messages that go with it. Each variant carries exactly one `{}` marking
/// where the coloured value is spliced in.
pub struct Tier {
    pub emoji: &'static str,
    pub variants: &'static [&'static str],
}

const L_1_9: Tier = Tier {
    emoji: "🌱",
    variants: &[
        "Everyone starts somewhere — grats on {}!",
        "{} down, a whole lot to go. Nice start!",
        "Fresh off Tutorial Island? Grats on {}!",
    ],
};

const L_10_19: Tier = Tier {
    emoji: "🐣",
    variants: &[
        "Grats on {}! The grind begins.",
        "{}! You're getting the hang of this.",
        "Nice, {}. Keep clicking.",
    ],
};

const L_20_34: Tier = Tier {
    emoji: "🪓",
    variants: &[
        "Grats on {}! Steady progress.",
        "{} — the early levels are flying by.",
        "Well done on {}! Momentum is building.",
    ],
};

const L_35_49: Tier = Tier {
    emoji: "⛏️",
    variants: &[
        "Grats on {}! Real work happening now.",
        "{}. You're past the easy part.",
        "Nice one, {}! The grind respects you.",
    ],
};

const L_50_59: Tier = Tier {
    emoji: "💪",
    variants: &[
        "Halfway to something special. Grats on {}!",
        "{}! Solid, respectable, hard-earned.",
        "Grats on {}. That's a proper level.",
    ],
};

const L_60_69: Tier = Tier {
    emoji: "🔥",
    variants: &[
        "Grats on {}! You're cooking now.",
        "{} — the sweaty levels start here.",
        "Look at {}! Impressive stuff.",
    ],
};

const L_70_79: Tier = Tier {
    emoji: "🏆",
    variants: &[
        "Grats on {}! You are a CHAMPION!",
        "{}! That took actual dedication.",
        "Big grats on {}. Elite territory.",
    ],
};

const L_80_89: Tier = Tier {
    emoji: "🌟",
    variants: &[
        "Grats on {}! You are one of the elite!",
        "{} — now we're talking. Serious work.",
        "Outstanding! Congratulations on {}!",
    ],
};

const L_90_94: Tier = Tier {
    emoji: "💎",
    variants: &[
        "Congratulations on {}! I am not worthy!",
        "{}! The home stretch. Incredible.",
        "Grats on {}. That is genuinely rare air.",
    ],
};

const L_95_98: Tier = Tier {
    emoji: "🚀",
    variants: &[
        "So close! Huge congratulations on {}!",
        "{} — 99 is RIGHT THERE. Don't stop now!",
        "Grats on {}! The cape is in sight.",
    ],
};

const L_99: Tier = Tier {
    emoji: "🎓",
    variants: &[
        "\\o/ CONGRATULATIONS ON {}! Go buy that cape!",
        "{}! You are a true Runescaper!",
        "GRATS ON {}!!! Skill cape earned. Wear it with pride.",
    ],
};

const L_100_119: Tier = Tier {
    emoji: "🧙",
    variants: &[
        "Virtual levels? Congratulations on {}, you absolute machine.",
        "{} — past 99 and still going. Terrifying.",
        "Grats on {}. Most people stopped a long time ago.",
    ],
};

const L_120: Tier = Tier {
    emoji: "🏅",
    variants: &[
        "Congratulations on {}! Master cape territory.",
        "{}! That is a monumental amount of work.",
        "Grats on {}. Genuinely elite. Go rest.",
    ],
};

const L_121_125: Tier = Tier {
    emoji: "🛸",
    variants: &[
        "Congratulations on {}. I have run out of adjectives.",
        "{} — this is beyond the game at this point.",
        "Grats on {}. Please, go outside for a moment.",
    ],
};

const L_126_PLUS: Tier = Tier {
    emoji: "🌌",
    variants: &[
        "\\o/ CONGRATULATIONS ON {}! You are a true Runescaper!",
        "{}. The absolute ceiling. Incredible.",
        "Grats on {}! Nothing left to climb.",
    ],
};

/// Level ladder, used for any milestone at or below 150.
pub fn level_tier(level: u32) -> &'static Tier {
    match level {
        0..=9 => &L_1_9,
        10..=19 => &L_10_19,
        20..=34 => &L_20_34,
        35..=49 => &L_35_49,
        50..=59 => &L_50_59,
        60..=69 => &L_60_69,
        70..=79 => &L_70_79,
        80..=89 => &L_80_89,
        90..=94 => &L_90_94,
        95..=98 => &L_95_98,
        99 => &L_99,
        100..=119 => &L_100_119,
        120 => &L_120,
        121..=125 => &L_121_125,
        _ => &L_126_PLUS,
    }
}

// Exact game breakpoints. Deliberately NOT derived from common::level_to_xp,
// which accumulates in f32 and drifts by a few XP at the top of the curve.
pub const XP_92: u64 = 6_517_253;
pub const XP_99: u64 = 13_034_431;
pub const XP_120: u64 = 104_273_167;
pub const XP_MAX: u64 = 200_000_000;

const X_SUB_100K: Tier = Tier {
    emoji: "🌱",
    variants: &[
        "Grats on {}! Every bit counts.",
        "{} — off to a start!",
        "Nice, {}. Onwards.",
    ],
};

const X_SUB_500K: Tier = Tier {
    emoji: "🐣",
    variants: &[
        "Congrats on {}! Building up nicely.",
        "{}. The numbers are getting real.",
        "Grats on {}! Keep at it.",
    ],
};

const X_SUB_1M: Tier = Tier {
    emoji: "🪙",
    variants: &[
        "Grats on {}! Nearly a million.",
        "{} — the first million is in sight.",
        "Congrats on {}. Almost there!",
    ],
};

const X_SUB_2_5M: Tier = Tier {
    emoji: "📈",
    variants: &[
        "Congratulations on {}! Moving on up!",
        "{}! Welcome to the millions club.",
        "Grats on {}. Real progress.",
    ],
};

const X_SUB_5M: Tier = Tier {
    emoji: "⛏️",
    variants: &[
        "Congrats on {}! Grinding away.",
        "{} — that is a lot of clicks.",
        "Nice work on {}!",
    ],
};

const X_SUB_92: Tier = Tier {
    emoji: "💪",
    variants: &[
        "Congrats on {}! 92 is closing in.",
        "{} — nearly at 92, the real halfway mark.",
        "Grats on {}. Almost halfway to 99!",
    ],
};

const X_92_TO_10M: Tier = Tier {
    emoji: "🔥",
    variants: &[
        "Congrats on {}! Past 92 — you're more than halfway to 99.",
        "{}. 92 down. The back half is shorter than it looks.",
        "More than halfway there! Keep on trucking! Congratulations for {}!",
    ],
};

const X_SUB_99: Tier = Tier {
    emoji: "🚀",
    variants: &[
        "ALMOST TO 99! Congratulations for reaching {}! *jealous*",
        "{} — 99 is within touching distance.",
        "Grats on {}! Do not stop now.",
    ],
};

const X_EXACTLY_99: Tier = Tier {
    emoji: "🎓",
    variants: &[
        "\\o/ {} — that is EXACTLY 99. Cape earned!",
        "{}. Bang on 99. Beautiful number.",
        "CONGRATULATIONS on {}! Precisely 99. Go get the cape.",
    ],
};

const X_SUB_20M: Tier = Tier {
    emoji: "🏆",
    variants: &[
        "IMPRESSIVE WORK! You must really love this skill. Congrats on {}.",
        "{} — post-99 and still going!",
        "WOW, congratulations on {}! Go get yourself a snack. You earned it.",
    ],
};

const X_SUB_50M: Tier = Tier {
    emoji: "💎",
    variants: &[
        "I'm jealous of your {}! Congrats though!",
        "{}. That is a frightening amount of dedication.",
        "Grats on {}! Well past what most will ever do.",
    ],
};

const X_SUB_100M: Tier = Tier {
    emoji: "👑",
    variants: &[
        "You might be insane! Incredible congratulations on {}!",
        "{} — everyone else is super jelly of your skillz.",
        "Grats on {}. Genuinely absurd. Respect.",
    ],
};

const X_SUB_120: Tier = Tier {
    emoji: "🧙",
    variants: &[
        "I have no more words for you. Congrats on {}, you beast.",
        "{}. I am Hulk green with envy.",
        "Congratulations on {}! Nine figures. Unbelievable.",
    ],
};

const X_120_TO_MAX: Tier = Tier {
    emoji: "🏅",
    variants: &[
        "Congratulations on {}! That is level 120 territory.",
        "{} — master cape numbers. Astonishing.",
        "Grats on {}. The end is actually in sight now.",
    ],
};

const X_MAXED: Tier = Tier {
    emoji: "🌌",
    variants: &[
        "Okay, you win. Endless congratulations on {}. Go get some sunshine and a nice snack to celebrate!",
        "{}. MAXED. You are on the highscores forever.",
        "\\o/ {} — 200 MILLION. There is nothing left. Go outside.",
    ],
};

const IMPOSSIBLE: Tier = Tier {
    emoji: "❌",
    variants: &[
        "{}? That is not even a thing, get out of here.",
        "{} is not a number this game recognises. Nice try.",
        "{}? Absolutely not. Get out of here.",
    ],
};

/// XP ladder, used for any non-Overall milestone above 150.
pub fn xp_tier(xp: u64) -> &'static Tier {
    match xp {
        0..100_000 => &X_SUB_100K,
        100_000..500_000 => &X_SUB_500K,
        500_000..1_000_000 => &X_SUB_1M,
        1_000_000..2_500_000 => &X_SUB_2_5M,
        2_500_000..5_000_000 => &X_SUB_5M,
        5_000_000..XP_92 => &X_SUB_92,
        XP_92..10_000_000 => &X_92_TO_10M,
        10_000_000..XP_99 => &X_SUB_99,
        XP_99 => &X_EXACTLY_99,
        // XP_99 + 1 ..= 19_999_999 — range patterns cannot hold arithmetic.
        13_034_432..20_000_000 => &X_SUB_20M,
        20_000_000..50_000_000 => &X_SUB_50M,
        50_000_000..100_000_000 => &X_SUB_100M,
        100_000_000..XP_120 => &X_SUB_120,
        XP_120..XP_MAX => &X_120_TO_MAX,
        XP_MAX => &X_MAXED,
        _ => &IMPOSSIBLE,
    }
}

/// Max total level, derived from the skill list rather than hardcoded so it
/// self-corrects when a skill is added. skills() includes "Overall" itself.
pub fn max_total_level() -> u64 {
    (skills().len() as u64 - 1) * 99
}

/// Max total XP, derived the same way: every skill capped at 200m.
pub fn max_total_xp() -> u64 {
    (skills().len() as u64 - 1) * 200_000_000
}

const O_SUB_750: Tier = Tier {
    emoji: "🌱",
    variants: &[
        "Grats on {}! The account is taking shape.",
        "{} — early days, but it's moving.",
        "Nice, {}! Plenty of skills left to touch.",
    ],
};

const O_SUB_1250: Tier = Tier {
    emoji: "🐣",
    variants: &[
        "Congrats on {}! Filling out nicely.",
        "{}. A proper account is forming.",
        "Grats on {}! Good spread of skills.",
    ],
};

const O_SUB_1600: Tier = Tier {
    emoji: "⛏️",
    variants: &[
        "Congratulations on {}! Solid all-rounder.",
        "{} — no more skipping the hard skills.",
        "Grats on {}. Well-rounded and climbing.",
    ],
};

const O_SUB_1900: Tier = Tier {
    emoji: "💪",
    variants: &[
        "Grats on {}! That is serious account building.",
        "{}. Most accounts never get here.",
        "Congrats on {}! The hard part starts now.",
    ],
};

const O_SUB_2100: Tier = Tier {
    emoji: "🔥",
    variants: &[
        "Congratulations on {}! Maxing is on the table.",
        "{} — the finish line is visible from here.",
        "Grats on {}. Only the painful ones left.",
    ],
};

const O_SUB_2300: Tier = Tier {
    emoji: "🏆",
    variants: &[
        "Grats on {}! So close to max.",
        "{}. A handful of 99s from the top.",
        "Congratulations on {}! Nearly there.",
    ],
};

const O_SUB_MAX: Tier = Tier {
    emoji: "💎",
    variants: &[
        "Congratulations on {}! Agonisingly close to max.",
        "{} — one or two skills from the cape.",
        "Grats on {}. Finish it!",
    ],
};

const O_MAXED: Tier = Tier {
    emoji: "🌌",
    variants: &[
        "\\o/ {} — MAX TOTAL. Every single 99. Legendary.",
        "{}. You maxed. Go get some sunshine and a nice snack to celebrate!",
        "CONGRATULATIONS on {}! Nothing left. Absolute legend.",
    ],
};

const OX_SUB_100M: Tier = Tier {
    emoji: "🌱",
    variants: &[
        "Grats on {}! The total is climbing.",
        "{} — every skill adds up.",
        "Nice work on {}!",
    ],
};

const OX_SUB_500M: Tier = Tier {
    emoji: "📈",
    variants: &[
        "Congratulations on {}! Serious total.",
        "{}. That is a lot of hours.",
        "Grats on {}! Well into the hundreds of millions.",
    ],
};

const OX_SUB_1B: Tier = Tier {
    emoji: "💎",
    variants: &[
        "Congrats on {}! Approaching the billion.",
        "{} — the billion is in sight.",
        "Grats on {}. Genuinely impressive total.",
    ],
};

const OX_SUB_2B: Tier = Tier {
    emoji: "👑",
    variants: &[
        "A BILLION. Congratulations on {}!",
        "{}. Ten digits of experience. Wild.",
        "Grats on {}! Billionaire status.",
    ],
};

const OX_SUB_3_5B: Tier = Tier {
    emoji: "🧙",
    variants: &[
        "Congratulations on {}! This is beyond dedication.",
        "{} — I do not know what to say any more.",
        "Grats on {}. Truly absurd numbers.",
    ],
};

const OX_SUB_MAX: Tier = Tier {
    emoji: "🛸",
    variants: &[
        "Congratulations on {}! The ceiling is close.",
        "{}. Almost every skill at 200m. Terrifying.",
        "Grats on {}! Nearly the theoretical maximum.",
    ],
};

const OX_MAXED: Tier = Tier {
    emoji: "🌌",
    variants: &[
        "\\o/ {} — MAX XP. 200m in everything. There is nothing left.",
        "{}. The literal ceiling of the game. Go outside.",
        "CONGRATULATIONS on {}! You have completed RuneScape.",
    ],
};

/// Overall total-level ladder, used when the milestone is at or below
/// max_total_level().
pub fn overall_level_tier(total: u64) -> &'static Tier {
    if total >= max_total_level() {
        return &O_MAXED;
    }

    match total {
        0..750 => &O_SUB_750,
        750..1_250 => &O_SUB_1250,
        1_250..1_600 => &O_SUB_1600,
        1_600..1_900 => &O_SUB_1900,
        1_900..2_100 => &O_SUB_2100,
        2_100..2_300 => &O_SUB_2300,
        _ => &O_SUB_MAX,
    }
}

/// Overall total-XP ladder, used when the milestone exceeds max_total_level().
pub fn overall_xp_tier(total: u64) -> &'static Tier {
    if total > max_total_xp() {
        return &IMPOSSIBLE;
    }
    if total == max_total_xp() {
        return &OX_MAXED;
    }

    match total {
        0..100_000_000 => &OX_SUB_100M,
        100_000_000..500_000_000 => &OX_SUB_500M,
        500_000_000..1_000_000_000 => &OX_SUB_1B,
        1_000_000_000..2_000_000_000 => &OX_SUB_2B,
        2_000_000_000..3_500_000_000 => &OX_SUB_3_5B,
        _ => &OX_SUB_MAX,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_tier_boundaries() {
        assert_eq!(level_tier(1).emoji, "🌱");
        assert_eq!(level_tier(9).emoji, "🌱");
        assert_eq!(level_tier(10).emoji, "🐣");
        assert_eq!(level_tier(19).emoji, "🐣");
        assert_eq!(level_tier(20).emoji, "🪓");
        assert_eq!(level_tier(34).emoji, "🪓");
        assert_eq!(level_tier(35).emoji, "⛏️");
        assert_eq!(level_tier(49).emoji, "⛏️");
        assert_eq!(level_tier(50).emoji, "💪");
        assert_eq!(level_tier(59).emoji, "💪");
        assert_eq!(level_tier(60).emoji, "🔥");
        assert_eq!(level_tier(69).emoji, "🔥");
        assert_eq!(level_tier(70).emoji, "🏆");
        assert_eq!(level_tier(79).emoji, "🏆");
        assert_eq!(level_tier(80).emoji, "🌟");
        assert_eq!(level_tier(89).emoji, "🌟");
        assert_eq!(level_tier(90).emoji, "💎");
        assert_eq!(level_tier(94).emoji, "💎");
        assert_eq!(level_tier(95).emoji, "🚀");
        assert_eq!(level_tier(98).emoji, "🚀");
    }

    #[test]
    fn level_tier_single_value_milestones() {
        // 99, 120 and 126 are single-value tiers: skill cape, master cape, max combat.
        assert_eq!(level_tier(99).emoji, "🎓");
        assert_eq!(level_tier(100).emoji, "🧙");
        assert_eq!(level_tier(119).emoji, "🧙");
        assert_eq!(level_tier(120).emoji, "🏅");
        assert_eq!(level_tier(121).emoji, "🛸");
        assert_eq!(level_tier(125).emoji, "🛸");
        assert_eq!(level_tier(126).emoji, "🌌");
        assert_eq!(level_tier(150).emoji, "🌌");
    }

    #[test]
    fn level_tier_handles_zero() {
        // Not reachable through +gz (the regex rejects an empty milestone and
        // Combat clamps at 4), but the function must not panic on it.
        assert_eq!(level_tier(0).emoji, "🌱");
    }

    #[test]
    fn xp_tier_lower_bands() {
        assert_eq!(xp_tier(151).emoji, "🌱");
        assert_eq!(xp_tier(99_999).emoji, "🌱");
        assert_eq!(xp_tier(100_000).emoji, "🐣");
        assert_eq!(xp_tier(499_999).emoji, "🐣");
        assert_eq!(xp_tier(500_000).emoji, "🪙");
        assert_eq!(xp_tier(999_999).emoji, "🪙");
        assert_eq!(xp_tier(1_000_000).emoji, "📈");
        assert_eq!(xp_tier(2_499_999).emoji, "📈");
        assert_eq!(xp_tier(2_500_000).emoji, "⛏️");
        assert_eq!(xp_tier(4_999_999).emoji, "⛏️");
    }

    #[test]
    fn xp_tier_92_is_the_halfway_breakpoint() {
        assert_eq!(XP_92, 6_517_253);
        assert_eq!(xp_tier(5_000_000).emoji, "💪");
        assert_eq!(xp_tier(XP_92 - 1).emoji, "💪");
        assert_eq!(xp_tier(XP_92).emoji, "🔥");
        assert_eq!(xp_tier(9_999_999).emoji, "🔥");
    }

    #[test]
    fn xp_tier_99_is_an_exact_single_value_tier() {
        assert_eq!(XP_99, 13_034_431);
        assert_eq!(xp_tier(10_000_000).emoji, "🚀");
        assert_eq!(xp_tier(XP_99 - 1).emoji, "🚀");
        assert_eq!(xp_tier(XP_99).emoji, "🎓");
        // One XP past 99 must fall out of the cape tier, not linger in it.
        assert_eq!(xp_tier(XP_99 + 1).emoji, "🏆");
    }

    #[test]
    fn xp_tier_upper_bands() {
        assert_eq!(xp_tier(19_999_999).emoji, "🏆");
        assert_eq!(xp_tier(20_000_000).emoji, "💎");
        assert_eq!(xp_tier(49_999_999).emoji, "💎");
        assert_eq!(xp_tier(50_000_000).emoji, "👑");
        assert_eq!(xp_tier(99_999_999).emoji, "👑");
        assert_eq!(xp_tier(100_000_000).emoji, "🧙");
    }

    #[test]
    fn xp_tier_120_and_max() {
        assert_eq!(XP_120, 104_273_167);
        assert_eq!(XP_MAX, 200_000_000);
        assert_eq!(xp_tier(XP_120 - 1).emoji, "🧙");
        assert_eq!(xp_tier(XP_120).emoji, "🏅");
        assert_eq!(xp_tier(XP_MAX - 1).emoji, "🏅");
        assert_eq!(xp_tier(XP_MAX).emoji, "🌌");
        assert_eq!(xp_tier(XP_MAX + 1).emoji, "❌");
        // u64, not u32 — this must not saturate or wrap.
        assert_eq!(xp_tier(4_600_000_000).emoji, "❌");
    }

    #[test]
    fn overall_maxima_are_derived_from_the_skill_list() {
        // 24 skills today (Attack through Sailing); skills() also contains
        // "Overall" itself, hence the -1. These must not be hardcoded literals
        // in the implementation — they self-correct when Jagex adds a skill.
        assert_eq!(max_total_level(), 2376);
        assert_eq!(max_total_xp(), 4_800_000_000);
    }

    #[test]
    fn overall_level_tier_boundaries() {
        assert_eq!(overall_level_tier(0).emoji, "🌱");
        assert_eq!(overall_level_tier(749).emoji, "🌱");
        assert_eq!(overall_level_tier(750).emoji, "🐣");
        assert_eq!(overall_level_tier(1_249).emoji, "🐣");
        assert_eq!(overall_level_tier(1_250).emoji, "⛏️");
        assert_eq!(overall_level_tier(1_599).emoji, "⛏️");
        assert_eq!(overall_level_tier(1_600).emoji, "💪");
        assert_eq!(overall_level_tier(1_899).emoji, "💪");
        assert_eq!(overall_level_tier(1_900).emoji, "🔥");
        assert_eq!(overall_level_tier(2_099).emoji, "🔥");
        assert_eq!(overall_level_tier(2_100).emoji, "🏆");
        assert_eq!(overall_level_tier(2_299).emoji, "🏆");
        assert_eq!(overall_level_tier(2_300).emoji, "💎");
        assert_eq!(overall_level_tier(2_375).emoji, "💎");
        assert_eq!(overall_level_tier(2_376).emoji, "🌌");
    }

    #[test]
    fn overall_xp_tier_boundaries() {
        assert_eq!(overall_xp_tier(0).emoji, "🌱");
        assert_eq!(overall_xp_tier(99_999_999).emoji, "🌱");
        assert_eq!(overall_xp_tier(100_000_000).emoji, "📈");
        assert_eq!(overall_xp_tier(499_999_999).emoji, "📈");
        assert_eq!(overall_xp_tier(500_000_000).emoji, "💎");
        assert_eq!(overall_xp_tier(999_999_999).emoji, "💎");
        assert_eq!(overall_xp_tier(1_000_000_000).emoji, "👑");
        assert_eq!(overall_xp_tier(1_999_999_999).emoji, "👑");
        assert_eq!(overall_xp_tier(2_000_000_000).emoji, "🧙");
        assert_eq!(overall_xp_tier(3_499_999_999).emoji, "🧙");
        assert_eq!(overall_xp_tier(3_500_000_000).emoji, "🛸");
    }

    #[test]
    fn overall_xp_tier_max_and_beyond() {
        assert_eq!(overall_xp_tier(max_total_xp() - 1).emoji, "🛸");
        assert_eq!(overall_xp_tier(max_total_xp()).emoji, "🌌");
        assert_eq!(overall_xp_tier(max_total_xp() + 1).emoji, "❌");
    }

    /// Every tier reachable through any ladder, gathered by walking the lookup
    /// functions across their full input ranges. Deduplicated by emoji + first
    /// variant so shared tiers (IMPOSSIBLE) are only checked once.
    fn all_tiers() -> Vec<&'static Tier> {
        let mut seen: Vec<&'static Tier> = Vec::new();

        let mut push = |t: &'static Tier| {
            if !seen
                .iter()
                .any(|s| s.emoji == t.emoji && s.variants.first() == t.variants.first())
            {
                seen.push(t);
            }
        };

        for level in 0..=200u32 {
            push(level_tier(level));
        }
        for xp in [
            0u64,
            99_999,
            100_000,
            499_999,
            500_000,
            999_999,
            1_000_000,
            2_499_999,
            2_500_000,
            4_999_999,
            5_000_000,
            XP_92 - 1,
            XP_92,
            9_999_999,
            10_000_000,
            XP_99 - 1,
            XP_99,
            XP_99 + 1,
            19_999_999,
            20_000_000,
            49_999_999,
            50_000_000,
            99_999_999,
            100_000_000,
            XP_120 - 1,
            XP_120,
            XP_MAX - 1,
            XP_MAX,
            XP_MAX + 1,
        ] {
            push(xp_tier(xp));
        }
        for total in [
            0u64, 749, 750, 1_249, 1_250, 1_599, 1_600, 1_899, 1_900, 2_099, 2_100, 2_299, 2_300,
            2_375, 2_376,
        ] {
            push(overall_level_tier(total));
        }
        for total in [
            0u64,
            99_999_999,
            100_000_000,
            499_999_999,
            500_000_000,
            999_999_999,
            1_000_000_000,
            1_999_999_999,
            2_000_000_000,
            3_499_999_999,
            3_500_000_000,
            max_total_xp() - 1,
            max_total_xp(),
            max_total_xp() + 1,
        ] {
            push(overall_xp_tier(total));
        }

        seen
    }

    #[test]
    fn every_variant_has_exactly_one_placeholder() {
        for tier in all_tiers() {
            for variant in tier.variants {
                assert_eq!(
                    variant.matches("{}").count(),
                    1,
                    "tier {} variant {:?} must contain exactly one {{}}",
                    tier.emoji,
                    variant
                );
            }
        }
    }

    #[test]
    fn every_tier_has_variants() {
        for tier in all_tiers() {
            assert!(
                !tier.variants.is_empty(),
                "tier {} has no variants",
                tier.emoji
            );
            assert!(
                !tier.emoji.is_empty(),
                "tier with variants {:?} has no emoji",
                tier.variants
            );
        }
    }

    #[test]
    fn variants_within_a_tier_are_distinct() {
        for tier in all_tiers() {
            for (i, a) in tier.variants.iter().enumerate() {
                for b in tier.variants.iter().skip(i + 1) {
                    assert_ne!(a, b, "tier {} repeats a variant", tier.emoji);
                }
            }
        }
    }

    #[test]
    fn all_ladders_are_covered() {
        // 15 level + 16 XP + 8 Overall-level + 7 Overall-XP tiers, minus the
        // IMPOSSIBLE tier counted once instead of twice.
        assert!(
            all_tiers().len() >= 40,
            "expected at least 40 distinct tiers, found {}",
            all_tiers().len()
        );
    }
}
