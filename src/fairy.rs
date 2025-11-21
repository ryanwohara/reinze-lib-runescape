use common::{c1, c2, l, p};

pub fn lookup(query: &str) -> Result<Vec<String>, ()> {
    let prefix = l("Fairy Ring");

    if query.len() == 0 {
        return Ok(vec![format!("{} {}", prefix, c1("No query provided"))]);
    }

    for ring in FairyRing::all() {
        let details = ring.details();

        if details
            .code
            .to_ascii_lowercase()
            .contains(&query.to_ascii_lowercase())
            || details
                .location
                .to_ascii_lowercase()
                .contains(&query.to_ascii_lowercase())
        {
            return Ok(vec![format!("{} {}", prefix, details.to_string(),)]);
        }
    }

    Ok(vec![format!("{}: {}", prefix, c1("No results found"))])
}

enum FairyRing {
    AIQ,
    AJR,
    AJS,
    AKQ,
    AKS,
    ALP,
    ALQ,
    ALR,
    ALS,
    BIQ,
    BIS,
    BJR,
    BJS,
    BKP,
    BKQ,
    BKR,
    BLP,
    BLQ,
    BLR,
    CIP,
    CIQ,
    CIS,
    CJQ,
    CJR,
    CKP,
    CKR,
    CKS,
    CLS,
    DIP,
    DIQ,
    DIR,
    DIS,
    DJP,
    DJR,
    DKP,
    DKR,
    DKS,
    DLQ,
    DLR,
    DLS,
}

impl FairyRing {
    fn all() -> Vec<Self> {
        vec![
            Self::AIQ,
            Self::AJR,
            Self::AJS,
            Self::AKQ,
            Self::AKS,
            Self::ALP,
            Self::ALQ,
            Self::ALR,
            Self::ALS,
            Self::BIQ,
            Self::BIS,
            Self::BJR,
            Self::BJS,
            Self::BKP,
            Self::BKQ,
            Self::BKR,
            Self::BLP,
            Self::BLQ,
            Self::BLR,
            Self::CIP,
            Self::CIQ,
            Self::CIS,
            Self::CJQ,
            Self::CJR,
            Self::CKP,
            Self::CKR,
            Self::CKS,
            Self::CLS,
            Self::DIP,
            Self::DIQ,
            Self::DIR,
            Self::DIS,
            Self::DJP,
            Self::DJR,
            Self::DKP,
            Self::DKR,
            Self::DKS,
            Self::DLQ,
            Self::DLR,
            Self::DLS,
        ]
    }
}

impl FairyRing {
    fn details(&self) -> FairyRingDetails {
        match self {
            Self::AIQ => FairyRingDetails::from("AIQ", "Mudskipper Point"),
            Self::AJR => FairyRingDetails::from("AJR", "Slayer Cave"),
            Self::AJS => FairyRingDetails::from("AJS", "Pengin Island"),
            Self::AKQ => FairyRingDetails::from("AKQ", "Piscatoris/Woodlands Hunting Area"),
            Self::AKS => FairyRingDetails::from("AKS", "Feldip Hills"),
            Self::ALP => FairyRingDetails::from("ALP", "Lighthouse"),
            Self::ALQ => FairyRingDetails::from("ALQ", "Haunted Woods"),
            Self::ALR => FairyRingDetails::from("ALR", "Abyss"),
            Self::ALS => FairyRingDetails::from("ALS", "McGrubor's Wood"),
            Self::BIQ => FairyRingDetails::from("BIQ", "Kharidian Desert (northwest)"),
            Self::BIS => FairyRingDetails::from("BIS", "Unicorn Pen (Ardougne Zoo)"),
            Self::BJR => FairyRingDetails::from("BJR", "Grail Castle"),
            Self::BJS => FairyRingDetails::from("BJS", "Port Tyras / Zulrah's Sacred Eels"),
            Self::BKP => FairyRingDetails::from("BKP", "Chompy Frog Pond"),
            Self::BKQ => FairyRingDetails::from("BKQ", "Enchanted Valley"),
            Self::BKR => FairyRingDetails::from("BKR", "Mort Myre Swamp"),
            Self::BLP => FairyRingDetails::from("BLP", "TzHaar"),
            Self::BLQ => FairyRingDetails::from("BLQ", "Yu'buisk"),
            Self::BLR => FairyRingDetails::from("BLR", "Legends Guild"),
            Self::CIP => FairyRingDetails::from("CIP", "Miscellania"),
            Self::CIQ => FairyRingDetails::from("CIQ", "Tree Gnome Maze (south)"),
            Self::CIS => FairyRingDetails::from("CIS", "Zeah / Great Kourend"),
            Self::CJQ => FairyRingDetails::from("CJQ", "The Great Conch"),
            Self::CJR => FairyRingDetails::from("CJR", "Sinclair Mansion"),
            Self::CKP => FairyRingDetails::from("CKP", "Starflower Plane"),
            Self::CKR => FairyRingDetails::from("CKR", "Cairn Isle"),
            Self::CKS => FairyRingDetails::from("CKS", "Canifis Mushroom Patch (Slayer Tower)"),
            Self::CLS => FairyRingDetails::from("CLS", "Hazelmere"),
            Self::DIP => FairyRingDetails::from("DIP", "Abyssal Sire"),
            Self::DIQ => FairyRingDetails::from("DIQ", "Player Owned House (POH)"),
            Self::DIR => FairyRingDetails::from("DIR", "Gorak Lair"),
            Self::DIS => FairyRingDetails::from("DIS", "Wizard Tower"),
            Self::DJP => FairyRingDetails::from("DJP", "Necromancer"),
            Self::DJR => FairyRingDetails::from("DJR", "Chasm of Fire"),
            Self::DKP => FairyRingDetails::from("DKP", "Karamja (south) (near Shilo)"),
            Self::DKR => FairyRingDetails::from("DKR", "Edgeville"),
            Self::DKS => FairyRingDetails::from("DKS", "Keldagrim"),
            Self::DLQ => FairyRingDetails::from("DLQ", "Kharidian Desert (southeast)"),
            Self::DLR => FairyRingDetails::from("DLR", "Zulrah's Audience Area"),
            Self::DLS => FairyRingDetails::from("DLS", "Myreque Secret Passage"),
        }
    }
}

struct FairyRingDetails {
    code: String,
    location: String,
}

impl FairyRingDetails {
    fn from<T>(code: T, location: T) -> Self
    where
        T: ToString,
    {
        Self {
            code: code.to_string(),
            location: location.to_string(),
        }
    }

    fn to_string(&self) -> String {
        vec![p(&self.code), c2(&self.location)].join(" ")
    }
}
