use sysinfo::System;

fn main() {
    let mut sys = System::new();

    sys.refresh_all();

    let cpus = sys.cpus();

    println!("CPU");
    println!("Model:                {}", cpus[0].brand());
    match sys.physical_core_count() {
        Some(count) => println!("Number of Cores:      {}", count),
        None => println!("Unable to retrieve cores"),
    }
    println!("Number of CPUs:       {}", cpus.len());
}
