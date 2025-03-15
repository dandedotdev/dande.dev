use leptos::prelude::*;

#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <div class="px-4 mx-auto max-w-3xl sm:px-6 xl:px-0 xl:max-w-5xl">
            <div class="flex flex-col justify-between h-screen">{children()}</div>
        </div>
    }
}
