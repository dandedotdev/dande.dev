#[derive(Debug)]
pub struct HeaderNavLinks<'header> {
    pub href: &'header str,
    pub label: &'header str,
}
