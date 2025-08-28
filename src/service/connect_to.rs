use crate::{
    service::{clear_terminal::clear_terminal, wait_enter::wait_enter},
    structs::{
        connection::Connection,
        dynamic_conn::{CurrentConnection, get_connection_state},
    },
};

pub fn connect_to(conn: &Connection) {
    clear_terminal();
    println!(
        "Connecting to {} ({} {}) via {}...",
        conn.name, conn.server, conn.port, conn.protocol
    );
    let mut state = get_connection_state().lock().unwrap();
    *state = Some(CurrentConnection {
        name: conn.name.clone(),
        server: conn.server.clone(),
    });
    wait_enter();
}
