mod component;
mod html_file;
mod helpers;

#[cfg(test)]
mod test;

use component::Component;
use html_file::HtmlFile;
use helpers::copy_directory;

use std::{fs, process};
use std::env::current_dir;

use std::path::{Path, PathBuf};
use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the project to compile
    path: PathBuf,

    #[clap(flatten)]
    verbose: Verbosity<WarnLevel>,
}

fn main() {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .format_timestamp(None)
        .format_target(false)
        .init();

    if !&args.path.is_dir() {
        eprintln!("The specified directory does not exist");
        process::exit(1);
    }

    if args.path == current_dir().unwrap() {
        println!("Generating a site from the current directory...");
    } else {
        println!("Generating a site from {}...", &args.path.display());
    }

    if let Err(e) = generate_site(&args.path) {
        println!("Error generating the site: {:?}", e);
    } else {
        println!("Done!");
    }
}

/// Generate site from a given directory
fn generate_site(path: &Path) -> Result<()> {
    let components = Component::get_components(&path.join("components"));
    fs::create_dir(path.join("dist")).ok();

    if path.join("public").exists() {
        log::info!("Copying the public directory");
        copy_directory(&path.join("public"), &path.join("dist/public"))?;
    }

    for entry in path.read_dir()? {
        let entry = entry?;
        let filename = entry.file_name().to_string_lossy().to_string();
        if !&filename.ends_with(".html") {
            continue;
        }
        log::info!("Found an HTML file {}", &filename);

        let content = fs::read_to_string(entry.path())?;

        let file = HtmlFile::new(filename.clone(), content)
            .insert_components(&components);
        let mut output_path = path
            .join("dist")
            .join(file.name);
        output_path.set_extension("html");

        log::info!("Writing dist/{}", &filename);
        fs::write(&output_path, file.content)?
    }
    
    Ok(())
}
