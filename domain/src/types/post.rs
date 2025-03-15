use serde::{Deserialize, Serialize};

type PostContent = String;

const fn default_draft() -> bool {
    true
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostFrontMatter {
    pub canonical_url: Option<String>,
    pub date: String,
    pub description: String,
    #[serde(default = "default_draft")]
    pub draft: bool,
    pub last_modified: Option<String>,
    pub tags: Vec<String>,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostMetadata {
    pub canonical_url: Option<String>,
    pub date: String,
    pub description: String,
    #[serde(default = "default_draft")]
    pub draft: bool,
    pub last_modified: Option<String>,
    pub tags: Vec<String>,
    pub title: String,
    pub slug: String,
}

impl PostMetadata {
    #[must_use]
    pub fn new(
        frontmatter: PostFrontMatter,
        slug: String,
    ) -> Self {
        Self {
            canonical_url: frontmatter.canonical_url,
            date: frontmatter.date,
            description: frontmatter.description,
            draft: frontmatter.draft,
            last_modified: frontmatter.last_modified,
            tags: frontmatter.tags,
            title: frontmatter.title,
            slug,
        }
    }
}

/// the slug is the filename without the .md extension
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Post {
    pub content: PostContent,
    pub metadata: PostMetadata,
}

impl Post {
    #[must_use]
    pub const fn new(
        content: PostContent,
        metadata: PostMetadata,
    ) -> Self {
        Self { content, metadata }
    }
}
