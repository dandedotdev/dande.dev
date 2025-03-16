use domain::data::SITE_METADATA;
use icondata as i;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_meta::{Meta, Title};
use leptos_router::components::A;

use crate::{components::ui::Tag, server_functions::list_posts, utils::format_post_date};

#[component]
pub fn BlogPage() -> impl IntoView {
    let posts = Resource::new(|| (), |()| list_posts());

    let posts = move || {
        posts
            .get()
            .map(Result::unwrap_or_default)
            .unwrap_or_default()
    };

    #[rustfmt::skip]
    view! {
		<Meta name="description" content=format!("Blog | {}", SITE_METADATA.title) />
		<Meta property="og:title" content=format!("Blog | {}", SITE_METADATA.title) />
		<Meta property="og:type" content="website" />
		<Meta property="og:url" content=format!("{}/blog", SITE_METADATA.site_url) />
		<Meta name="twitter:description" content=format!("Blog | {}", SITE_METADATA.title) />
		<Meta name="twitter:title" content=format!("Blog | {}", SITE_METADATA.title) />
		<Title text=format!("Blog | {}", SITE_METADATA.title) />

		<div class="px-4 mx-auto max-w-4xl sm:px-6 xl:px-0">
			<div class="pt-16 pb-12 space-y-8">
				<div class="space-y-2">
					<h1 class="text-3xl font-bold sm:text-4xl text-slate-900 dark:text-slate-100">
						"All Posts"
					</h1>
				</div>

				<div class="flex flex-col gap-8 md:flex-row">
					<div class="flex-1 min-w-0">
						// FIXME: improve loading a show loading state (but the user should never see it)
						<Suspense fallback=move || {
							view! { <p class="hidden">"Loading posts..."</p> }
						}>
							<ul class="divide-y divide-slate-200 dark:divide-slate-800">
								<For each=posts key=|post| post.slug.clone() let:post>
									{
										let tags = post.tags.clone();
										let title = post.title.clone();

										view! {
											<li class="py-12 first:pt-0">
												<article>
													<div class="space-y-8">
														<div class="space-y-4">
															<div class="flex flex-col gap-4 text-sm text-slate-500 dark:text-slate-300">
																<time datetime=post
																	.date>{format_post_date(&post.date)}</time>
																<h2 class="text-2xl font-semibold text-slate-900 dark:text-slate-100">
																	<A href=format!("/blog/{}", post.slug) target="_self">
																		{title}
																	</A>
																</h2>
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
															<p class="text-slate-600 dark:text-slate-300">
																{post.description}
															</p>
														</div>
														<A
															attr:aria-label=format!("Read {}", post.title)
															attr:class="inline-flex items-center text-sm transition-colors text-slate-600 group dark:text-slate-400 dark:hover:text-slate-100 hover:text-slate-900"
															href=format!("/blog/{}", post.slug)
															target="_self"
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
				</div>
			</div>
		</div>
	}
}
