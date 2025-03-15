#![allow(clippy::too_many_lines)]

use leptos::prelude::*;

use super::{AboutLayout, AboutNotFound};
use crate::{errors::AboutError, server_functions::get_about};

#[component]
pub fn AboutPage() -> impl IntoView {
    let about_resource = Resource::new(|| (), |()| get_about());

    let about_view = move || {
        Suspend::new(async move {
            match about_resource.await {
                Ok(Some(about)) => Ok(view! { <AboutLayout about=about /> }.into_any()),
                Ok(None) => Ok(view! { <AboutNotFound /> }.into_any()),
                Err(e) => Err(AboutError::ServerError(e.to_string())),
            }
        })
    };

    view! {
        // FIXME: improve loading a show loading state (but the user should never see it)
        <Suspense fallback=move || view! { <p class="hidden">"Loading about..."</p> }>
            <ErrorBoundary fallback=|errors| {
                #[cfg(feature = "ssr")]
                expect_context::<leptos_axum::ResponseOptions>()
                    .set_status(http::StatusCode::NOT_FOUND);

                view! {
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, error)| view! { <li>{error.to_string()}</li> })
                                    .collect::<Vec<_>>()
                            }}
                        </ul>
                    </div>
                }
            }>{about_view}</ErrorBoundary>
        </Suspense>
    }
}
