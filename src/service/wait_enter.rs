use std::io;

pub fn wait_enter() {
    let mut input = String::new();
    println!("\nPress Enter to continue...");
    io::stdin().read_line(&mut input).unwrap();
}
