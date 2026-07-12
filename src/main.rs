#![forbid(unsafe_code)]
mod deskenv;
mod display;
mod gpu;
mod theme;

use pkgmgr_info::PackageManager;
use shellver::Shell;
use sysinfo::{Motherboard, Networks, System};

const G: &str = "\x1b[32m";
const R: &str = "\x1b[0m";

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
    let display = display::fetch().unwrap_or_default();
    let desktop_env = deskenv::detect();
    let desktop_environment = desktop_env.to_string();
    let window_manager = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    let theme = theme::fetch(&desktop_env);
    let mut sys = System::new_all();
    sys.refresh_memory();
    let terminal = sys
        .process(sysinfo::Pid::from_u32(std::process::id()))
        .and_then(|p| sys.process(p.parent()?))
        .and_then(|p| sys.process(p.parent()?))
        .map(|p| p.name().to_string_lossy().into_owned())
        .unwrap_or_default();
    let cpu = sys.cpus().iter().next().map(sysinfo::Cpu::brand);
    let cpu = cpu.unwrap_or_default();
    let gpu = gpu::fetch();
    let (total_mem, used_mem, percentage_mem) = bytes_to_gib(sys.total_memory(), sys.used_memory());
    let (total_swap, used_swap, percentage_swap) = bytes_to_gib(sys.total_swap(), sys.used_swap());
    let disks_info: Vec<(String, f64, f64, u64, String)> =
        sysinfo::Disks::new_with_refreshed_list()
            .iter()
            .map(|disk| {
                let mount = disk.mount_point().to_string_lossy().into_owned();
                let total = disk.total_space();
                let used = total - disk.available_space();
                let (total, used, percentage) = bytes_to_gib(total, used);
                let fs = disk.file_system().to_string_lossy().into_owned();
                (mount, used, total, percentage, fs)
            })
            .collect();
    let (name, addr, prefix) = Networks::new_with_refreshed_list()
        .iter()
        .flat_map(|(name, net)| net.ip_networks().iter().map(move |ip| (name, ip)))
        .find(|(_, ip)| ip.addr.is_ipv4() && !ip.addr.is_loopback())
        .map(|(name, ip)| (name.clone(), ip.addr.to_string(), ip.prefix.to_string()))
        .unwrap_or_default();
    let locale = std::env::var("LANG").unwrap_or_default();

    println!("{G}OS:{R} {os} {arch}");
    println!("{G}Host:{R} {host}");
    println!("{G}Kernel:{R} {kernel}");
    println!("{G}Uptime:{R} {hours} hours, {minutes} minutes");
    println!("{G}Packages:{R} {packages}");
    println!("{G}Shell:{R} {shell}");
    println!("{G}Display:{R} {display}");
    println!("{G}DE:{R} {desktop_environment}");
    println!("{G}WM:{R} {window_manager}");
    println!("{G}WM Theme:{R} {}", theme.wm_theme);
    println!("{G}Theme:{R} {}", theme.theme);
    println!("{G}Icon:{R} {}", theme.icons);
    println!("{G}Font:{R} {}", theme.font);
    println!("{G}Cursor:{R} {} ({}px)", theme.cursor, theme.cursor_size);
    println!("{G}Terminal:{R} {terminal}");
    println!("{G}CPU:{R} {cpu}");
    println!("{G}GPU:{R} {gpu}");
    println!("{G}Memory:{R} {used_mem:.2} GiB / {total_mem:.2} GiB ({percentage_mem:.0}%)");
    println!("{G}Swap:{R} {used_swap:.2} GiB / {total_swap:.2} GiB ({percentage_swap:.0}%)");
    for (mount, used, total, percentage, fs) in disks_info {
        println!("{G}Disk ({mount}):{R} {used:.2} GiB / {total:.2} GiB ({percentage:.0}%) - {fs}");
    }
    println!("{G}Local IP ({name}):{R} {addr}/{prefix}");
    println!("{G}Locale:{R} {locale}");
}

fn bytes_to_gib(total: u64, used: u64) -> (f64, f64, u64) {
    let total_gib = f64::from(u32::try_from(total / 1_048_576).unwrap_or(0)) / 1024.0;
    let used_gib = f64::from(u32::try_from(used / 1_048_576).unwrap_or(0)) / 1024.0;
    let percentage = used
        .checked_mul(100)
        .and_then(|val| val.checked_div(total))
        .unwrap_or(0);
    (total_gib, used_gib, percentage)
}
