// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod keypress;
mod models;

use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use keypress::handle_keypress;
use models::conn::open_conn;
use models::keypress::get_total_key_press_count_for_date;
use rusqlite_migration::{Migrations, M};
use rusqlite::Connection;
use rdev::Event;
use chrono::*;
use tauri::State;

struct DbConn(Arc<Mutex<Connection>>);

#[tauri::command]
fn read_keypresses_of_month(conn: State<'_, DbConn>, year: i32, month: u32) -> HashMap<String, u32> {
	println!("Reading keypresses for {}-{}", year, month);
    let conn = conn.0.lock().unwrap();
    let mut counts_map = HashMap::new();

    let days_in_month = NaiveDate::from_ymd_opt(year, month, 1)
        .and_then(|d| d.with_month(month + 1).and_then(|next_month| next_month.pred_opt()))
        .map_or(0, |last_day_of_month| last_day_of_month.day());

    for day in 1..=days_in_month {
        let date = format!("{}-{:02}-{:02}", year, month, day);
        let count = get_total_key_press_count_for_date(&*conn, &date);
        counts_map.insert(date, count);
    }

    counts_map
}

fn main() {
	let conn = DbConn(Arc::new(Mutex::new(open_conn().unwrap())));
	let handler_conn = conn.0.clone();

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
	let migration_conn = conn.0.clone();
	let mut migration_conn_lock = migration_conn.lock().unwrap();
	migrations.to_latest(&mut *migration_conn_lock).unwrap();
	drop(migration_conn_lock);

	thread::spawn(move || {
		let handler = move |event: Event| {
			let conn_lock = handler_conn.lock().unwrap();
			handle_keypress(&*conn_lock, event);
			drop(conn_lock);
		};

		rdev::listen(handler).expect("Failed to listen for keypresses");
	});

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
		.manage(conn)
		.invoke_handler(tauri::generate_handler![read_keypresses_of_month])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
