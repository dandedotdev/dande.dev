#![allow(clippy::expl_impl_clone_on_copy)]

use leptos::prelude::*;
use leptos_router::components::A;
use tailwind_fuse::{
    AsTailwindClass, IntoBuilder, IntoTailwindClass, TailwindFuse, TailwindMerge, TwClass,
    TwVariant,
};

use super::{EmailIcon, GithubIcon, IconKind, LinkedinIcon, TwitterIcon};

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, TwClass)]
#[tw(
    class = r#"text-slate-700 fill-current dark:text-slate-200 size-16 dark:hover:text-primary-400 hover:text-primary-500"#
)]
pub struct IconClass {
    pub size: IconSize,
}

#[derive(Debug, TwVariant)]
pub enum IconSize {
    #[tw(default, class = "size-8")]
    Default,
    #[tw(class = "size-6")]
    Sm,
}

#[component]
pub fn Icon(
    href: &'static str,
    kind: IconKind,
    #[prop(into, optional)] size: Signal<IconSize>,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        let size = size.get();
        let icon = IconClass { size };
        icon.with_class("")
    });

    view! {
        <A href=href target="_self">
            <span class="sr-only">{format!("{kind:?}")}</span>
            {move || match kind {
                IconKind::Email => view! { <EmailIcon attr:class=class /> }.into_any(),
                IconKind::Github => view! { <GithubIcon attr:class=class /> }.into_any(),
                IconKind::Linkedin => view! { <LinkedinIcon attr:class=class /> }.into_any(),
                IconKind::Twitter => view! { <TwitterIcon attr:class=class /> }.into_any(),
            }}
        </A>
    }
}
