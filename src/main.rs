#![forbid(unsafe_code)]
use edid_info::base::basic::ScreenSize::Dimensions;
use edid_info::base::descriptor::monitor::DisplayDescriptor::ProductName;
use edid_info::base::descriptors::Descriptor::{Display, Timing};
use edid_info::edid::Edid;
use pkgmgr_info::PackageManager;
use shellver::Shell;
use std::fs;
use sysinfo::{Motherboard, System};

fn main() {
    let os = System::name().unwrap_or_else(|| "unknown".to_string());
    let arch = System::cpu_arch();
    println!("OS: {os} {arch}");

    if let Some(board) = Motherboard::new() {
        let board_name = board.name().unwrap_or_else(|| String::from("unknown"));
        println!("Host: {board_name}");
    }

    println!("Kernel: {}", System::kernel_long_version());

    let hours = System::uptime() / 3600;
    let minutes = (System::uptime() % 3600) / 60;
    println!("Uptime: {hours} hours, {minutes} mins");

    if let Ok(manager) = PackageManager::detect() {
        match manager.package_count() {
            Ok(cnt) => println!("Packages: {cnt} ({})", manager.name()),
            Err(_) => println!("Packages: unknown count ({})", manager.name()),
        }
    }

    if let Ok(shell) = Shell::detect() {
        let version = shell.version().unwrap_or_default();
        println!("Shell: {} {version}", shell.name());
    }

    println!("{}", display_info());
}

fn display_info() -> String {
    let Ok(dir) = fs::read_dir("/sys/class/drm") else {
        return String::new();
    };
    dir.flatten()
        .filter_map(|entry| drm_entry_info(&entry))
        .collect::<Vec<_>>()
        .join("\n")
}

fn drm_entry_info(entry: &std::fs::DirEntry) -> Option<String> {
    let Ok(n) = entry.file_name().into_string() else {
        return None;
    };
    if !n.contains('-') {
        return None;
    }
    let Ok(bytes) = fs::read(entry.path().join("edid")) else {
        return None;
    };
    if bytes.is_empty() {
        return None;
    }
    let Ok(edid) = Edid::parse(&bytes) else {
        return None;
    };
    Some(format_edid_info(&edid))
}

fn format_edid_info(edid: &Edid) -> String {
    let base = edid.base();
    let manufacturer: String = base.header().manufacturer().iter().collect();
    let product = base
        .descriptors()
        .iter()
        .filter_map(|d| if let Display(m) = d { Some(m) } else { None })
        .filter_map(|m| match m.descriptor() {
            ProductName(n) => Some(n),
            _ => None,
        })
        .map(|n| n.text().to_string())
        .next()
        .unwrap_or_default();
    let label = format!("{manufacturer} {product}").trim().to_string();
    let prefix = if label.is_empty() {
        "Display:".to_string()
    } else {
        format!("Display ({label}):")
    };

    let (h_active, v_active, pixel, h_total, v_total) = base
        .descriptors()
        .iter()
        .filter_map(|d| if let Timing(t) = d { Some(t) } else { None })
        .map(|t| {
            (
                t.horizontal().active(),
                t.vertical().active(),
                t.pixel_clock_khz(),
                t.horizontal().total(),
                t.vertical().total(),
            )
        })
        .next()
        .unwrap_or_default();

    let h_total = u64::from(h_total);
    let v_total = u64::from(v_total);
    let hz_info = if h_total > 0 && v_total > 0 {
        let hz = (u64::from(pixel) * 1_000) / (h_total * v_total);
        format!(", {hz} Hz")
    } else {
        String::new()
    };

    let size = format_display_size(base);
    format!("{prefix} {h_active}x{v_active}{size}{hz_info}")
}

fn format_display_size(base: edid_info::base::Base) -> String {
    let Dimensions(s) = base.basic().screen_size() else {
        return String::new();
    };
    if s.width == 0 || s.height == 0 {
        return String::new();
    }
    let height = f64::from(s.height);
    let width = f64::from(s.width);
    let diagonal = height.mul_add(height, width * width);
    format!(" in {:.0}\"", diagonal.sqrt() / 25.4)
}
