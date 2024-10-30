use serde_derive::Deserialize;
use std::{
    fs::{self, File},
    io::Read,
    process::exit,
};
use toml;

#[derive(Deserialize)]
struct Conf {
    print_info: PrintInfo,
}

#[derive(Deserialize)]
struct PrintInfo {
    os: bool,
}

fn main() {
    // reading contents from config file into String
    let conf_path = "./ff_conf.toml";
    let mut conf_file = File::open(conf_path).expect("failed to open the config file...");
    let mut conf_contents = String::new();
    conf_file
        .read_to_string(&mut conf_contents)
        .expect("failed to read config file...");

    let conf_data: Conf = match toml::from_str(&conf_contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Could not load config file from {conf_path}");
            exit(1);
        }
    };

    let cpu_data = fs::read_to_string("/proc/cpuinfo")
        .expect("No Information about the CPU could be retrievet");

    os_name_and_architecture();
    cpu_model(&cpu_data);
    cpu_cores(&cpu_data);
    cpu_threads(&cpu_data);
    cpu_max_speed();

    println!("OS in config set: {}", conf_data.print_info.os);
}

fn os_name_and_architecture() {
    let os_name =
        fs::read_to_string("/etc/os-release").expect("No information about the OS could be found");

    for line in os_name.lines() {
        if line.contains("PRETTY_NAME") {
            let os_name = line;
            let mut parts = os_name.split("\"");
            println!("OS: {}", parts.nth(1).unwrap());
            break;
        }
    }
}

fn cpu_model(cpu_data: &String) {
    // 18 unnötige Zeichen
    for line in cpu_data.lines() {
        if line.contains("model name") {
            let cpu_data = line;
            let mut parts = cpu_data.split(": ");
            println!("CPU model: {}", parts.nth(1).unwrap());
            break;
        }
    }
}

fn cpu_threads(cpu_data: &String) {
    for line in cpu_data.lines() {
        if line.contains("siblings") {
            let cpu_data = line;
            let mut parts = cpu_data.split(": ");
            println!("CPU threads: {}", parts.nth(1).unwrap());
            break;
        }
    }
}

fn cpu_cores(cpu_data: &String) {
    for line in cpu_data.lines() {
        if line.contains("cpu cores") {
            let cpu_data = line;
            let mut parts = cpu_data.split(": ");
            println!("CPU cores: {}", parts.nth(1).unwrap());
            break;
        }
    }
}

fn cpu_max_speed() {
    // Saves the content, the CPU clock speed in Hz, of the cpuinfo_max_freq file as String
    let cpu_speed = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
        .expect("No Data were fond");

    // Casts the value from String to float 64 Bit
    let cpu_speed: f64 = match cpu_speed.trim().parse() {
        Ok(num) => num,
        Err(_) => 42.0,
    };

    let cpu_speed = cpu_speed / 1_000_000.0;

    println!("CPU max speed: {cpu_speed} GHz")
}
