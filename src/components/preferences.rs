use yew::{function_component, html, Html};
use crate::components::menu::Menu;

#[function_component(Preferences)]
pub fn date() -> Html {
	html! {
		<div style={"display: flex; margin: 0;"}>
			<Menu />
			<main>
				{"Preferences"}
			</main>
		</div>
	}
}

