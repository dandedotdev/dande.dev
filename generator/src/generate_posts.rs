use std::path::Path;

use crate::{add_default_language, add_gfm_code_title, add_relative_div, process_html_with_prism};

/// Generates posts by processing markdown files from source path and outputs
/// them to the destination path.
///
/// # Panics
///
/// Panics if:
/// - Failed to parse markdown frontmatter
///
/// # Errors
///
/// Returns an error if:
/// - Failed to read from the source directory
/// - Failed to create the output directory
/// - Failed to read or write files during the process
pub fn generate_posts<P, Q>(
    src_path: P,
    out_path: Q,
) -> Result<(), std::io::Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    use std::{fs, io::Write};

    use domain::types::{Post, PostFrontMatter, PostMetadata};
    use gray_matter::{Matter, engine::YAML};
    use pulldown_cmark::{Options, Parser, html};
    use rayon::prelude::*;

    let src_path = src_path.as_ref();
    let out_path = out_path.as_ref();

    fs::create_dir_all(out_path)?;

    let posts: Vec<Post> = fs::read_dir(src_path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            if !path.is_file() {
                return None;
            }

            let extension = path.extension()?;

            if extension != "md" {
                return None;
            }

            let slug = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .replace(".md", "");
            let content = fs::read_to_string(&path).ok()?;
            let mut options = Options::empty();

            // https://docs.rs/pulldown-cmark/latest/pulldown_cmark/struct.Options.html
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_GFM);
            options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
            options.insert(Options::ENABLE_SMART_PUNCTUATION);
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_TASKLISTS);

            let matter = Matter::<YAML>::new();
            let post_data = matter
                .parse::<PostFrontMatter>(&content)
                .expect("Failed to parse md frontmatter");

            let maybe_post_data = post_data.data.expect("Failed to parse frontmatter data");

            if maybe_post_data.draft {
                println!("Skipping draft post: {slug}");
                return None;
            }

            let parser = Parser::new_ext(&post_data.content, options);
            let mut html_output = String::new();

            html::push_html(&mut html_output, parser);

            let html_output_with_default_language = add_default_language(&html_output);
            let html_output_with_relative_div =
                add_relative_div(&html_output_with_default_language);
            let html_output_with_gfm_code_title =
                add_gfm_code_title(&html_output_with_relative_div);
            let highlighted_html = match process_html_with_prism(&html_output_with_gfm_code_title) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Warning: Failed to process HTML with Prism for {slug}: {e}");
                    html_output_with_gfm_code_title
                },
            };

            Some(Post::new(
                highlighted_html,
                PostMetadata::new(maybe_post_data, slug),
            ))
        })
        .collect();

    posts.par_iter().try_for_each(|post| {
        let json_content =
            serde_json::to_string_pretty(post).expect("Failed to serialize post to JSON");
        let json_file_path = out_path.join(format!("{}.json", post.metadata.slug));
        let mut file = fs::File::create(&json_file_path)?;

        file.write_all(json_content.as_bytes())?;

        Ok::<(), std::io::Error>(())
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_posts() {
        use domain::data::{absolute_blog_out_dir, absolute_blog_src_dir};

        let src_dir = absolute_blog_src_dir();
        let out_dir = absolute_blog_out_dir();
        let result = generate_posts(&src_dir, &out_dir);

        assert!(
            result.is_ok(),
            "Failed to generate posts: {:?}",
            result.err()
        );

        println!("Posts generated successfully in {}", out_dir.display());

        if let Err(e) = std::fs::remove_dir_all(&out_dir) {
            eprintln!("Failed to clean up {}: {}", out_dir.display(), e);
        }
    }
}
