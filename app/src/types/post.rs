use leptos::prelude::*;
use leptos_router::params::Params;

#[derive(Clone, Debug, Params, PartialEq)]
pub struct PostParams {
    pub slug: Option<String>,
}
