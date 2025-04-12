#![allow(clippy::expl_impl_clone_on_copy)]

use leptos::prelude::*;
use leptos_router::components::A;
use tailwind_fuse::{
    AsTailwindClass, IntoBuilder, IntoTailwindClass, TailwindFuse, TailwindMerge, TwClass,
    TwVariant,
};

use super::{EmailIcon, GithubIcon, LinkedinIcon, TwitterIcon};

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, TwClass)]
#[tw(
    class = r#"text-slate-700 fill-current dark:text-slate-200 size-16 dark:hover:text-primary-400 hover:text-primary-500"#
)]
pub struct SocialIconClass {
    pub size: SocialIconSize,
}

#[derive(Debug, TwVariant)]
pub enum SocialIconSize {
    #[tw(default, class = "size-8")]
    Default,
    #[tw(class = "size-6")]
    Sm,
}

#[derive(Debug)]
pub enum SocialIconKind {
    Email,
    Github,
    Linkedin,
    Twitter,
}

#[component]
pub fn SocialIcon(
    href: &'static str,
    kind: SocialIconKind,
    #[prop(into, optional)] size: Signal<SocialIconSize>,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        let size = size.get();
        let icon = SocialIconClass { size };
        icon.with_class("")
    });

    view! {
        <A href=href>
            <span class="sr-only">{format!("{kind:?}")}</span>
            {move || match kind {
                SocialIconKind::Email => view! { <EmailIcon attr:class=class /> }.into_any(),
                SocialIconKind::Github => view! { <GithubIcon attr:class=class /> }.into_any(),
                SocialIconKind::Linkedin => view! { <LinkedinIcon attr:class=class /> }.into_any(),
                SocialIconKind::Twitter => view! { <TwitterIcon attr:class=class /> }.into_any(),
            }}
        </A>
    }
}
