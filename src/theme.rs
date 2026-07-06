use std::process::Command;

use detect_desktop_environment::DesktopEnvironment;

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

pub fn detect(de: Option<DesktopEnvironment>) -> Info {
    let Some(de) = de else { return Info::default() };
    // TODO: Extend with other desktop environment
    match de {
        DesktopEnvironment::Xfce => {
            let provider = Xfce;
            provider.collect()
        }
        _ => Info::default(),
    }
}
