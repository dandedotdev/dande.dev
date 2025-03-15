use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Error, PartialEq, Serialize)]
pub enum AboutError {
    #[error("Post not found.")]
    PostNotFound,
    #[error("Server error: {0}.")]
    ServerError(String),
}
