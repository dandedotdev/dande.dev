use domain::{data::SITE_METADATA, types::Post};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn PostLayout(post: Post) -> impl IntoView {
    view! {
        <>
            <Meta name="description" content=post.metadata.description.clone() />
            <Meta
                property="og:title"
                content=format!("{} | {}", post.metadata.title, SITE_METADATA.title)
            />
            <Meta property="og:type" content="article" />
            <Meta
                property="og:url"
                content=format!("{}/blog/{}", SITE_METADATA.site_url, post.metadata.slug)
            />
            <Meta name="twitter:description" content=post.metadata.description />
            <Meta
                name="twitter:title"
                content=format!("{} | {}", post.metadata.title, SITE_METADATA.title)
            />
            <Title text=format!("{} | {}", post.metadata.title, SITE_METADATA.title) />

            <div class="pt-10 pb-8 max-w-none prose dark:prose-invert">
                <h1>{post.metadata.title}</h1>
                <div inner_html=post.content />
            </div>
        </>
    }
}
