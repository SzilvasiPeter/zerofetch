use std::{fs, path::Path};

// The `0x0300` is the Display/VGA class code:
// 0x3 - Display Controller, 0x0 - VGA Compatible Controller, 0x0 - VGA Controller
// See https://wiki.osdev.org/PCI for more information.
const GPU_CLASS_CODE: &str = "0x0300";
const PCI_DB_PATHS: &[&str] = &[
    "/usr/share/hwdata/pci.ids",
    "/usr/share/misc/pci.ids",
    "/var/lib/pciutils/pci.ids",
];

pub fn fetch() -> String {
    let Some((vendor_id, device_id)) = find_gpu_device_ids("/sys/bus/pci/devices") else {
        return String::new();
    };
    let Some(db_path) = PCI_DB_PATHS.iter().find(|p| Path::new(p).exists()) else {
        return String::new();
    };

    let db_content = fs::read_to_string(db_path).unwrap_or_default();
    let (vendor, device) = parse_pci_ids(&vendor_id, &device_id, &db_content);
    format!("{vendor} {device}")
}

fn find_gpu_device_ids(sysfs_path: &str) -> Option<(String, String)> {
    fs::read_dir(sysfs_path)
        .into_iter()
        .flatten()
        .filter_map(std::result::Result::ok)
        .find(|entry| {
            fs::read_to_string(entry.path().join("class")).is_ok_and(|c| c.contains(GPU_CLASS_CODE))
        })
        .and_then(|entry| {
            let vendor = fs::read_to_string(entry.path().join("vendor"))
                .ok()?
                .trim()
                .strip_prefix("0x")?
                .to_string();
            let device = fs::read_to_string(entry.path().join("device"))
                .ok()?
                .trim()
                .strip_prefix("0x")?
                .to_string();
            Some((vendor, device))
        })
}

fn parse_pci_ids(db_content: &str, vendor_id: &str, device_id: &str) -> (String, String) {
    let mut lines = db_content.lines().skip_while(|l| !l.starts_with(vendor_id));
    let vendor = lines
        .next()
        .map_or("", |l| l.strip_prefix(vendor_id).unwrap_or(l).trim())
        .to_string();
    let device = lines
        .take_while(|l| l.starts_with('\t') || l.is_empty() || l.starts_with('#'))
        .find(|l| l.strip_prefix('\t').unwrap_or(l).starts_with(device_id))
        .map_or(String::new(), |l| {
            l.strip_prefix('\t')
                .unwrap_or(l)
                .strip_prefix(device_id)
                .unwrap_or(l)
                .trim()
                .to_string()
        });

    (vendor, device)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pci_ids_finds_vendor_and_device() {
        let db = "\
10de  NVIDIA Corporation
\t11c0  GeForce GTX 660
\t1180  GeForce GTX 670
\t11c2  GeForce GTX 660 Ti

";
        let (vendor, device) = parse_pci_ids(db, "10de", "11c2");
        assert_eq!(vendor, "NVIDIA Corporation");
        assert_eq!(device, "GeForce GTX 660 Ti");
    }

    #[test]
    fn parse_pci_ids_vendor_not_found() {
        let db = "\
10de  NVIDIA Corporation
\t11c0  GeForce GTX 660
";
        let (vendor, device) = parse_pci_ids(db, "ffff", "11c0");
        assert_eq!(vendor, "");
        assert_eq!(device, "");
    }

    #[test]
    fn parse_pci_ids_device_not_found() {
        let db = "\
10de  NVIDIA Corporation
\t11c0  GeForce GTX 660
";
        let (vendor, device) = parse_pci_ids(db, "10de", "ffff");
        assert_eq!(vendor, "NVIDIA Corporation");
        assert_eq!(device, "");
    }

    #[test]
    fn parse_pci_ids_skips_comment_lines() {
        let db = "\
10de  NVIDIA Corporation
# Comment line
\t11c0  GeForce GTX 660
";
        let (vendor, device) = parse_pci_ids(db, "10de", "11c0");
        assert_eq!(vendor, "NVIDIA Corporation");
        assert_eq!(device, "GeForce GTX 660");
    }

    #[test]
    fn parse_pci_ids_empty_db() {
        let (vendor, device) = parse_pci_ids("", "10de", "11c0");
        assert_eq!(vendor, "");
        assert_eq!(device, "");
    }
}
