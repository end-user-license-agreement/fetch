use std::{env, fs};
use sysinfo::{CpuExt, System, SystemExt};
use time::Duration;
use ubyte::ToByteUnit;
use yansi::Color;

fn main() {
    let system = System::new_all();
    let uptime = Duration::seconds(system.uptime() as i64).to_string();
    let cpu = system.global_cpu_info();
    let cpu_frequency = cpu.frequency() as f64 / 1000.0;
    let cpu_brand = cpu.brand();
    let mem_used = system.used_memory().bytes();
    let mem_available = system.total_memory().bytes();
    let desktop = env::var("XDG_CURRENT_DESKTOP");
    let os = system.long_os_version();
    let os = os.as_deref().unwrap_or("Linux");
    let kernel = system.kernel_version();
    let kernel = kernel.as_deref().unwrap_or("Unknown");

    let portage_count = fs::read_dir("/var/db/pkg").map(|iter| {
        iter.flatten()
            .flat_map(|entry| fs::read_dir(entry.path()))
            .flatten()
            .flatten()
            .count()
    });

    let paint = Color::Magenta;

    println!("{}: {os}", paint.paint("OS"));
    println!("{}: {kernel}", paint.paint("Kernel"));
    println!("{}: {uptime}", paint.paint("Uptime"));

    if let Ok(desktop) = desktop {
        println!("{}: {desktop}", paint.paint("DE"));
    }

    if let Ok(count) = portage_count {
        println!("{}: {count} (emerge)", paint.paint("Packages"));
    }

    println!(
        "{}: {cpu_brand} @ {cpu_frequency:.2}GHz",
        paint.paint("CPU")
    );

    println!("{}: {mem_used} / {mem_available}", paint.paint("Memory"));
}
