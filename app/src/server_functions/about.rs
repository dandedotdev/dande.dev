use domain::types::About;
use leptos::prelude::*;

#[server]
pub async fn get_about() -> Result<Option<About>, ServerFnError> {
    use std::sync::Arc;

    use crate::server_functions::CACHED_ABOUT;

    let about_arc = {
        let guard = CACHED_ABOUT.read().unwrap();
        Arc::clone(&guard)
    };

    Ok(about_arc.as_ref().clone())
}
