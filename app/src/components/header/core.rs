use domain::data::{HEADER_NAV_LINKS, SITE_METADATA};
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::{Logo, MobileNav, /* SearchButton, */ ThemeSwitcher};

#[component]
pub fn Header() -> impl IntoView {
    #[rustfmt::skip]
    view! {
		<header class="flex sticky top-0 z-50 justify-between items-center py-10 w-full bg-white dark:bg-slate-950">
			<A attr:aria_label=SITE_METADATA.header_title attr:class="break-words" href="/">
				<div class="flex justify-between items-center">
					<div class="mr-3">
						<Logo />
					</div>
					<div class="hidden h-6 text-2xl font-semibold sm:block">
						{SITE_METADATA.header_title}
					</div>
				</div>
			</A>
			<div class="flex items-center space-x-4 leading-5 sm:space-x-6">
				<nav class="hidden items-center space-x-4 sm:flex sm:space-x-6 max-w-72 lg:max-w-96">
					<For
						each=|| HEADER_NAV_LINKS
						key=|link| link.label
						children=move |link| {
							view! {
								<A
									attr:class="block font-medium text-slate-900 dark:text-slate-100 dark:hover:text-primary-400 hover:text-primary-500"
									href=link.href
								>
									{link.label}
								</A>
							}
						}
					/>
				</nav>
				<ThemeSwitcher />
				// <SearchButton />
				<MobileNav />
			</div>
		</header>
	}
}
