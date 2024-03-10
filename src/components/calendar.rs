use std::collections::HashMap;
use yew::*;
use crate::components::menu::Menu;
use crate::constants::colors::BG_BASE;
use chrono::*;
use wasm_bindgen_futures::spawn_local;
use tauri_sys::tauri;
use serde::{Deserialize, Serialize};

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
		match tauri::invoke::<_, HashMap<String, u32>>("read_keypresses_of_month", &(year, month)).await {
			Ok(hm) => {
				keypresses.set(hm);
			},
			Err(e) => {
				let e = format!("{:?}", e);
				let mut hm = HashMap::new();
				hm.insert(e, 0);
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
										let debugstr = format!("{:?}", *v);
										html! {<div>{debugstr}</div>}
                                        //html! {<td style={date_style}>{format!("{} ({})", day, count)}</td>}
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

