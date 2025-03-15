#![allow(clippy::must_use_candidate)]

pub mod components;
pub mod errors;
pub mod pages;
pub mod server_functions;
pub mod types;
pub mod utils;

use leptos::prelude::*;
use leptos_meta::{Link, Meta, MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    SsrMode,
    components::{FlatRoutes, Route, Router},
    // hooks::use_location,
    path,
    static_routes::StaticRoute,
};

use crate::{
    components::{Footer, Header},
    pages::{AboutPage, AppLayout, BlogPage, HomePage, PostPage},
    server_functions::list_slugs,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    #[rustfmt::skip]
    view! {
		<!DOCTYPE html>
		<html lang="en" class="scroll-smooth dark" style="color-scheme: dark">
			<head>
				<meta charset="utf-8" />
				<meta name="viewport" content="width=device-width, initial-scale=1" />
				<AutoReload options=options.clone() />
				// https://github.com/leptos-rs/leptos/discussions/3039#discussioncomment-10783691
				<HydrationScripts options />
				<MetaTags />
			</head>
			<body class="antialiased text-black bg-white dark:text-white pl-[calc(100vw-100%)] dark:bg-slate-950">
				<App />
			</body>
		</html>
	}
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let fallback = || view! { "Page not found." }.into_view();

    #[rustfmt::skip]
    view! {
		<Meta name="color-scheme" content="dark light" />
		<Meta name="theme-color" attr:media="(prefers-color-scheme: light)" content="#fff" />
		<Meta name="theme-color" attr:media="(prefers-color-scheme: dark)" content="#000" />
		<Link rel="apple-touch-icon" sizes="180x180" href="/favicons/apple-touch-icon.png" />
		<Link rel="icon" type_="image/png" sizes="32x32" href="/favicons/favicon-32x32.png" />
		<Link rel="icon" type_="image/png" sizes="16x16" href="/favicons/favicon-16x16.png" />
		<Link rel="manifest" href="/favicons/site.webmanifest" />
		<Link rel="shortcut icon" type_="image/x-icon" href="/favicons/favicon.ico" />
		// Google Fonts: Inter
		// <Link rel="preconnect" href="https://fonts.googleapis.com" />
		// <Link rel="preconnect" href="https://fonts.gstatic.com" attr:crossorigin="anonymous" />
		// <Link rel="preload" as_="style" href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap" />
		// <Link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&display=swap" media="print" on:load=move |ev| { event_target::<HtmlLinkElement>(&ev).set_media("all");} />
		<Stylesheet id="leptos" href="/pkg/dande_dev.css" />

		<AppLayout>
			<Router>
				{
					// Scroll to top on route change
					// let location = use_location();
					// let _ = Effect::new(move |_| {
					// let _ = location.pathname.get();
					// web_sys::window().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
					// || {}
					// });

					view! {
						<Header />
						<main class="mb-auto">
							<FlatRoutes fallback>
								<Route
									path=path!("/")
									view=HomePage
									ssr=SsrMode::Static(StaticRoute::new())
								/>

								<Route
									path=path!("/about")
									view=AboutPage
									ssr=SsrMode::Static(StaticRoute::new())
								/>

								<Route
									path=path!("/blog")
									view=BlogPage
									ssr=SsrMode::Static(StaticRoute::new())
								/>

								<Route
									path=path!("/blog/:slug")
									view=PostPage
									ssr=SsrMode::Static(
										StaticRoute::new()
											.prerender_params(|| async move {
												std::iter::once((
														"slug".into(),
														list_slugs().await.unwrap_or_default(),
													))
													.collect()
											}),
									)
								/>
							</FlatRoutes>
						</main>
						<Footer />
					}
				}
			</Router>
		</AppLayout>
	}
}
