/// Processes HTML content using Prism syntax highlighter via a Node.js script.
///
/// This function sends the provided HTML to a Node.js process that applies
/// Prism syntax highlighting and returns the processed HTML.
///
/// # Arguments
///
/// * `html` - The HTML content to process
///
/// # Returns
///
/// The processed HTML with syntax highlighting applied
///
/// # Errors
///
/// This function will return an error if:
/// - The Node.js process fails to start
/// - Writing to the Node.js process stdin fails
/// - The Node.js process exits with a non-zero status code
/// - The output from the Node.js process is not valid UTF-8
pub fn process_html_with_prism(html: &str) -> Result<String, std::io::Error> {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    use domain::data::absolute_process_rehype_prism_plus_filename;

    let mut child = Command::new("node")
        .arg(absolute_process_rehype_prism_plus_filename())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(html.as_bytes())?;
    } else {
        return Err(std::io::Error::other("Failed to open stdin"));
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(std::io::Error::other(error.to_string()));
    }

    let result = String::from_utf8(output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(result.trim().to_string())
}
