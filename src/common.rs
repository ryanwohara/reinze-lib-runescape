pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f
            .to_uppercase()
            .chain(c.flat_map(|c| c.to_lowercase()))
            .collect(),
    }
}

// Gray
// c1
pub fn c1(s: &str) -> String {
    format!("\x0314{}", s)
}

// Red
// c2
pub fn c2(s: &str) -> String {
    format!("\x0304{}", s)
}

// Red
// c3
pub fn c3(s: &str) -> String {
    format!("\x0305{}", s)
}

// Green
// c4
pub fn c4(s: &str) -> String {
    format!("\x0303{}", s)
}

// Yellow
// c5
pub fn c5(s: &str) -> String {
    format!("\x0307{}", s)
}

// A function for wrapping a string in brackets that are colored gray
// l
pub fn l(s: &str) -> String {
    format!("{}{}{}", c1("["), c2(s), c1("]"))
}

// A function for wrapping a string in parenthesis that are colored gray
// p
pub fn p(s: &str) -> String {
    format!("{}{}{}", c1("("), c2(s), c1(")"))
}

// Converts a level to experience
pub fn level_to_xp(level: u32) -> u32 {
    let mut xp = 0;

    for i in 1..level {
        let x = i as f32;

        xp += (x + 300.0 * 2.0_f32.powf(x / 7.0)).floor() as u32 / 4;
    }

    xp
}

// Converts experience to a level
pub fn xp_to_level(xp: u32) -> u32 {
    for level in 1..=127 {
        if xp <= level_to_xp(level) {
            return level;
        }
    }

    0
}
