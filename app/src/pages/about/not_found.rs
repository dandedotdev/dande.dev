use domain::data::SITE_METADATA;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn AboutNotFound() -> impl IntoView {
    view! {
        <>
            <Title text=format!("About | {}", SITE_METADATA.title) />

            <div class="prose dark:prose-invert">
                <h1>"About Page Uninitialized"</h1>
                <p>"Sorry, the about page content was not found."</p>
            </div>
        </>
    }
}
