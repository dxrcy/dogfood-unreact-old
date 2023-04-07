use unreact::prelude::*;

// Where the site is hosted
const URL: &str = "https://darccyy.github.io/dogfood";

fn main() -> Result<(), Error> {
    let config = Config {
        strict: true,
        minify: !is_dev(),
        ..Config::default()
    };

    let mut app = Unreact::new(config, is_dev(), URL)?;

    let entries = dogfood::get_entries();

    app
        // Index page
        .index("homepage", object! { entries })?
        // 404 page
        .not_found("404", object! {})?
        // Complete app
        .run()?;

    // Only prints if NOT in dev mode
    println!("Compiled for production.");
    Ok(())
}
