// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod keypress;
mod models;

use std::thread;
use keypress::handle_keypress;
use models::conn::open_conn;
use rusqlite_migration::{Migrations, M};
use rdev::Event;

#[tauri::command]
fn read_keypresses_of_day() -> String {
	"Hello from Rust!".to_string()
}

fn main() {
	let migrations = Migrations::new(vec![
		M::up("CREATE TABLE key_presses (id INTEGER PRIMARY KEY AUTOINCREMENT, key TEXT, timestamp INTEGER);")
    ]);

	thread::spawn(move || {
		let handler = move |event: Event| {
			let mut conn = open_conn().unwrap();
			migrations.to_latest(&mut conn).unwrap();

			handle_keypress(conn, event);
		};


		rdev::listen(handler).expect("Failed to listen for keypresses");
	});

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
