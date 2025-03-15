// TODO:
// - encapsulate these server_functions
// - make them generic
// - use `build.rs` here to generate rust code to get rid of `Suspense` of the
//   frontend

pub mod add_default_language;
pub mod add_gfm_code_title;
pub mod add_relative_div;
pub mod generate_about;
pub mod generate_posts;
pub mod process_html_with_prism;

pub use add_default_language::add_default_language;
pub use add_gfm_code_title::add_gfm_code_title;
pub use add_relative_div::add_relative_div;
pub use generate_about::generate_about;
pub use generate_posts::generate_posts;
pub use process_html_with_prism::process_html_with_prism;
