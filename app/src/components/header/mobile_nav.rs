// TODO: add transitions

use domain::data::HEADER_NAV_LINKS;
use leptos::{
    ev::MouseEvent, leptos_dom::helpers::set_timeout, portal::Portal, prelude::*,
    wasm_bindgen::JsCast,
};
use leptos_router::components::A;
use web_sys::HtmlElement;

#[component]
pub fn MobileNav() -> impl IntoView {
    let (is_shown, set_is_shown) = signal(false);

    let on_toggle_nav = move |_: MouseEvent| {
        let html_element = document()
            .document_element()
            .expect("<html> should exist")
            .dyn_into::<HtmlElement>()
            .expect("Should be an HtmlElement");

        set_is_shown.update(|status| {
            if *status {
                let _ = html_element.style().remove_property("overflow");
                let _ = html_element.style().remove_property("padding-right");
            } else {
                let _ = html_element.style().set_property("overflow", "hidden");
                let _ = html_element.style().set_property("padding-right", "0");
            }
            *status = !*status;
        });
    };

    // FIXME: the better way is prefetch for `<A>`
    let delayed_toggle_nav = move |ev: MouseEvent| {
        set_timeout(
            move || on_toggle_nav(ev),
            std::time::Duration::from_millis(100),
        );
    };

    #[rustfmt::skip]
    view! {
		<>
			<button aria-label="Toggle Menu" class="sm:hidden" on:click=on_toggle_nav>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 20 20"
					fill="currentColor"
					class="size-8 text-slate-900 dark:text-slate-100 dark:hover:text-primary-400 hover:text-primary-500"
				>
					<path
						fill-rule="evenodd"
						d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
						clip-rule="evenodd"
					/>
				</svg>
			</button>

			<Show when=is_shown fallback=|| ()>
				<Portal mount=document().body().unwrap()>
					<div class="fixed inset-0 bg-opacity-50 z-60 bg-black/25">
						<div class="fixed top-0 left-0 w-full h-full bg-white opacity-95 duration-300 translate-x-0 z-70 dark:bg-slate-950 dark:opacity-98">
							<nav class="flex overflow-y-auto flex-col items-start pt-2 pl-12 mt-8 w-full h-full text-left basis-0">
								<For
									each=move || HEADER_NAV_LINKS.iter()
									key=|link| link.label
									children=move |link| {
										view! {
											<A
												attr:class="mb-4 py-2 pr-4 text-2xl font-bold tracking-widest text-slate-900 outline-0 hover:text-primary-500 dark:text-slate-100 dark:hover:text-primary-400"
												href=link.href
												on:click=delayed_toggle_nav
											>
												{link.label}
											</A>
										}
									}
								/>
							</nav>
						</div>

						<button
							aria-label="Toggle Menu"
							class="fixed right-4 top-7 p-4 z-80 size-16 text-slate-900 dark:text-slate-100 dark:hover:text-primary-400 hover:text-primary-500"
							on:click=on_toggle_nav
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								viewBox="0 0 20 20"
								fill="currentColor"
							>
								<path
									fill-rule="evenodd"
									d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
									clip-rule="evenodd"
								/>
							</svg>
						</button>
					</div>
				</Portal>
			</Show>
		</>
	}
}
