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
}
