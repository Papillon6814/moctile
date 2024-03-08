use yew::{function_component, Html, html};

#[function_component(Menu)]
pub fn menu() -> Html {
	html! {
		<nav class="menu">
			<ul>
				<li><a href="/">{"Home"}</a></li>
				<li><a href="/about">{"About"}</a></li>
				<li><a href="/contact">{"Contact"}</a></li>
			</ul>
		</nav>
	}
}

