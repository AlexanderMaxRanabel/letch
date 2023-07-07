use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::env;
use colored::*;

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
    let mut file = File::open("/etc/hostname").expect("Failed to open the file");
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

fn kernel_v() -> String {
    let mut kernel_file = File::open("/proc/version").expect("Failed to open the file");
    let mut kernel_name = String::new();
    kernel_file.read_to_string(&mut kernel_name).expect("Failed to read the file");

    let mut kernel_version: String = (kernel_name[14..kernel_name.len()]).to_string();
    kernel_version = kernel_version.split_whitespace()
        .next()
        .unwrap()
        .to_string();
    return kernel_version;
}

fn main() {
    let distro_name = distro_name_get();
    let hostname = hostname_get();
    let username = username();
    let kernel = kernel_v();
    let full = username.clone() + "@" + &hostname;

    match distro_name.as_str() {
        "Arch Linux" => {
            let logo = r#"
                       /\\
				      /  \\
				     /\\   \\
				    /      \\
				   /   ,,   \\
				  /   |  |  -\\
				 /_-''    ''-_\\
            "#;

            println!("{}", logo.blue());
        },

        "NixOS" => {
            let logo = r#"
  \\  \\ //      
 ==\\__\\/ //    
   //   \\//     
==//     //==    
 //\\___//       
// /\\  \\==     
  // \\  \\  
            "#;

            println!("{}", logo.blue());
        },
        _=> {
           println!("LOGO not avaible"); 
        }
    }
    println!("{}", full);
    println!(" ");
    println!("| User: {} ", username.red());
    println!("| Distro: {}", distro_name.blue());
    println!("| Hostname: {}", hostname.trim().red());
    println!("| Kernel: {}", kernel.green());
}
