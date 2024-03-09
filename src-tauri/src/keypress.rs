use rdev::{Event, Key};
use rdev::EventType::KeyRelease;
use rusqlite::Connection;
use crate::models::keypress::increment_keypress_count;

pub fn handle_keypress(conn: Connection, event: Event) {
	match event.event_type {
		KeyRelease(key) => {
			let s = classify_keypress(key);
			increment_keypress_count(conn, s.clone(), 1).unwrap();
			
			println!("Key released: {:?}", s);
		},
		_ => {}
	}
}

fn classify_keypress(key: Key) -> String {
	match key {
		Key::Num0 => "0".to_string(),
		Key::Num1 => "1".to_string(),
		Key::Num2 => "2".to_string(),
		Key::Num3 => "3".to_string(),
		Key::Num4 => "4".to_string(),
		Key::Num5 => "5".to_string(),
		Key::Num6 => "6".to_string(),
		Key::Num7 => "7".to_string(),
		Key::Num8 => "8".to_string(),
		Key::Num9 => "9".to_string(),
		Key::KeyA => "a".to_string(),
		Key::KeyB => "b".to_string(),
		Key::KeyC => "c".to_string(),
		Key::KeyD => "d".to_string(),
		Key::KeyE => "e".to_string(),
		Key::KeyF => "f".to_string(),
		Key::KeyG => "g".to_string(),
		Key::KeyH => "h".to_string(),
		Key::KeyI => "i".to_string(),
		Key::KeyJ => "j".to_string(),
		Key::KeyK => "k".to_string(),
		Key::KeyL => "l".to_string(),
		Key::KeyM => "m".to_string(),
		Key::KeyN => "n".to_string(),
		Key::KeyO => "o".to_string(),
		Key::KeyP => "p".to_string(),
		Key::KeyQ => "q".to_string(),
		Key::KeyR => "r".to_string(),
		Key::KeyS => "s".to_string(),
		Key::KeyT => "t".to_string(),
		Key::KeyU => "u".to_string(),
		Key::KeyV => "v".to_string(),
		Key::KeyW => "w".to_string(),
		Key::KeyX => "x".to_string(),
		Key::KeyY => "y".to_string(),
		Key::KeyZ => "z".to_string(),
		Key::Backspace => "backspace".to_string(),
		Key::ShiftRight => "shift_right".to_string(),
		Key::ShiftLeft => "shift_left".to_string(),
		Key::ControlRight => "control_right".to_string(),
		Key::ControlLeft => "control_left".to_string(),
		Key::MetaRight => "meta_right".to_string(),
		Key::MetaLeft => "meta_left".to_string(),
		Key::Space => "space".to_string(),
		Key::Alt => "alt".to_string(),
		Key::AltGr => "alt_gr".to_string(),
		Key::Comma => "comma".to_string(),
		Key::Dot => "dot".to_string(),
		Key::Slash => "slash".to_string(),
		Key::SemiColon => "semicolon".to_string(),
		Key::Quote => "quote".to_string(),
		Key::BackSlash => "backslash".to_string(),
		Key::Return => "return".to_string(),
		_ => "unknown".to_string(),
	}
}

