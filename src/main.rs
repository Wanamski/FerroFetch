use serde_derive::Deserialize;
use std::{fs, fs::File, io::Read, process::exit};
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
    cpu_model_and_cores();
    cpu_max_speed();

    println!("OS in config set: {}", conf_data.print_info.os);
}

fn cpu_model_and_cores() {
    let cpu_data = fs::read_to_string("/proc/cpuinfo")
        .expect("No Information about the cpu could be retrievet");

    // 18 unnötige Zeichen
    for line in cpu_data.lines() {
        if line.contains("model name") {
            let cpu_data = line;
            let mut parts = cpu_data.split(": ");
            println!("CPU Model: {}", parts.nth(1).unwrap());
        } else if line.contains("cpu cores") {
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

    let cpu_speed = (cpu_speed / 1000.0) / 1000.0;

    println!("{cpu_speed} GHz")
}