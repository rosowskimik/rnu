use anyhow::bail;
use anyhow::{Context, Result};
use clap::{crate_authors, crate_description, crate_version};
use clap::{App, Arg};
use std::fs;
use std::path::Path;
use uuid::Uuid;

fn main() -> Result<()> {
    let matches = App::new("rnu")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("path").required(true).value_name("FILE"))
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .short("q")
                .help("Don't print out anything")
                .overrides_with("verbose"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("Print out the full path to renamed file")
                .overrides_with("quiet"),
        )
        .get_matches();

    let old_file = matches.value_of("path").unwrap();
    let old_file = Path::new(old_file);

    if !old_file.is_file() {
        bail!("File doesn't exist!");
    }

    let parent_dir = old_file.parent().unwrap();

    let mut new_file = Uuid::new_v4().to_string();
    if let Some(val) = old_file.extension() {
        new_file.push('.');
        new_file.push_str(val.to_str().unwrap());
    }

    let new_file = parent_dir.join(new_file);

    fs::rename(old_file, &new_file).context("Failed to rename file")?;

    match (matches.is_present("quiet"), matches.is_present("verbose")) {
        (false, true) => println!("{}", new_file.to_str().unwrap()),
        (false, false) => println!("{}", new_file.file_name().unwrap().to_str().unwrap()),
        _ => {}
    };

    Ok(())
}
