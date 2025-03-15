use leptos::prelude::*;
use leptos_meta::Stylesheet;
use leptos_router::hooks::use_params;

use super::PostLayout;
use crate::{errors::PostError, server_functions::get_post, types::PostParams};

#[component]
pub fn PostPage() -> impl IntoView {
    let query = use_params::<PostParams>();

    let slug = move || {
        query
            .get()
            .map(|q| q.slug.unwrap_or_default())
            .map_err(|_| PostError::InvalidId)
    };

    let post_resource = Resource::new(slug, |slug| {
        async move {
            match slug {
                Err(e) => Err(e),
                Ok(slug) =>
                    get_post(slug)
                        .await
                        .map(|data| data.ok_or(PostError::PostNotFound))
                        .map_err(|e| PostError::ServerError(e.to_string())),
            }
        }
    });
    let post_view = move || {
        Suspend::new(async move {
            match post_resource.await {
                Ok(Ok(post)) => Ok(view! { <PostLayout post=post /> }),
                Ok(Err(e)) | Err(e) => Err(PostError::ServerError(e.to_string())),
            }
        })
    };

    view! {
        <>
            <Stylesheet href="/styles/prism.css" />

            // FIXME: improve loading a show loading state (but the user should never see it)
            <Suspense fallback=move || view! { <p class="hidden">"Loading post..."</p> }>
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
                }>{post_view}</ErrorBoundary>
            </Suspense>
        </>
    }
}
