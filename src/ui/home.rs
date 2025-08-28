use crate::structs::dynamic_conn::get_connection_state;

pub fn home_page() {
    print!("----Welcome! Unicron CLI Network----\n");

    let mut status = String::from("[Not connected]");
    if let Some(conn) = get_connection_state().lock().unwrap().clone() {
        status = format!("Connected to {} {}", conn.name, conn.server);
    }
    let menu = format!(
        r#"
        [------------Connection------------]
        [1]: Added connection;
        [2]: Your list connection;
        [3]: Remove connection;
        [------------About CLI-------------]
        [4]: GitHub;
        [5]: About CLI;
        [--------------Others--------------]
        [c]: clear terminal/console
        [q]: quit;
        [d]: disconnect
        [?] How does it work?
        [------------Connection------------]
        {status}
        [----------------------------------]

    "#
    );
    println!("{menu}")
}
