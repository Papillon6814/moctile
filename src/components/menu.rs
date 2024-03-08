use yew::{function_component, Html, html};
use crate::constants::colors::SENTENCE_BASE;

#[function_component(Menu)]
pub fn menu() -> Html {
	let a_style = format!("text-decoration: none; font-size: 1.2rem; color: {}", SENTENCE_BASE);

	html! {
		<nav>
			<ul style={"list-style-type: none;"}>
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

