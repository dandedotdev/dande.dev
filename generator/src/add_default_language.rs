#[must_use]
pub fn add_default_language(html: &str) -> String {
    html.replace("<pre><code>", "<pre><code class=\"language-js\">")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_replacement() {
        let input = "<pre><code>let x = 1;</code></pre>";
        let expected = "<pre><code class=\"language-js\">let x = 1;</code></pre>";
        assert_eq!(add_default_language(input), expected);
    }

    #[rustfmt::skip]
    #[test]
    fn test_multiple_code_blocks() {
        let input = "<pre><code>fn main()</code></pre><pre><code>println!()</code></pre>";
        let expected = "<pre><code class=\"language-js\">fn main()</code></pre><pre><code class=\"language-js\">println!()</code></pre>";
        assert_eq!(add_default_language(input), expected);
    }

    #[test]
    fn test_existing_language_untouched() {
        let input = "<pre><code class=\"language-rust\">fn main()</code></pre>";
        assert_eq!(add_default_language(input), input);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        assert_eq!(add_default_language(input), "");
    }
}
