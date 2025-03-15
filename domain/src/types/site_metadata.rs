// https://www.possiblerust.com/pattern/naming-your-lifetimes

#[derive(Debug)]
pub struct SiteMetadata<'site> {
    pub author: &'site str,
    pub avatar_image: &'site str,
    pub description: &'site str,
    pub email: &'site str,
    pub github_url: &'site str,
    pub header_title: &'site str,
    pub language: &'site str,
    pub locale: &'site str,
    pub linkedin_url: &'site str,
    pub mail_to: &'site str,
    pub opengraph_image: &'site str,
    pub repository_url: &'site str,
    pub site_url: &'site str,
    pub title: &'site str,
    pub twitter_url: &'site str,
}
