use serde::{Deserialize, Serialize};

type AboutContent = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct AboutFrontMatter {
    pub avatar_image: String,
    pub company: String,
    pub email: String,
    pub github_url: String,
    pub linkedin_url: String,
    pub name: String,
    pub occupation: String,
    pub twitter_url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AboutMetadata {
    pub avatar_image: String,
    pub company: String,
    pub email: String,
    pub github_url: String,
    pub linkedin_url: String,
    pub name: String,
    pub occupation: String,
    pub twitter_url: String,
}

impl AboutMetadata {
    #[must_use]
    pub fn new(frontmatter: AboutFrontMatter) -> Self {
        Self {
            avatar_image: frontmatter.avatar_image,
            company: frontmatter.company,
            email: frontmatter.email,
            github_url: frontmatter.github_url,
            linkedin_url: frontmatter.linkedin_url,
            name: frontmatter.name,
            occupation: frontmatter.occupation,
            twitter_url: frontmatter.twitter_url,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct About {
    pub content: AboutContent,
    pub metadata: AboutMetadata,
}

impl About {
    #[must_use]
    pub const fn new(
        content: AboutContent,
        metadata: AboutMetadata,
    ) -> Self {
        Self { content, metadata }
    }
}
