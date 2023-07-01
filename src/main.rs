use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::env;

fn distro_name_get() -> String {
    //most of the code comes from my other project
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("ID=nixos") {
                    return "NixOS".to_string();
                } else if line.starts_with("ID=arch") {
                    return "Arch Linux".to_string();
                } else if line.starts_with("ID=debian") {
                    return "Debian".to_string();
                } else if line.starts_with("ID=fedora") {
                    return "Fedora".to_string();
                }
            }
        }
    }
    return "Linux".to_string();
}

fn hostname_get() -> String {
    let mut file = File::open("/etc/hostname").expect("Failed to open file");
    let mut hostname = String::new();
    file.read_to_string(&mut hostname).expect("Failed to read the file");

    return hostname;
}

fn username() -> String {
    match env::var("USER") {
        Ok(username) => return username,
        Err(_) => return "Unknown".to_string(),
    }
}
fn main() {
    let distro_name = distro_name_get();
    let hostname = hostname_get();
    let username = username();
    let full = username.clone() + "@" + &hostname;
    println!("{}", full);
    println!(" ");
    println!("User: {} ", username);
    println!("Distro: {}", distro_name);
    println!("Hostname: {}", hostname);
}
