use std::{
    env,
    path::{Path, PathBuf},
};

use crate::constants::path::{
    ABOUT_OUT_DIR, ABOUT_OUT_FILENAME, ABOUT_SRC_DIR, ABOUT_SRC_FILENAME, BLOG_OUT_DIR,
    BLOG_SRC_DIR, GENERATOR_DIR, PROCESS_REHYPE_PRISM_PLUS_FILENAME, ROOT_OUT_DIR,
};

// ===== Generator ===== #
#[must_use]
pub fn absolute_generator_dir() -> PathBuf {
    project_root().join(GENERATOR_DIR)
}

#[must_use]
pub fn absolute_process_rehype_prism_plus_filename() -> PathBuf {
    absolute_generator_dir().join(PROCESS_REHYPE_PRISM_PLUS_FILENAME)
}

// ===== App ===== #
#[must_use]
pub fn absolute_about_out_dir() -> PathBuf {
    project_root().join(ABOUT_OUT_DIR)
}

#[must_use]
pub fn absolute_about_out_filename() -> PathBuf {
    absolute_about_out_dir().join(ABOUT_OUT_FILENAME)
}

#[must_use]
pub fn absolute_about_src_dir() -> PathBuf {
    project_root().join(ABOUT_SRC_DIR)
}

#[must_use]
pub fn absolute_about_src_filename() -> PathBuf {
    absolute_about_src_dir().join(ABOUT_SRC_FILENAME)
}

#[must_use]
pub fn absolute_post_root_dir() -> PathBuf {
    project_root().join(ROOT_OUT_DIR)
}

#[must_use]
pub fn absolute_blog_out_dir() -> PathBuf {
    project_root().join(BLOG_OUT_DIR)
}

#[must_use]
pub fn absolute_blog_src_dir() -> PathBuf {
    project_root().join(BLOG_SRC_DIR)
}

#[must_use]
pub fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf()
}
