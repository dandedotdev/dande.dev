// TODO: implement the feature

use leptos::prelude::*;

#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    #[rustfmt::skip]
    view! {
		<div class="flex items-center mr-5">
			<div class="inline-block relative text-left">
				<div class="flex justify-center items-center dark:hover:text-primary-400 hover:text-primary-500">
					<button aria-label="Theme switcher">
						<svg
							xmlns="http://www.w3.org/2000/svg"
							viewBox="0 0 20 20"
							fill="currentColor"
							class="size-6 group:hover:text-slate-100"
						>
							<path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
						</svg>
					</button>
				</div>
			</div>
		</div>
	}
}
