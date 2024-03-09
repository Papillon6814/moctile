use yew::{function_component, html, Html, Properties, use_state};
use crate::components::menu::Menu;
use chrono::*;

#[derive(Properties, Clone, PartialEq)]
struct MonthViewProps {
	year: i32,
	month: u32,
}

#[function_component(MonthView)]
fn month_view(props: &MonthViewProps) -> Html {
	let MonthViewProps { year, month } = props;

	let first_day_of_month = NaiveDate::from_ymd_opt(*year, *month, 1).unwrap();
	let first_weekday = first_day_of_month.weekday();
	let days_in_month = first_day_of_month.with_month(*month + 1).unwrap().pred_opt().unwrap().day() as i32;

	let mut days: Vec<Html> = Vec::new();

	let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
	let start_padding = first_weekday.num_days_from_sunday() as i32;
	let total_slots = days_in_month + start_padding;
	let weeks = (total_slots as f32 / 7.0).ceil() as i32;

	html! {
		<>
			<h1>{format!("{} {}", year, month)}</h1>
			<table>
				<thead>
					<tr>
						{weekdays.iter().map(|weekday| html! {<th>{weekday}</th>}).collect::<Html>()}
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
										html! {<td>{day}</td>}
									} else {
										html! {<td></td>}
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

	html! {
		<div style={"display: flex; margin: 0;"}>
			<Menu />
			<main>
				<MonthView year={*year} month={*month} />
			</main>
		</div>
	}
}

