// FIXME: There is a delay in the theme switch (with Leptos SSG). Wait for the
// feature to be implemented by Leptos.

use leptos::{ev::MouseEvent, prelude::*, wasm_bindgen::JsCast};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use web_sys::HtmlElement;

use crate::components::ui::{MoonIcon, SunIcon};

#[derive(Clone, Debug, Deserialize, Display, PartialEq, Serialize)]
#[strum(serialize_all = "lowercase")]
enum Theme {
    Dark,
    Light,
    Unknown,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

#[cfg(feature = "ssr")]
fn get_initial_theme() -> Theme {
    Theme::default()
}

#[cfg(not(feature = "ssr"))]
fn get_initial_theme() -> Theme {
    use domain::constants::THEME_LOCAL_STORAGE_KEY;
    use web_sys::window;

    window()
        .and_then(|win| win.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(THEME_LOCAL_STORAGE_KEY).ok().flatten())
        .map_or(Theme::Dark, |theme| {
            match theme.as_str() {
                "light" => Theme::Light,
                _ => Theme::Dark,
            }
        })
}

#[cfg(feature = "ssr")]
fn set_theme_in_local_storage(_theme_str: &str) {}

#[cfg(not(feature = "ssr"))]
fn set_theme_in_local_storage(theme_str: &str) {
    use domain::constants::THEME_LOCAL_STORAGE_KEY;
    use web_sys::window;

    let window = window().expect("window should exist");

    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item(THEME_LOCAL_STORAGE_KEY, theme_str);
    }
}

#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    let theme = RwSignal::new(Theme::Unknown);

    let on_toggle_theme = move |_: MouseEvent| {
        let html_element = document()
            .document_element()
            .expect("<html> should exist")
            .dyn_into::<HtmlElement>()
            .expect("Should be an HtmlElement");
        let class_list = html_element.class_list();

        match theme.get() {
            Theme::Dark => {
                let _ = class_list.remove_1("dark");
                let _ = html_element.style().set_property("color-scheme", "light");
                set_theme_in_local_storage("light");
                theme.set(Theme::Light);
            },
            Theme::Light => {
                let _ = class_list.add_1("dark");
                let _ = html_element.style().set_property("color-scheme", "dark");
                set_theme_in_local_storage("dark");
                theme.set(Theme::Dark);
            },
            Theme::Unknown => (),
        }
    };

    let _ = Effect::new(move || {
        theme.set(get_initial_theme());
    });

    #[rustfmt::skip]
    view! {
		<div class="flex items-center mr-5">
			<div class="inline-block relative text-left">
				<div class="flex justify-center items-center dark:hover:text-primary-400 hover:text-primary-500">
					<button
						aria-label="Theme switcher"
						disabled=move || theme.get() == Theme::Unknown
						on:click=on_toggle_theme
					>
						{move || {
							match theme.get() {
								Theme::Dark => view! { <MoonIcon /> }.into_any(),
								Theme::Light => view! { <SunIcon /> }.into_any(),
								Theme::Unknown => {
									view! {
										<span class="block invisible text-0 size-6">
											"Theme Unknown"
										</span>
									}
										.into_any()
								}
							}
						}}
					</button>
				</div>
			</div>
		</div>
	}
}
