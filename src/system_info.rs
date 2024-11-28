use clap::Args;
use sysinfo::{System, SystemExt, CpuExt};
use os_info;

#[derive(Args)]
pub struct InfoOptions {
    #[arg(short, long, help = "Display all system information")]
    all: bool,

    #[arg(short, long, help = "Display CPU information")]
    cpu: bool,

    #[arg(short, long, help = "Display memory information")]
    memory: bool,

    #[arg(short, long, help = "Display OS information")]
    os: bool,
}

pub fn run_system_info(options: InfoOptions) {
    let mut sys = System::new_all();
    sys.refresh_all();

    if options.all {
        print_cpu_info(&sys);
        print_memory_info(&sys);
        print_os_info();
    } else {
        if options.cpu {
            print_cpu_info(&sys);
        }
        if options.memory {
            print_memory_info(&sys);
        }
        if options.os {
            print_os_info();
        }
    }
}

fn print_cpu_info(sys: &System) {
    println!("CPU Information:");
    for cpu in sys.cpus() {
        println!("  Name: {}", cpu.name());
        println!("  Frequency: {} MHz", cpu.frequency());
    }
    println!();
}

fn print_memory_info(sys: &System) {
    println!("Memory Information:");
    println!("  Total Memory: {} MB", sys.total_memory() / 1024);
    println!("  Used Memory: {} MB", sys.used_memory() / 1024);
    println!("  Available Memory: {} MB", sys.available_memory() / 1024);
    println!();
}

fn print_os_info() {
    let info = os_info::get();
    println!("OS Information:");
    println!("  OS: {}", info.os_type());
    println!("  Version: {}", info.version());
    println!("  Architecture: {}", info.bitness());
    println!();
}
