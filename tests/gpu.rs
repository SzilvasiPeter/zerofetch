use std::fs;
use tempfile::tempdir;
use zerofetch::gpu::find_gpu_device_ids;

#[test]
fn find_gpu_device_ids_path_not_exist() {
    assert_eq!(find_gpu_device_ids("/nonexistent/path"), None);
}

#[test]
fn find_gpu_device_ids_no_class_file() {
    let dir = tempdir().unwrap();
    let entry = dir.path().join("0000:01:00.0");
    fs::create_dir_all(&entry).unwrap();
    assert_eq!(find_gpu_device_ids(dir.path().to_str().unwrap()), None);
}

#[test]
fn find_gpu_device_ids_class_file_no_gpu_code() {
    let dir = tempdir().unwrap();
    let entry = dir.path().join("0000:01:00.0");
    fs::create_dir_all(&entry).unwrap();
    fs::write(entry.join("class"), "0x0200").unwrap();
    assert_eq!(find_gpu_device_ids(dir.path().to_str().unwrap()), None);
}

#[test]
fn find_gpu_device_ids_valid_class_no_vendor() {
    let dir = tempdir().unwrap();
    let entry = dir.path().join("0000:01:00.0");
    fs::create_dir_all(&entry).unwrap();
    fs::write(entry.join("class"), "0x0300").unwrap();
    assert_eq!(find_gpu_device_ids(dir.path().to_str().unwrap()), None);
}

#[test]
fn find_gpu_device_ids_valid_class_vendor_no_device() {
    let dir = tempdir().unwrap();
    let entry = dir.path().join("0000:01:00.0");
    fs::create_dir_all(&entry).unwrap();
    fs::write(entry.join("class"), "0x0300").unwrap();
    fs::write(entry.join("vendor"), "0x10de").unwrap();
    assert_eq!(find_gpu_device_ids(dir.path().to_str().unwrap()), None);
}

#[test]
fn find_gpu_device_ids_all_files_present() {
    let dir = tempdir().unwrap();
    let entry = dir.path().join("0000:01:00.0");
    fs::create_dir_all(&entry).unwrap();
    fs::write(entry.join("class"), "0x0300").unwrap();
    fs::write(entry.join("vendor"), "0x10de").unwrap();
    fs::write(entry.join("device"), "0x11c2").unwrap();
    assert_eq!(
        find_gpu_device_ids(dir.path().to_str().unwrap()),
        Some(("10de".to_string(), "11c2".to_string()))
    );
}
