use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct KeyPress{
	id: u16,
	key: String,
	timestamp: i64,
}

pub fn increment_keypress_count(conn: Connection, key: String, count: u32) -> Result<(), rusqlite::Error> {
	let timestamp = chrono::Utc::now().timestamp();
	let mut stmt = conn.prepare("INSERT INTO key_presses (key, timestamp) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET timestamp = ?2")?;
	stmt.execute(params![key, timestamp])?;

	Ok(())
}
