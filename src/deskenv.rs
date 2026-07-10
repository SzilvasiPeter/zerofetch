use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopEnv {
    Budgie,
    Cinnamon,
    Cosmic,
    Dde,
    Ede,
    Endless,
    Enlightenment,
    Gnome,
    Hyprland,
    Kde,
    Lxde,
    Lxqt,
    Mate,
    Pantheon,
    Razor,
    Rox,
    Sway,
    Tde,
    Unity,
    Xfce,
    Other(String),
}

impl fmt::Display for DesktopEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Budgie => write!(f, "Budgie"),
            Self::Cinnamon => write!(f, "Cinnamon"),
            Self::Cosmic => write!(f, "COSMIC"),
            Self::Dde => write!(f, "DDE"),
            Self::Ede => write!(f, "EDE"),
            Self::Endless => write!(f, "Endless"),
            Self::Enlightenment => write!(f, "Enlightenment"),
            Self::Gnome => write!(f, "GNOME"),
            Self::Hyprland => write!(f, "Hyprland"),
            Self::Kde => write!(f, "KDE Plasma"),
            Self::Lxde => write!(f, "LXDE"),
            Self::Lxqt => write!(f, "LXQt"),
            Self::Mate => write!(f, "MATE"),
            Self::Pantheon => write!(f, "Pantheon"),
            Self::Razor => write!(f, "Razor"),
            Self::Rox => write!(f, "ROX"),
            Self::Sway => write!(f, "Sway"),
            Self::Tde => write!(f, "TDE"),
            Self::Unity => write!(f, "Unity"),
            Self::Xfce => write!(f, "Xfce"),
            Self::Other(name) => write!(f, "{name}"),
        }
    }
}

pub fn detect() -> DesktopEnv {
    let raw = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    DesktopEnv::from_raw(&raw)
}

impl DesktopEnv {
    pub fn from_raw(raw: &str) -> Self {
        for part in raw.split(':') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            match part.to_lowercase().as_str() {
                "budgie" => return Self::Budgie,
                "cinnamon" | "x-cinnamon" => return Self::Cinnamon,
                "cosmic" => return Self::Cosmic,
                "dde" | "deepin" => return Self::Dde,
                "ede" => return Self::Ede,
                "endless" => return Self::Endless,
                "enlightenment" | "e" => return Self::Enlightenment,
                "gnome" => return Self::Gnome,
                "hyprland" => return Self::Hyprland,
                "kde" | "plasmax11" => return Self::Kde,
                "lxde" => return Self::Lxde,
                "lxqt" => return Self::Lxqt,
                "mate" => return Self::Mate,
                "pantheon" => return Self::Pantheon,
                "razor" | "razor-qt" => return Self::Razor,
                "rox" => return Self::Rox,
                "sway" => return Self::Sway,
                "tde" | "trinity" => return Self::Tde,
                "unity" => return Self::Unity,
                "xfce" | "xubuntu" => return Self::Xfce,
                _ => {}
            }
        }
        Self::Other(raw.to_string())
    }
}
