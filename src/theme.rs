use crate::deskenv::DesktopEnv;
use std::process::Command;

#[derive(Default)]
pub struct Info {
    pub wm_theme: String,
    pub theme: String,
    pub icons: String,
    pub font: String,
    pub cursor: String,
    pub cursor_size: String,
}

pub trait ThemeQuerier {
    fn wm_theme(&self) -> String {
        String::new()
    }

    fn theme(&self) -> String {
        String::new()
    }

    fn icons(&self) -> String {
        String::new()
    }

    fn font(&self) -> String {
        String::new()
    }

    fn cursor(&self) -> String {
        String::new()
    }

    fn cursor_size(&self) -> String {
        String::new()
    }

    fn collect(&self) -> Info {
        Info {
            wm_theme: self.wm_theme(),
            theme: self.theme(),
            icons: self.icons(),
            font: self.font(),
            cursor: self.cursor(),
            cursor_size: self.cursor_size(),
        }
    }
}

struct Sway;

impl ThemeQuerier for Sway {
    fn theme(&self) -> String {
        gsettings_get("org.gnome.desktop.interface", "gtk-theme")
    }

    fn icons(&self) -> String {
        gsettings_get("org.gnome.desktop.interface", "icon-theme")
    }

    fn font(&self) -> String {
        gsettings_get("org.gnome.desktop.interface", "font-name")
    }

    fn cursor(&self) -> String {
        gsettings_get("org.gnome.desktop.interface", "cursor-theme")
    }

    fn cursor_size(&self) -> String {
        gsettings_get("org.gnome.desktop.interface", "cursor-size")
    }
}

fn gsettings_get(schema: &str, key: &str) -> String {
    Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .map(|out| {
            String::from_utf8_lossy(&out.stdout)
                .trim()
                .trim_matches('\'')
                .to_string()
        })
        .unwrap_or_default()
}

struct Xfce;

impl ThemeQuerier for Xfce {
    fn wm_theme(&self) -> String {
        xfce_query("xfwm4", "/general/theme")
    }

    fn theme(&self) -> String {
        xfce_query("xsettings", "/Net/ThemeName")
    }

    fn icons(&self) -> String {
        xfce_query("xsettings", "/Net/IconThemeName")
    }

    fn font(&self) -> String {
        xfce_query("xsettings", "/Gtk/FontName")
    }

    fn cursor(&self) -> String {
        xfce_query("xsettings", "/Gtk/CursorThemeName")
    }

    fn cursor_size(&self) -> String {
        xfce_query("xsettings", "/Gtk/CursorThemeSize")
    }
}

fn xfce_query(channel: &str, property: &str) -> String {
    Command::new("xfconf-query")
        .args(["-c", channel, "-p", property])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_default()
}

pub fn fetch(de: &DesktopEnv) -> Info {
    if matches!(de, DesktopEnv::Other(_)) {
        return Info::default();
    }
    match de {
        DesktopEnv::Xfce => {
            let provider = Xfce;
            provider.collect()
        }
        DesktopEnv::Sway => {
            let provider = Sway;
            provider.collect()
        }
        // TODO: Replace `_` with `DesktopEnv::Other(_)` once the exhaustive match in place
        _ => Info::default(),
    }
}
