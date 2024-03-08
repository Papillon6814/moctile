use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::components::menu::Menu;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
		<div style={"display: flex;"}>
			<Menu />
			<main>
			</main>
		</div>
    }
}
