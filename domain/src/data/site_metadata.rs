use crate::types::SiteMetadata;

pub const SITE_METADATA: SiteMetadata<'_> = SiteMetadata {
    author: "Dandelion Huang",
    avatar_image: "/images/avatar.png",
    description: "Ame ni mo makezu | Miyazawa Kenji",
    email: "contact@dande.dev",
    github_url: "https://github.com/dandedotdev",
    header_title: "Dande.dev",
    // https://www.w3.org/International/articles/language-tags/
    // The golden rule when creating language tags is to keep the tag as short as possible.
    language: "en",
    locale: "en-US",
    linkedin_url: "https://www.linkedin.com/in/dandelion-huang",
    mail_to: "mailto:contact@dande.dev",
    opengraph_image: "/images/opengraph.png",
    repository_url: "https://github.com/dandedotdev/dande.dev",
    site_url: "https://dande.dev",
    title: "Dande.dev",
    twitter_url: "https://x.com/dandelion_huang",
};
