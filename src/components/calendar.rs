use std::collections::HashMap;
use yew::*;
use crate::components::menu::Menu;
use crate::constants::colors::BG_BASE;
use chrono::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};

fn convert_jsvalue_to_hashmap(js_value: JsValue) -> Result<HashMap<String, u32>, JsValue> {
    // JsValueがオブジェクトであることを確認
    if js_value.is_object() {
        let js_object = js_sys::Object::from(js_value);
        let entries = js_sys::Object::entries(&js_object).into_iter();
        let mut map = HashMap::new();

        for entry in entries {
            let entry_array: js_sys::Array = entry.dyn_into().unwrap();
            let key = entry_array.get(0).as_string().unwrap();
            let value = entry_array.get(1).as_f64().unwrap() as u32; // JavaScriptの数値をRustのu32に変換
            map.insert(key, value);
        }

        Ok(map)
    } else {
        Err(JsValue::from_str("Input JsValue is not an Object."))
    }
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
	#[wasm_bindgen(js_name = readKeyPressesOfMonth, catch)]
	pub async fn read_keypresses_of_month(year: i32, month: u32) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct UpdateKeyPressInput {
	year: i32,
	month: u32,
}

#[derive(Properties, Clone, PartialEq)]
struct MonthViewProps {
	year: i32,
	month: u32,
}

fn update_keypresses(year: i32, month: u32, keypresses: UseStateHandle<HashMap<String, u32>>) {
	spawn_local(async move {
		match read_keypresses_of_month(year, month).await {
			Ok(val) => {
				let key_presses: HashMap<String, u32> = convert_jsvalue_to_hashmap(val).unwrap();
				println!("Key presses: {:?}", key_presses);
				keypresses.set(key_presses);
			},
			Err(e) => {
				let mut hm = HashMap::new();
				hm.insert(format!("{:?}", e), 1);
				keypresses.set(hm);
			}
		}
	})
}

#[function_component(MonthView)]
fn month_view(props: &MonthViewProps) -> Html {
	let MonthViewProps { year, month } = props;
	let keypresses = use_state(|| HashMap::<String, u32>::new());

	use_effect_with((year.clone(), month.clone(), keypresses.clone()), move |(year, month, keypresses)| {
		update_keypresses(*year, *month, keypresses.clone());
	});

	let first_day_of_month = NaiveDate::from_ymd_opt(*year, *month, 1).unwrap();
	let first_weekday = first_day_of_month.weekday();
	let days_in_month = if *month == 12 {
        NaiveDate::from_ymd_opt(*year + 1, 1, 1).unwrap().pred_opt().unwrap().day()
    } else {
        NaiveDate::from_ymd_opt(*year, *month + 1, 1).unwrap().pred_opt().unwrap().day()
    } as i32;

	let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
	let start_padding = first_weekday.num_days_from_sunday() as i32;
	let total_slots = days_in_month + start_padding;
	let weeks = (total_slots as f32 / 7.0).ceil() as i32;

	let weekday_style = "font-family: DIN;";
	let date_style = "width: 40px; height: 40px; text-align: center; font-family: DIN;";

	html! {
		<>
			<table>
				<thead>
					<tr>
						{weekdays.iter().map(|weekday| html! {<th style={weekday_style}>{weekday}</th>}).collect::<Html>()}
					</tr>
				</thead>
				<tbody>
				{
					for (0..weeks).map(|week| {
						html! {
							<tr>
							{
								for (0..7).map(|weekday| {
									let day = week * 7 + weekday + 1 - start_padding;
									if day > 0 && day <= days_in_month {
										let date_key = format!("{}-{:02}-{:02}", *year, *month, day);
										let v = keypresses.clone();
                                        let count = v.get(&date_key).unwrap_or(&0);
                                        html! {<td style={date_style}>{format!("{} ({})", day, count)}</td>}
									} else {
										html! {<td style={date_style}></td>}
									}
								})
							}
							</tr>
						}
					})
				}
				</tbody>
			</table>
		</>
	}
}

#[function_component(Calendar)]
pub fn calendar() -> Html {
	let year = use_state(|| Local::now().year());
	let month = use_state(|| Local::now().month());

	let on_next = use_callback((year.clone(), month.clone()), move |_: MouseEvent, (year, month)| {
		if **month == 12 {
			year.set(**year + 1);
			month.set(1);
		} else {
			month.set(**month + 1);
		}
	});

	let on_prev = use_callback((year.clone(), month.clone()), move |_: MouseEvent, (year, month)| {
		if **month == 1 {
			year.set(**year - 1);
			month.set(12);
		} else {
			month.set(**month - 1);
		}
	});

	let main_style = "padding-top: 20px;";
	let button_style = format!("background-color: {}; outline: none; border: 1px; height: 36px; width: 36px; border-radius: 6px; cursor: pointer;", BG_BASE);

	html! {
		<div style={"display: flex; margin: 0;"}>
			<Menu />
			<main style={main_style}>
				<div style={"display: flex; justify-content: space-between; align-items: center;"}>
					<button style={button_style.clone()} onclick={on_prev}>
						<img src="/public/icons/chevron-left.svg" />
					</button>
					<h1 style={"font-family: DIN;"}>{format!("{} {}", *year, *month)}</h1>
					<button style={button_style.clone()} onclick={on_next}>
						<img src="/public/icons/chevron-right.svg" />
					</button>
				</div>
				<MonthView year={*year} month={*month} />
			</main>
		</div>
	}
}

