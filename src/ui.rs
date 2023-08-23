use std::io::{stdin, stdout, Write};

pub fn write_main_menu() {
    write_title();
    println!("\n Main Menu");
    println!(" ---------");
    println!("\n What would you like to do?");
    println!("   [A] --> List all available database");
    println!("   [B] --> Create a new Database");
    println!("   [C] --> Delete a Database");
    println!("   [D] --> Connect to a database");
    println!("   [E] --> Exit program");
    print!("\n     Please type in the letter corresponding to your choice --> ");
    flush();
}

pub fn write_title() {
    clearscreen::ClearScreen::default().clear().unwrap();
    println!("\n----------------------");
    println!("   MariaDB cli tool");
    println!("----------------------");
}

pub fn pause_console() {
    let mut temp = String::new();
    stdin().read_line(&mut temp).unwrap();
}

pub fn flush() {
    stdout().flush().unwrap();
}

pub fn get_input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
