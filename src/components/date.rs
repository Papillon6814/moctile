use yew::{function_component, html};
use crate::components::menu::Menu;

#[function_component(Date)]
pub fn date() -> Html {
	html! {
		<div style={"display: flex; margin: 0;"}>
			<Menu />
			<main>
				{"Date"}
			</main>
		</div>
	}
}
