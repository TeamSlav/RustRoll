use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn generate_redirect_script(random_url: String) {
    let selections = &["windows", "gnu/linux", "macos"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("For what platform should the script be for:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("you selected: {}!", selections[selection]);
    if selections[selection] == "windows" {
        generate_windows_script(random_url.to_string());
    } else if selections[selection] == "gnu/linux" {
        generate_linux_script(random_url.to_string());
    } else if selections[selection] == "macos" {
        generate_macos_script(random_url.to_string());
    } else {
        println!("you selected nothing");
    }
}

fn generate_windows_script(random_url: String) {
    let url_random = format!(" start {}", random_url.to_string());

    let mut file_name = String::new();

    println!("enter the name of the script(with file extension):");

    io::stdin()
        .read_line(&mut file_name)
        .expect("failed to read line");

    let mut file = File::create("Not-rickroll.sh").expect("Error encountered while creating file!");
    file.write_all(url_random.as_bytes())
        .expect("Error while writing to file");
    fs::rename("Not-rickroll.sh", file_name); // Rename a.txt to b.txt
}

fn generate_linux_script(random_url: String) {
    let url_random = format!("start {}", random_url.to_string());

    let mut file_name = String::new();

    println!("enter the name of the script(with file extension):");

    io::stdin()
        .read_line(&mut file_name)
        .expect("failed to read line");

    let mut file = File::create("Not-rickroll.sh").expect("Error encountered while creating file!");
    file.write_all(url_random.as_bytes())
        .expect("Error while writing to file");
    fs::rename("Not-rickroll.sh", file_name); // Rename a.txt to b.txt
}

fn generate_macos_script(random_url: String) {
    let url_random = format!("open {}", random_url.to_string());

    let mut file_name = String::new();

    println!("enter the name of the script(with file extension):");

    io::stdin()
        .read_line(&mut file_name)
        .expect("failed to read line");

    let mut file = File::create("Not-rickroll.sh").expect("Error encountered while creating file!");
    file.write_all(url_random.as_bytes())
        .expect("Error while writing to file");
    fs::rename("Not-rickroll.sh", file_name); // Rename a.txt to b.txt
}
