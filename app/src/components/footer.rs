use chrono::{Datelike, Utc};
use domain::data::SITE_METADATA;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::ui::{Icon, IconKind, IconSize};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="flex flex-col items-center mt-16">
            <div class="flex mb-3 space-x-4">
                <Icon kind=IconKind::Email size=IconSize::Sm href=SITE_METADATA.mail_to />
                <Icon kind=IconKind::Github size=IconSize::Sm href=SITE_METADATA.github_url />
                <Icon kind=IconKind::Linkedin size=IconSize::Sm href=SITE_METADATA.linkedin_url />
                <Icon kind=IconKind::Twitter size=IconSize::Sm href=SITE_METADATA.twitter_url />
            </div>
            <div class="flex mb-2 space-x-2 text-sm text-slate-500 dark:text-slate-400">
                <div>{SITE_METADATA.author}</div>
                <div>{" • "}</div>
                <div>{format!("© {}", Utc::now().year())}</div>
                <div>{" • "}</div>
                <A href="/" target="_self">
                    {SITE_METADATA.title}
                </A>
            </div>
        </footer>
    }
}
