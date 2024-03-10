use rusqlite::{Connection, Result};

pub fn open_conn() -> Result<Connection, rusqlite::Error> {
	let path = "sqlite:moctile.db";
	let conn = Connection::open(path)?;

	Ok(conn)
}

