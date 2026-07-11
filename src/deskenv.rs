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

impl DesktopEnv {
    pub fn from_raw(raw: &str) -> Self {
        raw.split(':')
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .find_map(|p| {
                let variant = match p.to_lowercase().as_str() {
                    "budgie" => Self::Budgie,
                    "cinnamon" | "x-cinnamon" => Self::Cinnamon,
                    "cosmic" => Self::Cosmic,
                    "dde" | "deepin" => Self::Dde,
                    "ede" => Self::Ede,
                    "endless" => Self::Endless,
                    "enlightenment" | "e" => Self::Enlightenment,
                    "gnome" => Self::Gnome,
                    "hyprland" => Self::Hyprland,
                    "kde" | "plasmax11" => Self::Kde,
                    "lxde" => Self::Lxde,
                    "lxqt" => Self::Lxqt,
                    "mate" => Self::Mate,
                    "pantheon" => Self::Pantheon,
                    "razor" | "razor-qt" => Self::Razor,
                    "rox" => Self::Rox,
                    "sway" => Self::Sway,
                    "tde" | "trinity" => Self::Tde,
                    "unity" => Self::Unity,
                    "xfce" | "xubuntu" => Self::Xfce,
                    _ => return None,
                };
                Some(variant)
            })
            .unwrap_or_else(|| Self::Other(raw.to_string()))
    }
}

pub fn detect() -> DesktopEnv {
    let raw = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    DesktopEnv::from_raw(&raw)
}
