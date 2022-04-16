use dialoguer::{theme::ColorfulTheme, Select};
use rand::{self, Rng};
extern crate urlshortener;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::fs;






fn main() {

    let urls = [
    "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
    "https://www.youtube.com/watch?v=oHg5SJYRHA0",
    "https://www.youtube.com/watch?v=6_b7RDuLwcI",         
    "https://www.youtube.com/watch?v=G8iEMVr7GFg",        
    "https://www.youtube.com/watch?v=AyOqGRjVtls",        
    "https://www.youtube.com/watch?v=6mhmcwmgWbA",       
    "https://www.youtube.com/watch?v=SpZ2FsEfwP4",        
    "https://www.youtube.com/watch?v=H01BwSD9eyQ",        
    "https://www.youtube.com/watch?v=nrsnN23tmUA",        
    "https://www.youtube.com/watch?v=8mkofgRW1II",       
    "https://www.youtube.com/watch?v=rAx5LIul1N8",        
    "https://www.youtube.com/watch?v=sO4wVSA9UPs",        
    "https://www.youtube.com/watch?v=rrs0B_LM898",        
    "https://www.youtube.com/watch?v=doEqUhFiQS4",        
    "https://www.youtube.com/watch?v=epyRUp0BhrA",        
    "https://www.youtube.com/watch?v=uK5WDo_3s7s",        
    "https://www.youtube.com/watch?v=wzSVOcgKq04",     
    "https://www.youtube.com/watch?v=7B--1KArxow",        
    "https://www.youtube.com/watch?v=rbsPu1z3ugQ",        
    "https://www.youtube.com/watch?v=ptw2FLKXDQE",       
    "https://www.youtube.com/watch?v=E50L-JYWm3w", 
    "https://www.youtube.com/watch?v=8leAAwMIigI", 
    "https://www.youtube.com/watch?v=ByqFY-Boq5Y",  
    "https://www.youtube.com/watch?v=E4ihJMQUmUQ",   
    "https://www.youtube.com/watch?v=cjBHXvBYw5s",    
    "https://www.youtube.com/watch?v=xaazUgEKuVA",      
    "https://www.youtube.com/watch?v=TzXXHVhGXTQ",
    "https://www.youtube.com/watch?v=Uj1ykZWtPYI",
    "https://www.youtube.com/watch?v=EE-xtCF3T94",
    "https://www.youtube.com/watch?v=V-_O7nl0Ii0",
    "https://www.youtube.com/watch?v=cqF6M25kqq4",
    "https://www.youtube.com/watch?v=0SoNH07Slj0",
    "https://www.youtube.com/watch?v=xfr64zoBTAQ",
    "https://www.youtube.com/watch?v=j5a0jTc9S10",
    "https://www.youtube.com/watch?v=dPmZqsQNzGA",
    "https://www.youtube.com/watch?v=nHRbZW097Uk",
    "https://www.youtube.com/watch?v=BjDebmqFRuc",
    "https://www.youtube.com/watch?v=Gc2u6AFImn8",
    "https://www.youtube.com/watch?v=8VFzHYtOARw",
    "https://www.youtube.com/watch?v=cSAp9sBzPbc",
    "https://www.youtube.com/watch?v=Dx5i1t0mN78",
    "https://www.youtube.com/watch?v=Oo0twK2ZbLU",
    "https://www.youtube.com/watch?v=cvh0nX08nRw",
    "https://www.youtube.com/watch?v=lXMskKTw3Bc",
    "https://www.youtube.com/watch?v=7z_1E8VGJOw",
    "https://www.youtube.com/watch?v=VgojnNgmgVs",
    "https://www.youtube.com/watch?v=5wOXc03RwVA"
    ];
    let random_url = urls[rand::thread_rng().gen_range(0..urls.len())];


    let selections = &[
        "redirect-script(redirects target to a rickroll link)",
        "random-url(gives a random url)",
        "",

    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("what rickroll do you want to generate?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("you selected: {}!", selections[selection]);
    if selections[selection] == "redirect-script(redirects target to a rickroll link)" {
        generate_redirect_script(random_url.to_string());
    } else if selections[selection] == "random-url(gives a random url)" {
        generate_urls(random_url.to_string());
    } else {
        println!("you selected nothing");
    }
}

fn generate_urls(random_url: String,) {
    

    //get a random url from the array
    
    println!("{}", random_url);

    let selections = &[
        "shortened",
        "None",

    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("select url shortening service")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    
    
    println!("you selected: {}!", selections[selection]);
    
    if selections[selection] == "shortened" {
        shorten_url(random_url.to_string());
    } else if selections[selection] == "None" {
        println!("{}", random_url);
    } else {
        println!("you selected nothing");
    }


}

fn shorten_url(random_url: String) {


    println!("idk how to shorten urls lol so here is a random one");
    println!("{}", random_url);
    
    

}

fn generate_redirect_script(random_url: String) {
    let selections = &[
        "windows",
        "gnu/linux",
        "macos",

    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("For what platform should the script be for:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    
    
    println!("you selected: {}!", selections[selection]);
    if selections[selection] == "windows" {
        generate_windows_script(random_url.to_string());
    }else if selections[selection] == "gnu/linux" {
        generate_linux_script(random_url.to_string());
    }else if selections[selection] == "macos" {
        generate_macos_script(random_url.to_string());
    } else {
        println!("you selected nothing");
    }
}

fn generate_windows_script(random_url: String) {
    let url_random = format!(" start {}", random_url.to_string());

    let mut file_name = String::new();

    println!("enter the name of the script(with file extension):");

    io::stdin().read_line(&mut file_name).expect("failed to read line");


    
    let mut file = File::create("Not-rickroll.sh").expect("Error encountered while creating file!");
    file.write_all(url_random.as_bytes()).expect("Error while writing to file");
    fs::rename("Not-rickroll.sh", file_name); // Rename a.txt to b.txt
}

fn generate_linux_script(random_url: String) {
    let url_random = format!("start {}", random_url.to_string());

    let mut file_name = String::new();

    println!("enter the name of the script(with file extension):");

    io::stdin().read_line(&mut file_name).expect("failed to read line");


    
    let mut file = File::create("Not-rickroll.sh").expect("Error encountered while creating file!");
    file.write_all(url_random.as_bytes()).expect("Error while writing to file");
    fs::rename("Not-rickroll.sh", file_name); // Rename a.txt to b.txt
}

fn generate_macos_script(random_url: String) {
    let url_random = format!("open {}", random_url.to_string());

    let mut file_name = String::new();

    println!("enter the name of the script(with file extension):");

    io::stdin().read_line(&mut file_name).expect("failed to read line");


    
    let mut file = File::create("Not-rickroll.sh").expect("Error encountered while creating file!");
    file.write_all(url_random.as_bytes()).expect("Error while writing to file");
    fs::rename("Not-rickroll.sh", file_name); // Rename a.txt to b.txt
}


    

    
    
