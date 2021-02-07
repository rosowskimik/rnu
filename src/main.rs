use anyhow::bail;
use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version};
use clap::{App, Arg};
use std::{ffi, fs, path::Path};
use uuid::Uuid;

fn main() -> Result<()> {
    let matches = App::new("rnu")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("path")
                .required(true)
                .value_name("path")
                .multiple(true),
        )
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .short("q")
                .help("Don't print out anything"),
        )
        .arg(
            Arg::with_name("copy")
                .long("copy")
                .short("c")
                .help("Copy <path> instead of renaming"),
        )
        .arg(
            Arg::with_name("append")
                .long("append")
                .short("a")
                .help("Append to original name instead of replacing it"),
        )
        .get_matches();

    let input_files = matches.values_of("path").unwrap();
    let quiet = matches.is_present("quiet");
    let copy = matches.is_present("copy");
    let append = matches.is_present("append");
    let sign = if !quiet {
        Some(if !copy { "->" } else { "+" })
    } else {
        None
    };

    for infile in input_files {
        let infile_path = Path::new(infile);
        let infile_ext = infile_path.extension();

        let mut new_filename = if append {
            let old_filename = infile_path
                .file_stem()
                .unwrap_or_else(|| ffi::OsStr::new(""))
                .to_string_lossy();
            format!("{}-{}", old_filename, Uuid::new_v4().to_string())
        } else {
            Uuid::new_v4().to_string()
        };
        if let Some(ext) = infile_ext {
            new_filename.push('.');
            new_filename.push_str(&ext.to_string_lossy());
        }

        let outfile_path = infile_path.with_file_name(new_filename);
        let file_result = if !copy {
            fs::rename(infile_path, &outfile_path)
        } else {
            fs::copy(infile_path, &outfile_path).map(|_x| ())
        };

        match (file_result, quiet) {
            (Ok(_), false) => println!(
                "{} {} {}",
                infile_path.to_str().unwrap(),
                sign.unwrap(),
                outfile_path.to_str().unwrap()
            ),
            //
            (Err(e), _) => bail!(e),
            _ => {}
        }
    }
    Ok(())
}
