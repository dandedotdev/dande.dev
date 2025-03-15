use std::sync::OnceLock;

use regex::Regex;

static CODE_BLOCK_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_code_block_regex() -> &'static Regex {
    CODE_BLOCK_REGEX
        .get_or_init(|| Regex::new(r"(?s)(<pre><code(?:\s+[^>]*)?>.*?</code></pre>)").unwrap())
}

#[must_use]
pub fn add_relative_div(html: &str) -> String {
    get_code_block_regex()
        .replace_all(html, r#"<div class="relative">$1</div>"#)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_wrapping() {
        let input = "<pre><code>let x = 1;</code></pre>";
        let expected = "<div class=\"relative\"><pre><code>let x = 1;</code></pre></div>";
        assert_eq!(add_relative_div(input), expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_wrapping_with_language_class() {
        let input = "<pre><code class=\"language-rust\">fn main() {}</code></pre>";
        let expected = "<div class=\"relative\"><pre><code class=\"language-rust\">fn main() {}</code></pre></div>";
        assert_eq!(add_relative_div(input), expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_wrapping_multiple_blocks() {
        let input = "<pre><code>fn main()</code></pre><p>Some text</p><pre><code>println!()</code></pre>";
        let expected = "<div class=\"relative\"><pre><code>fn main()</code></pre></div><p>Some text</p><div class=\"relative\"><pre><code>println!()</code></pre></div>";
        assert_eq!(add_relative_div(input), expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_wrapping_with_multiline_content() {
        let input = "<pre><code>fn main() {\n    println!(\"Hello, world!\");\n}</code></pre>";
        let expected = "<div class=\"relative\"><pre><code>fn main() {\n    println!(\"Hello, world!\");\n}</code></pre></div>";
        assert_eq!(add_relative_div(input), expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_wrapping_with_html_entities() {
        let input = "<pre><code>&lt;div&gt;HTML content&lt;/div&gt;</code></pre>";
        let expected = "<div class=\"relative\"><pre><code>&lt;div&gt;HTML content&lt;/div&gt;</code></pre></div>";
        assert_eq!(add_relative_div(input), expected);
    }

    #[test]
    fn test_no_code_blocks() {
        let input = "<p>Just some text without code blocks</p>";
        let expected = "<p>Just some text without code blocks</p>";
        assert_eq!(add_relative_div(input), expected);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(add_relative_div(input), expected);
    }
}
