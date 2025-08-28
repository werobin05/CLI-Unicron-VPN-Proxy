use crate::{
    service::{clear_terminal::clear_terminal, wait_enter::wait_enter},
    structs::dynamic_conn::get_connection_state,
};

pub fn disconnect() {
    let mut state = get_connection_state().lock().unwrap();
    if state.is_some() {
        *state = None;
        clear_terminal();
        println!("Disconnected successfully!")
    } else {
        clear_terminal();
        println!("Your are not connection")
    }
    wait_enter();
}
