use domain::{data::SITE_METADATA, types::Post};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn PostLayout(post: Post) -> impl IntoView {
    view! {
        <>
            <Title text=format!("{} | {}", post.metadata.title, SITE_METADATA.title) />
            <Meta name="description" content=post.metadata.description />

            <div class="pt-10 pb-8 max-w-none prose dark:prose-invert">
                <h1>{post.metadata.title}</h1>
                <div inner_html=post.content />
            </div>
        </>
    }
}
