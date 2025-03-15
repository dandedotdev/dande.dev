use leptos::prelude::*;

#[component]
pub fn Tag(text: &'static str) -> impl IntoView {
    #[rustfmt::skip]
    view! {
		<div class="mr-3 text-sm font-medium uppercase text-primary-500 dark:hover:text-primary-400 hover:text-primary-600">
			{text.replace(' ', "-")}
		</div>
	}
}
