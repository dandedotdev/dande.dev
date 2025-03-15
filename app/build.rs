fn main() {
    use domain::data::{absolute_about_out_dir, absolute_blog_out_dir, absolute_blog_src_dir};

    let about_out_dir = absolute_about_out_dir();
    let blog_out_dir = absolute_blog_out_dir();
    let blog_src_dir = absolute_blog_src_dir();
    let blog_dir = &blog_out_dir;

    println!("Attempting to access: {}", blog_dir.display());

    generator::generate_posts(&blog_src_dir, &blog_out_dir)
        .expect("Failed to generate posts during build");

    let about_dir = &about_out_dir;

    println!("Attempting to access: {}", about_dir.display());

    generator::generate_about(&about_out_dir).expect(
        "Failed to generate
    about during build",
    );
}
