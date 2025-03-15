use domain::types::{Post, PostMetadata};
use leptos::prelude::*;

#[server]
pub async fn list_posts() -> Result<Vec<PostMetadata>, ServerFnError> {
    use std::sync::Arc;

    use crate::server_functions::CACHED_ORDERED_POSTS;

    let posts_arc = {
        let guard = CACHED_ORDERED_POSTS.read().unwrap();
        Arc::clone(&guard)
    };

    Ok(posts_arc.to_vec())
}

#[server]
pub async fn list_homepage_posts() -> Result<Vec<PostMetadata>, ServerFnError> {
    use std::sync::Arc;

    use crate::server_functions::CACHED_HOMEPAGE_POSTS;

    let posts_arc = {
        let guard = CACHED_HOMEPAGE_POSTS.read().unwrap();
        Arc::clone(&guard)
    };

    Ok(posts_arc.to_vec())
}

#[server]
pub async fn get_post(slug: String) -> Result<Option<Post>, ServerFnError> {
    use std::sync::Arc;

    use crate::server_functions::CACHED_POSTS;

    let posts_arc = {
        let guard = CACHED_POSTS.read().unwrap();
        Arc::clone(&guard)
    };

    Ok(posts_arc.get(&slug).cloned())
}

#[cfg(feature = "ssr")]
pub fn sort_posts(posts: &mut [Post]) {
    use chrono::NaiveDateTime;
    use domain::constants::DATETIME_FORMAT;

    posts.sort_by(|a, b| {
        let a_date = NaiveDateTime::parse_from_str(&a.metadata.date, DATETIME_FORMAT).unwrap();
        let b_date = NaiveDateTime::parse_from_str(&b.metadata.date, DATETIME_FORMAT).unwrap();

        b_date.cmp(&a_date)
    });
}

#[server]
pub async fn list_slugs() -> Result<Vec<String>, ServerFnError> {
    use std::sync::Arc;

    use crate::server_functions::CACHED_ORDERED_POSTS;

    let posts_arc = {
        let guard = CACHED_ORDERED_POSTS.read().unwrap();
        Arc::clone(&guard)
    };

    Ok(posts_arc.iter().map(|post| post.slug.clone()).collect())
}
