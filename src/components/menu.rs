use yew::{function_component, Html, html};
use crate::constants::colors::SENTENCE_BASE;

#[function_component(Menu)]
pub fn menu() -> Html {
	let ul_style = format!("list-style-type: none; padding-top: 20px; padding-bottom: 20px; margin: 0; padding-left: 0;");
	let li_style = format!("padding-left: 20px; padding-top: 10px; padding-bottom: 10px; cursor: pointer;");
	let a_style = format!("font-family: DIN; font-weight: 700; width: 180px; display: flex; flex-direction: row; align-items: center; gap: 10px; text-decoration: none; font-size: 1.1rem; color: {}", SENTENCE_BASE);

	html! {
		<nav>
			<ul style={ul_style.clone()}>
				<li style={li_style.clone()}>
					<a href="/" style={a_style.clone()}>
						<img src="/public/icons/calendar.svg" />
						{"Calendar"}
					</a>
				</li>
				<li style={li_style.clone()}>
					<a href="/about" style={a_style.clone()}>
						<img src="/public/icons/gear.svg" />
						{"Preference"}
					</a>
				</li>
			</ul>
		</nav>
	}
}

