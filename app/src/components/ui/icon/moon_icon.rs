use leptos::prelude::*;

#[component]
pub fn MoonIcon() -> impl IntoView {
    #[rustfmt::skip]
    view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 20 20"
			fill="currentColor"
			class="group:hover:text-slate-100 size-6"
		>
			<path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
		</svg>
	}
}
