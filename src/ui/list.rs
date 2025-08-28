use std::{fs, io};

use crate::{
    service::{
        clear_terminal::clear_terminal,
        connect_to::connect_to,
        crypto::decrypt,
        path_key::{get_connection_path, get_encryption_key},
        wait_enter::wait_enter,
    },
    structs::connection::Connection,
};

pub fn list_connection() {
    let path = get_connection_path();
    let key = get_encryption_key();

    if !path.exists() {
        clear_terminal();
        println!("|--- Your Connections ---|\n|");
        println!("|No connection found.\n|");
        println!("|------------------------|");
        wait_enter();
        return;
    }

    let encrypted_list: Vec<String> =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap_or_default();

    if encrypted_list.is_empty() {
        clear_terminal();
        println!("\n--- Your list Connections ---\n|");
        print!("|No connection found.\n");
        println!("------------------------|");
        wait_enter();
        return;
    }

    clear_terminal();
    println!("\n|----- Your list Connections -----\n|");
    for (i, encrypted) in encrypted_list.iter().enumerate() {
        let decrypted = decrypt(&encrypted, &key);
        let conn: Connection = serde_json::from_str(&decrypted).unwrap();
        println!(
            "|{}: {} -> {} {} [{}]",
            i + 1,
            conn.name,
            conn.server,
            conn.port,
            conn.protocol,
        );
    }
    println!("|\n|---------------------------------");
    print!("Enter number to connect, or [X] to return: \n");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.trim().eq_ignore_ascii_case("x") {
            break;
        }

        if let Ok(num) = input.parse::<usize>() {
            if num >= 1 && num <= encrypted_list.len() {
                let decrypted = decrypt(&encrypted_list[num - 1], &key);
                let conn: Connection = serde_json::from_str(&decrypted).unwrap();
                connect_to(&conn);
                break;
            } else {
                println!("Invalid number, try again:");
            }
        } else {
            println!("Invalid input, enter number or [x]:")
        }
    }
}
