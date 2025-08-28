use std::sync::{Mutex, OnceLock};

#[derive(Clone)]
pub struct CurrentConnection {
    pub name: String,
    pub server: String,
}

static CONNECTION: OnceLock<Mutex<Option<CurrentConnection>>> = OnceLock::new();

pub fn get_connection_state() -> &'static Mutex<Option<CurrentConnection>> {
    CONNECTION.get_or_init(|| Mutex::new(None))
}