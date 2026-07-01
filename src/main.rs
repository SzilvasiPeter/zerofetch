#![forbid(unsafe_code)]
use pkgmgr_info::PackageManager;
use shellver::Shell;
use sysinfo::{Motherboard, System};

fn main() {
    let os = System::name().unwrap_or_else(|| "unknown".to_string());
    let arch = System::cpu_arch();
    println!("OS: {os} {arch}");

    if let Some(mboard) = Motherboard::new() {
        let name = mboard.name().unwrap_or_else(|| "unknown".to_string());
        println!("Host: {name}");
    }

    println!("Kernel: {}", System::kernel_long_version());

    let hours = System::uptime() / 3600;
    let mins = (System::uptime() % 3600) / 60;
    println!("Uptime: {hours} hours, {mins} mins");

    if let Ok(pkgmgr) = PackageManager::detect() {
        match pkgmgr.package_count() {
            Ok(cnt) => println!("Packages: {cnt} ({})", pkgmgr.name()),
            Err(_) => println!("Packages: unknown count ({})", pkgmgr.name()),
        }
    }

    if let Ok(shell) = Shell::detect() {
        let version = shell.version().unwrap_or_default();
        println!("Shell: {} {version}", shell.name());
    }

    print_display_info();
}

fn print_display_info() {
    use edid_info::base::descriptor::monitor::DisplayDescriptor::ProductName;
    use edid_info::base::descriptors::Descriptor::Display;
    use std::fs;

    let Ok(dir) = fs::read_dir("/sys/class/drm") else {
        return;
    };
    for entry in dir.flatten() {
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        if !name.contains('-') {
            continue;
        }
        let edid_path = entry.path().join("edid");
        let bytes = match fs::read(&edid_path) {
            Ok(b) if !b.is_empty() => b,
            _ => continue,
        };
        let Ok(edid) = edid_info::edid::Edid::parse(&bytes) else {
            continue;
        };
        let base = edid.base();
        let header = base.header();

        let mfr: String = header.manufacturer().iter().collect();
        let product_name = base
            .descriptors()
            .iter()
            .find_map(|d| {
                if let Display(m) = d
                    && let ProductName(n) = m.descriptor()
                {
                    return Some(n.text().to_string());
                }
                None
            })
            .unwrap_or_default();

        let label = format!("{mfr} {product_name}").trim().to_string();
        if label.is_empty() {
            print!("Display:");
        }
        print!("Display ({label}):");

        let Some((h_active, v_active, pixel_clock, h_total, v_total)) =
            base.descriptors().iter().find_map(|d| {
                if let edid_info::base::descriptors::Descriptor::Timing(t) = d {
                    Some((
                        t.horizontal().active(),
                        t.vertical().active(),
                        t.pixel_clock_khz(),
                        t.horizontal().total(),
                        t.vertical().total(),
                    ))
                } else {
                    None
                }
            })
        else {
            println!();
            continue;
        };

        if h_active == 0 || v_active == 0 {
            println!();
            continue;
        }

        print!(" {h_active}x{v_active}");

        if let edid_info::base::basic::ScreenSize::Dimensions(s) = base.basic().screen_size()
            && s.width > 0
            && s.height > 0
        {
            let sq = f64::from(s.height)
                .mul_add(f64::from(s.height), f64::from(s.width) * f64::from(s.width));
            print!(" in {:.0}\"", sq.sqrt() / 25.4);
        }

        let h_total = u64::from(h_total);
        let v_total = u64::from(v_total);
        if h_total > 0 && v_total > 0 {
            let hz = (u64::from(pixel_clock) * 1_000) / (h_total * v_total);
            print!(", {hz} Hz");
        }

        println!();
    }
}
