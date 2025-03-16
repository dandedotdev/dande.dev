use domain::{data::SITE_METADATA, types::About};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::ui::{SocialIcon, SocialIconKind, SocialIconSize};

#[component]
pub fn AboutLayout(about: About) -> impl IntoView {
    #[rustfmt::skip]
    view! {
		<>
			<Meta name="description" content=SITE_METADATA.description />
			<Meta property="og:title" content=format!("About | {}", SITE_METADATA.title) />
			<Meta property="og:type" content="website" />
			<Meta property="og:url" content=format!("{}/about", SITE_METADATA.site_url) />
			<Meta name="twitter:description" content=SITE_METADATA.description />
			<Meta name="twitter:title" content=format!("About | {}", SITE_METADATA.title) />
			<Title text=format!("About | {}", SITE_METADATA.title) />

			<div class="px-4 mx-auto max-w-4xl sm:px-6 xl:px-0">
				<div class="pt-16 pb-12 space-y-8">
					<div class="space-y-2">
						<h1 class="text-2xl font-bold sm:text-4xl text-slate-900 dark:text-slate-100">
							"About"
						</h1>
					</div>
					<div class="grid gap-12 pt-4 md:grid-cols-7">
						<div class="flex flex-row gap-8 items-start md:flex-col md:col-span-2">
							<img
								alt="avatar"
								class="overflow-hidden rounded-lg bg-slate-100 w-[150px] lg:w-[200px] dark:bg-slate-800"
								src=about.metadata.avatar_image.to_string()
							/>
							<div class="space-y-4">
								<div class="space-y-2">
									<h2 class="text-2xl font-semibold text-slate-900 dark:text-slate-100">
										{about.metadata.name}
									</h2>
									<div class="space-y-1">
										<p class="font-mono text-sm uppercase text-slate-600 dark:text-slate-400">
											{about.metadata.occupation}
										</p>
										<p class="font-mono text-sm uppercase text-slate-600 dark:text-slate-400">
											{about.metadata.company}
										</p>
									</div>
								</div>
								<div class="flex gap-4">
									<SocialIcon
										kind=SocialIconKind::Email
										size=SocialIconSize::Sm
										href=SITE_METADATA.mail_to
									/>
									<SocialIcon
										kind=SocialIconKind::Github
										size=SocialIconSize::Sm
										href=SITE_METADATA.github_url
									/>
									<SocialIcon
										kind=SocialIconKind::Linkedin
										size=SocialIconSize::Sm
										href=SITE_METADATA.linkedin_url
									/>
									<SocialIcon
										kind=SocialIconKind::Twitter
										size=SocialIconSize::Sm
										href=SITE_METADATA.twitter_url
									/>
								</div>
							</div>
						</div>
						<div
							class="max-w-none md:col-span-5 prose prose-slate dark:prose-invert"
							inner_html=about.content
						/>
					</div>
				</div>
			</div>
		</>
	}
}
