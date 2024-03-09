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
	let days_in_month = first_day_of_month.with_month(*month + 1).unwrap().pred_opt().unwrap().day();

	let mut days: Vec<Html> = Vec::new();
	let mut day_count = 1;

	let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

	for _ in 0..first_weekday.num_days_from_sunday() {
		days.push(html! {<td></td>});
	}

	while day_count <= days_in_month {
		days.push(html! {<td>{day_count}</td>});
		day_count += 1;
	}

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
					<tr>
						{for days.into_iter()}
					</tr>
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

