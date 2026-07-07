use std::{fs, path::Path};

pub fn fetch() -> String {
    let Some((vendor_id, device_id)) = fs::read_dir("/sys/bus/pci/devices")
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .find(|entry| {
            // The `0x0300` is the Display/VGA class code:
            // 0x3 - Display Controller
            // 0x0 - VGA Compatible Controller
            // 0x0 - VGA Controller
            // see https://wiki.osdev.org/PCI for more information
            fs::read_to_string(entry.path().join("class")).is_ok_and(|c| c.contains("0x0300"))
        })
        .and_then(|entry| {
            let v = fs::read_to_string(entry.path().join("vendor"))
                .ok()?
                .trim()
                .strip_prefix("0x")?
                .to_string();
            let d = fs::read_to_string(entry.path().join("device"))
                .ok()?
                .trim()
                .strip_prefix("0x")?
                .to_string();
            Some((v, d))
        })
    else {
        return String::new();
    };

    let db_paths = [
        "/usr/share/hwdata/pci.ids",
        "/usr/share/misc/pci.ids",
        "/var/lib/pciutils/pci.ids",
    ];
    let Some(db_path) = db_paths.iter().find(|p| Path::new(p).exists()) else {
        return String::new();
    };

    let file_content = fs::read_to_string(db_path).unwrap_or_default();
    let mut lines = file_content
        .lines()
        .skip_while(|l| !l.starts_with(&vendor_id));
    let vendor = lines
        .next()
        .map_or("", |l| l.strip_prefix(&vendor_id).unwrap_or(l).trim());
    let device = lines
        .take_while(|l| l.starts_with('\t') || l.is_empty() || l.starts_with('#'))
        .find(|l| l.strip_prefix('\t').unwrap_or(l).starts_with(&device_id))
        .map_or("", |l| {
            l.strip_prefix('\t')
                .unwrap_or(l)
                .strip_prefix(&device_id)
                .unwrap_or(l)
                .trim()
        });

    format!("{vendor} {device}")
}
