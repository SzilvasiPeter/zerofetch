use edid_info::base::basic::ScreenSize;
use edid_info::base::descriptor::monitor::DisplayDescriptor::ProductName;
use edid_info::base::descriptors::Descriptor::{Display, Timing};
use edid_info::edid::Edid;
use std::fs;

pub fn fetch() -> Option<String> {
    let dir = fs::read_dir("/sys/class/drm").ok()?;
    for entry in dir.flatten() {
        let path = entry.path();
        let name = path.file_name()?.to_str()?;
        if !name.contains('-') {
            continue;
        }

        if let Ok(enabled_state) = fs::read_to_string(path.join("enabled"))
            && enabled_state.trim() == "enabled"
            && let Ok(bytes) = fs::read(path.join("edid"))
        {
            return process_drm(&bytes);
        }
    }

    None
}

fn process_drm(bytes: &[u8]) -> Option<String> {
    if bytes.is_empty() {
        return None;
    }

    let edid = Edid::parse(bytes).ok()?;
    Some(edid_info(&edid))
}

fn edid_info(edid: &Edid) -> String {
    let base = edid.base();
    let manufacturer: String = base.header().manufacturer().iter().collect();
    let product = base
        .descriptors()
        .iter()
        .filter_map(|d| if let Display(m) = d { Some(m) } else { None })
        .find_map(|m| match m.descriptor() {
            ProductName(n) => Some(n),
            _ => None,
        })
        .map(|n| format!(" {}", n.text()))
        .unwrap_or_default();

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
    let size = match base.basic().screen_size() {
        ScreenSize::Dimensions(s) => {
            let height = f64::from(s.height);
            let width = f64::from(s.width);
            let diagonal = height.mul_add(height, width * width);
            format!(" in {:.0}\"", diagonal.sqrt() / 25.4)
        }
        ScreenSize::Aspect(ar) => {
            let height = f64::from(ar.height());
            let width = f64::from(ar.width());
            let diagonal = height.mul_add(height, width * width);
            format!(" in {:.0}\"", diagonal.sqrt() / 25.4)
        }
        ScreenSize::Undefined => String::new(),
    };

    let h_total = u64::from(h_total);
    let v_total = u64::from(v_total);
    let hz = if h_total > 0 && v_total > 0 {
        format!("{} Hz", (u64::from(pixel) * 1_000) / (h_total * v_total))
    } else {
        String::new()
    };

    format!("{manufacturer}{product}, {h_active}x{v_active}{size}, {hz}")
}

#[cfg(test)]
mod tests {
    use super::{edid_info, process_drm};
    use edid_info::common::BLOCK_LEN;
    use edid_info::edid::Edid;

    fn base_block() -> [u8; BLOCK_LEN] {
        let mut raw = [0u8; BLOCK_LEN];
        raw[0..8].copy_from_slice(&[0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]);
        raw[8..10].copy_from_slice(&[0x10, 0xAC]);
        raw[10..12].copy_from_slice(&[0x01, 0x00]);
        raw[18] = 1;
        raw[19] = 4;
        raw[21] = 52;
        raw[22] = 32;
        raw
    }

    fn set_timing(raw: &mut [u8; BLOCK_LEN], offset: usize) {
        let pixel_clock = 2518u16;
        raw[offset..offset + 2].copy_from_slice(&pixel_clock.to_le_bytes());
        raw[offset + 2] = 0x80;
        raw[offset + 3] = 0xA0;
        raw[offset + 4] = 0x20;
        raw[offset + 5] = 0xE0;
        raw[offset + 6] = 0x2D;
        raw[offset + 7] = 0x10;
        raw[offset + 8] = 0x10;
        raw[offset + 9] = 0x60;
        raw[offset + 10] = 0xA2;
    }

    fn set_timing_zero(raw: &mut [u8; BLOCK_LEN], offset: usize) {
        raw[offset] = 1;
    }

    fn set_display_monitor(raw: &mut [u8; BLOCK_LEN], offset: usize, tag: u8) {
        raw[offset] = 0x00;
        raw[offset + 1] = 0x00;
        raw[offset + 3] = tag;
    }

    fn edid_output(raw: &[u8; BLOCK_LEN]) -> String {
        let edid = Edid::parse(raw).unwrap();
        edid_info(&edid)
    }

    #[test]
    fn no_display_descriptor() {
        let mut raw = base_block();
        for offset in [54usize, 72, 90, 108] {
            set_timing(&mut raw, offset);
        }
        assert_eq!(edid_output(&raw), "DEL, 640x480 in 24\", 59 Hz");
    }

    #[test]
    fn display_without_product_name() {
        let mut raw = base_block();
        set_timing(&mut raw, 54);
        raw[72] = 0x00;
        raw[73] = 0x00;
        raw[75] = 0xFF;
        raw[77..83].copy_from_slice(b"SN1234");
        set_timing(&mut raw, 90);
        set_timing(&mut raw, 108);
        assert_eq!(edid_output(&raw), "DEL, 640x480 in 24\", 59 Hz");
    }

    #[test]
    fn display_with_empty_product_name() {
        let mut raw = base_block();
        set_timing(&mut raw, 54);
        raw[72] = 0x00;
        raw[73] = 0x00;
        raw[75] = 0xFC;
        set_timing(&mut raw, 90);
        set_timing(&mut raw, 108);
        assert_eq!(edid_output(&raw), "DEL , 640x480 in 24\", 59 Hz");
    }

    #[test]
    fn display_with_product_name() {
        let mut raw = base_block();
        set_timing(&mut raw, 54);
        raw[72] = 0x00;
        raw[73] = 0x00;
        raw[75] = 0xFC;
        raw[77..87].copy_from_slice(b"My Monitor");
        set_timing(&mut raw, 90);
        set_timing(&mut raw, 108);
        assert_eq!(edid_output(&raw), "DEL My Monitor, 640x480 in 24\", 59 Hz");
    }

    #[test]
    fn no_timing_descriptor() {
        let mut raw = base_block();
        for offset in [54, 72, 90, 108] {
            set_display_monitor(&mut raw, offset, 0x10);
        }
        assert_eq!(edid_output(&raw), "DEL, 0x0 in 24\", ");
    }

    #[test]
    fn timing_zero_h_total_v_total() {
        let mut raw = base_block();
        set_timing_zero(&mut raw, 54);
        for offset in [72, 90, 108] {
            set_display_monitor(&mut raw, offset, 0x10);
        }
        assert_eq!(edid_output(&raw), "DEL, 0x0 in 24\", ");
    }

    #[test]
    fn timing_nonzero_h_total_v_total() {
        let mut raw = base_block();
        set_timing(&mut raw, 54);
        for offset in [72, 90, 108] {
            set_display_monitor(&mut raw, offset, 0x10);
        }
        assert_eq!(edid_output(&raw), "DEL, 640x480 in 24\", 59 Hz");
    }

    #[test]
    fn screen_size_undefined() {
        let mut raw = base_block();
        raw[21] = 0;
        raw[22] = 0;
        set_timing(&mut raw, 54);
        for offset in [72, 90, 108] {
            set_display_monitor(&mut raw, offset, 0x10);
        }
        assert_eq!(edid_output(&raw), "DEL, 640x480, 59 Hz");
    }

    #[test]
    fn screen_size_aspect() {
        let mut raw = base_block();
        raw[21] = 16;
        raw[22] = 0;
        set_timing(&mut raw, 54);
        for offset in [72, 90, 108] {
            set_display_monitor(&mut raw, offset, 0x10);
        }
        assert_eq!(edid_output(&raw), "DEL, 640x480 in 6\", 59 Hz");
    }

    #[test]
    fn screen_size_dimensions() {
        let mut raw = base_block();
        set_timing(&mut raw, 54);
        for offset in [72, 90, 108] {
            set_display_monitor(&mut raw, offset, 0x10);
        }
        assert_eq!(edid_output(&raw), "DEL, 640x480 in 24\", 59 Hz");
    }

    #[test]
    fn process_drm_empty_bytes() {
        assert_eq!(process_drm(&[]), None);
    }

    #[test]
    fn process_drm_invalid_bytes() {
        assert_eq!(process_drm(&[0xDE, 0xAD, 0xBE, 0xEF]), None);
    }

    #[test]
    fn process_drm_valid_bytes() {
        let mut raw = base_block();
        set_timing(&mut raw, 54);
        for offset in [72, 90, 108] {
            set_display_monitor(&mut raw, offset, 0x10);
        }
        assert_eq!(
            process_drm(&raw),
            Some("DEL, 640x480 in 24\", 59 Hz".into())
        );
    }
}
