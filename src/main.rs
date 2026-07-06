#![forbid(unsafe_code)]
mod display;
mod theme;

use detect_desktop_environment::DesktopEnvironment;
use pkgmgr_info::PackageManager;
use shellver::Shell;
use std::fs;
use sysinfo::{Motherboard, System};

fn main() {
    let os = System::name().unwrap_or_else(|| "unknown".to_string());
    let arch = System::cpu_arch();
    let host = Motherboard::new()
        .and_then(|board| board.name())
        .unwrap_or_default();
    let kernel = System::kernel_long_version();
    let hours = System::uptime() / 3600;
    let minutes = (System::uptime() % 3600) / 60;
    let packages = PackageManager::detect()
        .map(|m| {
            m.package_count().map_or_else(
                |_| m.name().to_string(),
                |cnt| format!("{cnt} ({})", m.name()),
            )
        })
        .unwrap_or_default();
    let shell = Shell::detect()
        .map(|s| format!("{} {}", s.name(), s.version().unwrap_or_default()))
        .unwrap_or_default();
    let display = fs::read_dir("/sys/class/drm").map_or_else(
        |_| String::new(),
        |dir| {
            dir.flatten()
                .filter_map(|entry| display::drm_entry_info(&entry))
                .collect::<Vec<_>>()
                .join("\n")
        },
    );
    let desktop_env = DesktopEnvironment::detect();
    let desktop_environment = desktop_env.map(deskenv_to_str).unwrap_or_default();
    let window_manager = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    let theme = theme::detect(desktop_env);

    println!("OS: {os} {arch}");
    println!("Host: {host}");
    println!("Kernel: {kernel}");
    println!("Uptime: {hours} hours, {minutes} minutes");
    println!("Packages: {packages}");
    println!("Shell: {shell}");
    println!("Display: {display}");
    println!("DE: {desktop_environment}");
    println!("WM: {window_manager}");
    println!("WM Theme: {}", theme.wm_theme);
    println!("Theme: {}", theme.theme);
    println!("Icon: {}", theme.icons);
    println!("Font: {}", theme.font);
    println!("Cursor: {} ({}px)", theme.cursor, theme.cursor_size);
}

// TODO: Remove this and call the `to_string()` method directly once the https://github.com/demurgos/detect-desktop-environment/pull/19 PR is merged
const fn deskenv_to_str(de: DesktopEnvironment) -> &'static str {
    match de {
        DesktopEnvironment::Cinnamon => "Cinnamon",
        DesktopEnvironment::Cosmic => "COSMIC",
        DesktopEnvironment::CosmicEpoch => "COSMIC Epoch",
        DesktopEnvironment::Dde => "DDE",
        DesktopEnvironment::Ede => "EDE",
        DesktopEnvironment::Endless => "Endless",
        DesktopEnvironment::Enlightenment => "Enlightenment",
        DesktopEnvironment::Gnome => "GNOME",
        DesktopEnvironment::Hyprland => "Hyprland",
        DesktopEnvironment::Kde => "KDE Plasma",
        DesktopEnvironment::Lxde => "LXDE",
        DesktopEnvironment::Lxqt => "LXQt",
        DesktopEnvironment::MacOs => "macOS",
        DesktopEnvironment::Mate => "MATE",
        DesktopEnvironment::Old => "Old",
        DesktopEnvironment::Pantheon => "Pantheon",
        DesktopEnvironment::Razor => "Razor",
        DesktopEnvironment::Rox => "ROX",
        DesktopEnvironment::Sway => "Sway",
        DesktopEnvironment::Tde => "TDE",
        DesktopEnvironment::Unity => "Unity",
        DesktopEnvironment::Windows => "Windows",
        DesktopEnvironment::Xfce => "Xfce",
        _ => "",
    }
}
