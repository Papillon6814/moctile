use yew::{function_component, html};
use crate::components::menu::Menu;

#[function_component(Calendar)]
pub fn calendar() -> Html {
	html! {
		<div style={"display: flex; margin: 0;"}>
			<Menu />
			<main>
			</main>
		</div>
	}
}

