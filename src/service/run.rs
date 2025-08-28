use crate::service::add_connection::add_connection;
use crate::service::clear_terminal::clear_terminal;
use crate::service::disconnect::disconnect;
use crate::ui::home::home_page;
use crate::ui::list::list_connection;
use std::io::{self, Write};

pub async fn run_cli() {
    loop {
        home_page();

        print!("Enter option: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let choice = input.trim();

        match choice {
            "1" => add_connection(),
            "2" => list_connection(),
            "3" => println!("Remove connection"),
            "4" => println!("GitHub"),
            "5" => println!("About CLI"),
            "6" => println!("Developer"),
            "d" => disconnect(),
            "?" => println!(
                "Connecting via the console works the same way as in the client, 
                but here it's right in the console."
            ),
            "c" => clear_terminal(),
            "q" | "quit" => break,
            _ => println!("Unknown option"),
        }
    }
}
