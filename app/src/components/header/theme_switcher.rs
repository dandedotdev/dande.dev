// TODO: implement the feature

use leptos::prelude::*;

use crate::components::ui::MoonIcon;

#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    #[rustfmt::skip]
    view! {
		<div class="flex items-center mr-5">
			<div class="inline-block relative text-left">
				<div class="flex justify-center items-center">
					<button aria-label="Theme switcher" disabled>
						<MoonIcon />
					</button>
				</div>
			</div>
		</div>
	}
}
