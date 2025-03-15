use std::sync::OnceLock;

use regex::Regex;

static CODE_TITLE_REGEX: OnceLock<Regex> = OnceLock::new();

#[rustfmt::skip]
fn get_code_title_regex() -> &'static Regex {
    CODE_TITLE_REGEX.get_or_init(|| {
        Regex::new(r#"(<div class=(\\"|")relative(\\"|")><pre><code class=(\\"|")language-([^:]+):([^"\\]+)(\\"|")>)"#)
            .unwrap()
    })
}

#[rustfmt::skip]
#[must_use]
pub fn add_gfm_code_title(html: &str) -> String {
    get_code_title_regex()
        .replace_all(html, |caps: &regex::Captures<'_>| {
            let language = &caps[5];
            let filename = &caps[6];
            let quote_type = if caps[2].contains('\\') { "\\\"" } else { "\"" };

            format!(
                r#"<div class="remark-code-title">{filename}</div><div class={quote_type}relative{quote_type}><pre><code class={quote_type}language-{language}{quote_type}>"#
            )
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn test_basic_extraction() {
        let input = r#"<div class="relative"><pre><code class="language-rust:main.rs">fn main() {}</code></pre></div>"#;
        let expected = r#"<div class="remark-code-title">main.rs</div><div class="relative"><pre><code class="language-rust">fn main() {}</code></pre></div>"#;
        assert_eq!(add_gfm_code_title(input), expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_complex_filepath() {
        let input = r#"<div class="relative"><pre><code class="language-js:src/components/Button.jsx">export default function Button() {}</code></pre></div>"#;
        let expected = r#"<div class="remark-code-title">src/components/Button.jsx</div><div class="relative"><pre><code class="language-js">export default function Button() {}</code></pre></div>"#;
        assert_eq!(add_gfm_code_title(input), expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_multiple_code_blocks() {
        let input = r#"
        <div class="relative"><pre><code class="language-rust:lib.rs">pub fn add(a: i32, b: i32) -> i32 { a + b }</code></pre></div>
        <p>Some text</p>
        <div class="relative"><pre><code class="language-js:app.js">console.log('Hello');</code></pre></div>
        "#;
        let expected = r#"
        <div class="remark-code-title">lib.rs</div><div class="relative"><pre><code class="language-rust">pub fn add(a: i32, b: i32) -> i32 { a + b }</code></pre></div>
        <p>Some text</p>
        <div class="remark-code-title">app.js</div><div class="relative"><pre><code class="language-js">console.log('Hello');</code></pre></div>
        "#;
        assert_eq!(add_gfm_code_title(input), expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_mixed_code_blocks_with_html() {
        let input = r#"<h2>Syntax highlighting</h2>\n<p>Here's an example of how you can use syntax highlighting with <a href=\"https://help.github.com/articles/basic-writing-and-formatting-syntax/\">GitHub Flavored Markdown</a>:</p>\n<h2>Footnotes</h2>\n<div class=\"relative\"><pre><code class=\"language-rust:test.rs\">fn main() {\n    use domain::data::{\n        absolute_about_out_dir, absolute_blog_out_dir, absolute_blog_src_dir,\n        absolute_post_root_dir,\n    };\n\n    let about_out_dir = absolute_about_out_dir();\n    let blog_out_dir = absolute_blog_out_dir();\n    let blog_src_dir = absolute_blog_src_dir();\n    let root_dir = absolute_post_root_dir();\n\n    println!(\"cargo:rerun-if-changed={}\", root_dir.display());\n\n    let blog_dir = &amp;blog_out_dir;\n    println!(\"Attempting to access: {}\", blog_dir.display());\n\n    generator::generate_posts(&amp;blog_src_dir, &amp;blog_out_dir)\n        .expect(\"Failed to generate posts during build\");\n\n    let about_dir = &amp;about_out_dir;\n    println!(\"Attempting to access: {}\", about_dir.display());\n\n    generator::generate_about(&amp;about_out_dir).expect(\"Failed to generate about during build\");\n}\n</code></pre></div>\n<div class=\"relative\"><pre><code class=\"language-js\">Here is a simple footnote[^1]. With some additional text after it.\n\n[^1]: My reference.\n</code></pre></div>"#;

        println!("Input: {input}");

        let result = add_gfm_code_title(input);

        println!("Result: {result}");

        let expected = r#"<h2>Syntax highlighting</h2>\n<p>Here's an example of how you can use syntax highlighting with <a href=\"https://help.github.com/articles/basic-writing-and-formatting-syntax/\">GitHub Flavored Markdown</a>:</p>\n<h2>Footnotes</h2>\n<div class="remark-code-title">test.rs</div><div class=\"relative\"><pre><code class=\"language-rust\">fn main() {\n    use domain::data::{\n        absolute_about_out_dir, absolute_blog_out_dir, absolute_blog_src_dir,\n        absolute_post_root_dir,\n    };\n\n    let about_out_dir = absolute_about_out_dir();\n    let blog_out_dir = absolute_blog_out_dir();\n    let blog_src_dir = absolute_blog_src_dir();\n    let root_dir = absolute_post_root_dir();\n\n    println!(\"cargo:rerun-if-changed={}\", root_dir.display());\n\n    let blog_dir = &amp;blog_out_dir;\n    println!(\"Attempting to access: {}\", blog_dir.display());\n\n    generator::generate_posts(&amp;blog_src_dir, &amp;blog_out_dir)\n        .expect(\"Failed to generate posts during build\");\n\n    let about_dir = &amp;about_out_dir;\n    println!(\"Attempting to access: {}\", about_dir.display());\n\n    generator::generate_about(&amp;about_out_dir).expect(\"Failed to generate about during build\");\n}\n</code></pre></div>\n<div class=\"relative\"><pre><code class=\"language-js\">Here is a simple footnote[^1]. With some additional text after it.\n\n[^1]: My reference.\n</code></pre></div>"#;

        println!("Expected: {expected}");

        assert_eq!(result, expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_no_filepath() {
        let input = r#"<div class="relative"><pre><code class="language-rust">fn main() {}</code></pre></div>"#;
        assert_eq!(add_gfm_code_title(input), input);
    }

    #[rustfmt::skip]
    #[test]
    fn test_with_special_characters() {
        let input = r#"<div class="relative"><pre><code class="language-tsx:pages/index.tsx">import React from 'react';</code></pre></div>"#;
        let expected = r#"<div class="remark-code-title">pages/index.tsx</div><div class="relative"><pre><code class="language-tsx">import React from 'react';</code></pre></div>"#;
        assert_eq!(add_gfm_code_title(input), expected);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        assert_eq!(add_gfm_code_title(input), "");
    }
}
