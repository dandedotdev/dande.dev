use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

use domain::types::{About, Post, PostMetadata};
use leptos::prelude::*;

pub static CACHED_ABOUT: LazyLock<RwLock<Arc<Option<About>>>> =
    LazyLock::new(|| RwLock::new(Arc::new(None)));

pub static CACHED_POSTS: LazyLock<RwLock<Arc<HashMap<String, Post>>>> =
    LazyLock::new(|| RwLock::new(Arc::new(HashMap::new())));

pub static CACHED_ORDERED_POSTS: LazyLock<RwLock<Arc<Vec<PostMetadata>>>> =
    LazyLock::new(|| RwLock::new(Arc::new(Vec::new())));

pub static CACHED_HOMEPAGE_POSTS: LazyLock<RwLock<Arc<Vec<PostMetadata>>>> =
    LazyLock::new(|| RwLock::new(Arc::new(Vec::new())));

#[server]
pub async fn initialize_cache() -> Result<(), ServerFnError> {
    use domain::{
        constants::MAX_POSTS_PER_PAGE,
        data::{absolute_about_out_filename, absolute_blog_out_dir},
    };
    use futures::TryStreamExt;
    use tokio::fs;
    use tokio_stream::wrappers::ReadDirStream;

    use crate::server_functions::sort_posts;

    let about_path = absolute_about_out_filename();

    match fs::read_to_string(&about_path).await {
        Ok(content) =>
            match serde_json::from_str::<About>(&content) {
                Ok(about) => {
                    let mut about_cache = CACHED_ABOUT.write().unwrap();
                    *about_cache = Arc::new(Some(about));
                },
                Err(e) => {
                    eprintln!(
                        "Error parsing About JSON file {}: {}",
                        about_path.display(),
                        e
                    );
                    return Err(ServerFnError::ServerError(format!(
                        "Failed to parse About JSON: {}",
                        e
                    )));
                },
            },
        Err(e) => {
            eprintln!("Error reading About file {}: {}", about_path.display(), e);
            return Err(ServerFnError::ServerError(format!(
                "Failed to read About file: {}",
                e
            )));
        },
    }

    let blog_out_dir = absolute_blog_out_dir();

    let files = ReadDirStream::new(fs::read_dir(&blog_out_dir).await?);

    let mut posts: Vec<Post> = files
        .try_filter_map(|entry| {
            async move {
                let path = entry.path();

                if !path.is_file() {
                    return Ok(None);
                }

                let Some(extension) = path.extension() else {
                    return Ok(None);
                };

                if extension != "json" {
                    return Ok(None);
                }

                let content = fs::read_to_string(&path).await?;

                match serde_json::from_str::<Post>(&content) {
                    Ok(post) => Ok(Some(post)),
                    Err(e) => {
                        eprintln!("Error parsing JSON file {}: {}", path.display(), e);
                        Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Failed to parse JSON: {}", e),
                        ))
                    },
                }
            }
        })
        .try_collect()
        .await?;

    sort_posts(&mut posts);

    let ordered_metadata = posts
        .iter()
        .map(|post| post.metadata.clone())
        .collect::<Vec<_>>();

    let posts_map = posts
        .into_iter()
        .map(|post| (post.metadata.slug.clone(), post))
        .collect::<HashMap<_, _>>();

    {
        let mut ordered = CACHED_ORDERED_POSTS.write().unwrap();
        *ordered = Arc::new(ordered_metadata.clone());
    }
    {
        let mut homepage_ordered = CACHED_HOMEPAGE_POSTS.write().unwrap();
        *homepage_ordered = Arc::new(
            ordered_metadata
                .iter()
                .take(MAX_POSTS_PER_PAGE)
                .cloned()
                .collect(),
        );
    }
    {
        let mut lookup = CACHED_POSTS.write().unwrap();
        *lookup = Arc::new(posts_map);
    }

    Ok(())
}
