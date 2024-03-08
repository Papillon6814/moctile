use yew::{function_component, Html, html};
use crate::constants::colors::SENTENCE_BASE;

#[function_component(Menu)]
pub fn menu() -> Html {
	let nav_style = format!("background-color: green; display: flex; flex-direction: column; align-items: center;");
	let ul_style = format!("list-style-type: none; padding: 20px; margin: 0;");
	let a_style = format!("font-family: DIN; font-weight: 700; width: 180px; display: flex; flex-direction: row; align-items: center; gap: 10px; text-decoration: none; font-size: 1.2rem; color: {}", SENTENCE_BASE);

	html! {
		<nav style={nav_style.clone()}>
			<ul style={ul_style.clone()}>
				<li>
					<a href="/" style={a_style.clone()}>
						<img src="/public/icons/calendar.svg" />
						{"Calendar"}
					</a>
				</li>
				<li>
					<a href="/about" style={a_style.clone()}>
						<img src="/public/icons/gear.svg" />
						{"Preference"}
					</a>
				</li>
			</ul>
		</nav>
	}
}

