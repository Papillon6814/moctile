// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod keypress;
mod models;

use std::thread;
use std::sync::{Arc, Mutex};
use keypress::handle_keypress;
use models::conn::open_conn;
use rusqlite_migration::{Migrations, M};
use rusqlite::Connection;
use rdev::Event;

struct DbConn(Arc<Mutex<Connection>>);

#[tauri::command]
fn read_keypresses_of_month(year: i32, month: u32) -> String {
	"Hello from Rust!".to_string()
}

fn main() {
	let conn = Arc::new(Mutex::new(open_conn().unwrap()));
	let db_conn = DbConn(conn.clone());
	let handler_conn = conn.clone();

	let migrations = Migrations::new(vec![
		M::up("
			CREATE TABLE daily_key_presses (
				id INTEGER PRIMARY KEY,
				date TEXT NOT NULL,
				key_name TEXT NOT NULL,
				count INTEGER NOT NULL,
				UNIQUE(date, key_name)
			);
		"),
    ]);

	thread::spawn(move || {
		let handler = move |event: Event| {
			let mut conn = handler_conn.lock().unwrap();
			migrations.to_latest(&mut *conn).unwrap();

			handle_keypress(&*conn, event);
		};


		rdev::listen(handler).expect("Failed to listen for keypresses");
	});

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
		.manage(db_conn)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
