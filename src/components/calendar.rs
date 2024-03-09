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

	let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

	html! {
		<>
			<h1>{format!("{} {}", year, month)}</h1>
			<table>
				<thead>
					<tr>
					{weekdays.iter().map(|weekday| html! {<th>{weekday}</th>}).collect::<Html>()}
					</tr>
				</thead>
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

