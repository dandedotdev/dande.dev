#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use app::{server_functions::initialize_cache, shell};
    use axum::Router;
    use leptos::{logging::log, prelude::*};
    use leptos_axum::{LeptosRoutes, generate_route_list_with_ssg};
    use tower_http::compression::CompressionLayer;

    if let Err(e) = initialize_cache().await {
        log!("Failed to initialize blog cache: {:?}", e);
    }

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let (routes, static_routes) = generate_route_list_with_ssg({
        let leptos_options = leptos_options.clone();
        move || shell(leptos_options.clone())
    });

    static_routes.generate(&leptos_options).await;

    let compression_layer = CompressionLayer::new().br(true);
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(compression_layer)
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub const fn main() {}
