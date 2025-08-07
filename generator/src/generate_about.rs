use std::path::Path;

/// Generates about information.
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
pub fn generate_about<P>(out_path: P) -> Result<(), std::io::Error>
where
    P: AsRef<Path>,
{
    use std::{fs, io::Write};

    use domain::{
        data::{absolute_about_out_filename, absolute_about_src_filename},
        types::{About, AboutFrontMatter, AboutMetadata},
    };
    use gray_matter::{Matter, engine::YAML};
    use pulldown_cmark::{Options, Parser, html};

    let out_path = out_path.as_ref();

    fs::create_dir_all(out_path)?;

    let about_path = absolute_about_src_filename();
    let content = fs::read_to_string(&about_path)?;
    let mut options = Options::empty();

    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let matter = Matter::<YAML>::new();
    let about_data = matter
        .parse::<AboutFrontMatter>(&content)
        .expect("Failed to parse md frontmatter");
    let parser = Parser::new_ext(&about_data.content, options);
    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    let about = About::new(
        html_output,
        AboutMetadata::new(about_data.data.expect("Failed to parse frontmatter data")),
    );

    let json_content =
        serde_json::to_string_pretty(&about).expect("Failed to serialize about to JSON");
    let file_path = absolute_about_out_filename();
    let mut file = fs::File::create(&file_path)?;

    file.write_all(json_content.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_about() {
        use domain::data::absolute_about_out_dir;

        let out_dir = absolute_about_out_dir();
        let result = generate_about(&out_dir);

        assert!(
            result.is_ok(),
            "Failed to generate about: {:?}",
            result.err()
        );

        println!("About generated successfully in {}", out_dir.display());

        if let Err(e) = std::fs::remove_dir_all(&out_dir) {
            eprintln!("Failed to clean up {}: {}", out_dir.display(), e);
        }
    }
}
