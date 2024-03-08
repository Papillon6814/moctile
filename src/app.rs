use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::calendar::Calendar;
use crate::components::date::Date;
use crate::components::preferences::Preferences;

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

fn switch(route: &Route) -> Html {
	match route {
		Route::Calendar => html! { <Calendar /> },
		Route::Date => html! { <Date /> },
		Route::Preferences => html! { <Preferences /> },
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
