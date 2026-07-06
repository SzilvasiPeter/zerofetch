use edid_info::base::basic::ScreenSize::Dimensions;
use edid_info::base::descriptor::monitor::DisplayDescriptor::ProductName;
use edid_info::base::descriptors::Descriptor::{Display, Timing};
use edid_info::edid::Edid;
use std::fs;

pub fn drm_entry_info(entry: &std::fs::DirEntry) -> Option<String> {
    let name = entry.file_name().into_string().ok()?;
    if !name.contains('-') {
        return None;
    }

    let bytes = fs::read(entry.path().join("edid")).ok()?;
    if bytes.is_empty() {
        return None;
    }

    let edid = Edid::parse(&bytes).ok()?;
    Some(edid_info(&edid))
}

pub fn edid_info(edid: &Edid) -> String {
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
    let size = if let Dimensions(s) = base.basic().screen_size()
        && s.width != 0
        && s.height != 0
    {
        let height = f64::from(s.height);
        let width = f64::from(s.width);
        let diagonal = height.mul_add(height, width * width);
        format!(" in {:.0}\"", diagonal.sqrt() / 25.4)
    } else {
        String::new()
    };

    let h_total = u64::from(h_total);
    let v_total = u64::from(v_total);
    let hz = if h_total > 0 && v_total > 0 {
        format!("{} Hz", (u64::from(pixel) * 1_000) / (h_total * v_total))
    } else {
        String::new()
    };

    format!("{manufacturer} {product}, {h_active}x{v_active}{size}, {hz}")
}
