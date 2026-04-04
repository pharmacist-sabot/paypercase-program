// state.rs — Application state

use std::sync::{Arc, Mutex};
use mysql::Pool;
use rusqlite::Connection;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub hosxp_pool: Mutex<Option<Pool>>,
}

impl AppState {
    pub fn new(conn: Connection) -> Arc<Self> {
        Arc::new(AppState {
            db: Mutex::new(conn),
            hosxp_pool: Mutex::new(None),
        })
    }
}
