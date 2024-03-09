use rusqlite::*;

pub struct KeyPress{
	id: u16,
	date: String,
	key_name: String,
	count: u32,
}

pub fn get_total_key_press_count_for_date(conn: &Connection, date: &str) -> u32 {
    conn.query_row(
        "SELECT SUM(count) FROM daily_key_presses WHERE date = ?1",
        params![date],
        |row| row.get(0),
    ).unwrap_or(0)
}

pub fn load_keypress(conn: &Connection, date: String, key_name: String) -> Result<Option<KeyPress>> {
	let mut stmt = (*conn).prepare("SELECT id, date, key_name, count FROM daily_key_presses WHERE date = ?1 AND key_name = ?2")?;
    
    let key_press = stmt.query_row(params![date, key_name], |row| {
        Ok(KeyPress {
            id: row.get(0)?,
            date: row.get(1)?,
            key_name: row.get(2)?,
            count: row.get(3)?,
        })
    }).optional()?;

    Ok(key_press)
}

pub fn increment_keypress_count(conn: &Connection, key: String, count: u32) -> Result<(), rusqlite::Error> {
	let timestamp = chrono::Utc::now().timestamp();
	let mut stmt = (*conn).prepare("
		INSERT INTO daily_key_presses (date, key_name, count)
		VALUES ('2024-03-10', 'Space', 1)
		ON CONFLICT(date, key_name) DO UPDATE SET count = count + 1;
	")?;
	stmt.execute(params![key, timestamp])?;

	Ok(())
}

