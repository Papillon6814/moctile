use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::menu::Menu;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
	#[at("/")]
	Calendar,
	#[at("/date")]
	Date,
	#[at("/preferences")]
	Preferences,
}

#[function_component(Calendar)]
fn calendar() -> Html {
	html! {
		<div style={"display: flex; margin: 0;"}>
			<Menu />
			<main>
			</main>
		</div>
	}
}

fn switch(route: &Route) -> Html {
	match route {
		Route::Calendar => html! { <Calendar /> },
		Route::Date => html! { <div>{"Date"}</div> },
		Route::Preferences => html! { <div>{"Preferences"}</div> },
	}
}

#[function_component(App)]
pub fn app() -> Html {
	html! {
		<BrowserRouter>
			<Switch<Route> render={Switch::render(switch)} />
		</BrowserRouter>
	}
}
