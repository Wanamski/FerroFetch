use serde_derive::Deserialize;
use std::{fs::File, io::Read, process::exit};
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

    println!("OS in config set: {}", conf_data.print_info.os);
}
