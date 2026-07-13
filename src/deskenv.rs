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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let cases = [
            (DesktopEnv::Budgie, "Budgie"),
            (DesktopEnv::Cinnamon, "Cinnamon"),
            (DesktopEnv::Cosmic, "COSMIC"),
            (DesktopEnv::Dde, "DDE"),
            (DesktopEnv::Ede, "EDE"),
            (DesktopEnv::Endless, "Endless"),
            (DesktopEnv::Enlightenment, "Enlightenment"),
            (DesktopEnv::Gnome, "GNOME"),
            (DesktopEnv::Hyprland, "Hyprland"),
            (DesktopEnv::Kde, "KDE Plasma"),
            (DesktopEnv::Lxde, "LXDE"),
            (DesktopEnv::Lxqt, "LXQt"),
            (DesktopEnv::Mate, "MATE"),
            (DesktopEnv::Pantheon, "Pantheon"),
            (DesktopEnv::Razor, "Razor"),
            (DesktopEnv::Rox, "ROX"),
            (DesktopEnv::Sway, "Sway"),
            (DesktopEnv::Tde, "TDE"),
            (DesktopEnv::Unity, "Unity"),
            (DesktopEnv::Xfce, "Xfce"),
            (DesktopEnv::Other("CustomDE".into()), "CustomDE"),
            (DesktopEnv::Other(String::new()), ""),
        ];
        for (env, expected) in cases {
            assert_eq!(env.to_string(), expected);
        }
    }

    #[test]
    fn from_raw_primary_names() {
        let cases = [
            ("budgie", DesktopEnv::Budgie),
            ("cinnamon", DesktopEnv::Cinnamon),
            ("cosmic", DesktopEnv::Cosmic),
            ("dde", DesktopEnv::Dde),
            ("ede", DesktopEnv::Ede),
            ("endless", DesktopEnv::Endless),
            ("enlightenment", DesktopEnv::Enlightenment),
            ("gnome", DesktopEnv::Gnome),
            ("hyprland", DesktopEnv::Hyprland),
            ("kde", DesktopEnv::Kde),
            ("lxde", DesktopEnv::Lxde),
            ("lxqt", DesktopEnv::Lxqt),
            ("mate", DesktopEnv::Mate),
            ("pantheon", DesktopEnv::Pantheon),
            ("razor", DesktopEnv::Razor),
            ("rox", DesktopEnv::Rox),
            ("sway", DesktopEnv::Sway),
            ("tde", DesktopEnv::Tde),
            ("unity", DesktopEnv::Unity),
            ("xfce", DesktopEnv::Xfce),
        ];
        for (raw, expected) in cases {
            assert_eq!(DesktopEnv::from_raw(raw), expected, "raw: {raw}");
        }
    }

    #[test]
    fn from_raw_aliases() {
        let cases = [
            ("x-cinnamon", DesktopEnv::Cinnamon),
            ("deepin", DesktopEnv::Dde),
            ("plasmax11", DesktopEnv::Kde),
            ("razor-qt", DesktopEnv::Razor),
            ("trinity", DesktopEnv::Tde),
            ("xubuntu", DesktopEnv::Xfce),
            ("e", DesktopEnv::Enlightenment),
        ];
        for (raw, expected) in cases {
            assert_eq!(DesktopEnv::from_raw(raw), expected, "raw: {raw}");
        }
    }

    #[test]
    fn from_raw_case_insensitive() {
        let cases = [
            ("GNOME", DesktopEnv::Gnome),
            ("Kde", DesktopEnv::Kde),
            ("SWAY", DesktopEnv::Sway),
            ("Xfce", DesktopEnv::Xfce),
        ];
        for (raw, expected) in cases {
            assert_eq!(DesktopEnv::from_raw(raw), expected, "raw: {raw}");
        }
    }

    #[test]
    fn from_raw_colon_separated() {
        let cases = [
            ("X-GNOME:KDE", DesktopEnv::Kde),
            ("budgie:gnome", DesktopEnv::Budgie),
            ("bad:unknown:xfce", DesktopEnv::Xfce),
            (":sway:", DesktopEnv::Sway),
        ];
        for (raw, expected) in cases {
            assert_eq!(DesktopEnv::from_raw(raw), expected, "raw: {raw}");
        }
    }

    #[test]
    fn from_raw_whitespace_trimmed() {
        let cases = [
            ("  budgie  ", DesktopEnv::Budgie),
            ("  gnome : kde", DesktopEnv::Gnome),
        ];
        for (raw, expected) in cases {
            assert_eq!(DesktopEnv::from_raw(raw), expected, "raw: {raw}");
        }
    }

    #[test]
    fn from_raw_unknown_falls_back_to_other() {
        let cases = ["unknown", "random-de", "fluxbox", ""];
        for raw in cases {
            assert_eq!(
                DesktopEnv::from_raw(raw),
                DesktopEnv::Other(raw.to_string()),
                "raw: {raw}"
            );
        }
    }
}
