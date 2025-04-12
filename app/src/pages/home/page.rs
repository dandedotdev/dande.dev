use domain::data::{HOMEPAGE_INTRO, SITE_METADATA};
use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_meta::{Meta, Title};
use leptos_router::components::A;

use crate::{components::ui::Tag, server_functions::list_homepage_posts, utils::format_post_date};

#[component]
pub fn HomePage() -> impl IntoView {
    let posts = Resource::new(|| (), |()| list_homepage_posts());

    let posts = move || {
        posts
            .get()
            .map(Result::unwrap_or_default)
            .unwrap_or_default()
    };

    #[rustfmt::skip]
    view! {
		<Meta name="description" content=SITE_METADATA.description />
		<Meta property="og:title" content=SITE_METADATA.title />
		<Meta property="og:type" content="website" />
		<Meta property="og:url" content=SITE_METADATA.site_url />
		<Meta name="twitter:description" content=SITE_METADATA.description />
		<Meta name="twitter:title" content=SITE_METADATA.title />
		<Title text=SITE_METADATA.title />

		<div class="flex flex-col gap-x-12 my-6 lg:flex-row lg:mb-12">
			<div class="flex flex-col justify-start items-start space-y-6 md:flex-row md:justify-center md:items-center md:mt-24 md:space-x-6 md:divide-y-0">
				<div class="space-y-4 md:border-r-2 md:border-slate-200 dark:md:border-slate-700">
					<h1 class="text-3xl font-bold sm:text-4xl text-slate-900 dark:text-slate-100">
						{SITE_METADATA.author}
					</h1>
					<p class="mr-2 text-sm tracking-wider uppercase md:w-96 text-primary-500">
						{SITE_METADATA.description}
					</p>
				</div>
				<div
					class="space-y-4 max-w-xl text-slate-600 dark:text-slate-400"
					inner_html=HOMEPAGE_INTRO
				></div>
			</div>
		</div>

		<div class="divide-y divide-slate-200 dark:divide-slate-700">
			<div class="pt-6 pb-8 space-y-2 md:space-y-5">
				<h2 class="font-mono text-sm tracking-wider uppercase text-slate-500 dark:text-slate-400">
					"Latest Writing"
				</h2>
			</div>

			// FIXME: improve loading a show loading state (but the user should never see it)
			<Suspense fallback=move || view! { <p class="hidden">"Loading posts..."</p> }>
				<ul class="divide-y divide-slate-200 dark:divide-slate-700">
					<For each=posts key=|post| post.slug.clone() let:post>
						{
							let tags = post.tags.clone();
							let title = post.title.clone();

							view! {
								<li class="py-12">
									<article>
										<div class="space-y-8">
											<div class="space-y-4">
												<div class="flex flex-col gap-4 text-sm text-slate-500 dark:text-slate-400">
													<time datetime=post
														.date>{format_post_date(&post.date)}</time>
													<h3 class="text-2xl font-semibold text-slate-900 dark:text-slate-100">
														<A
															attr:class="break-words"
															href=format!("/blog/{}", post.slug)
														>
															{title}
														</A>
													</h3>
													<div class="flex flex-wrap gap-2">
														<For
															each=move || tags.clone()
															key=|tag| tag.clone()
															let:tag
														>
															{
																let tag_static = tag.leak();
																// https://doc.rust-lang.org/nightly/alloc/string/struct.String.html#method.leak

																view! { <Tag text=tag_static /> }
															}
														</For>
													</div>
												</div>
												<p class="max-w-4xl text-slate-600 dark:text-slate-400">
													{post.description}
												</p>
											</div>
											<A
												attr:aria-label=format!("Read {}", post.title)
												attr:class="inline-flex items-center text-sm transition-colors text-slate-600 group dark:text-slate-400 dark:hover:text-slate-100 hover:text-slate-900"
												href=format!("/blog/{}", post.slug)
											>
												"Read article"
												<Icon
													icon=i::LuArrowRight
													attr:class="ml-1 size-4 transition-transform group-hover:translate-x-0.5"
												/>
											</A>
										</div>
									</article>
								</li>
							}
						}
					</For>
				</ul>
			</Suspense>
		</div>
	}
}
