mod component;
mod html_file;

#[cfg(test)]
mod test;

use component::Component;
use html_file::HtmlFile;

use std::{env, fs, process};
use std::path::Path;
use anyhow::Result;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please specify the project directory (use a dot for the current directory)");
        process::exit(1);
    }

    if args[1].as_str() == "." {
        println!("Generating a site from the current directory...");
    } else {
        println!("Generating a site from {}...", &args[1].as_str());
    }
    
    let path = Path::new(&args[1]);
    if !path.is_dir() {
        eprintln!("The specified directory does not exist");
        process::exit(1);
    }

    if let Err(e) = generate_site(path) {
        println!("Error generating the site: {:?}", e);
    } else {
        println!("Done!");
    }
}

/// Generate site from a given directory
fn generate_site(path: &Path) -> Result<()> {
    let components = Component::get_components(&path.join("components"));
    fs::create_dir(path.join("dist")).ok();

    for entry in path.read_dir()? {
        let entry = entry?;
        let filename = entry.file_name().to_string_lossy().to_string();
        if !&filename.ends_with(".html") {
            continue;
        }

        let content = fs::read_to_string(entry.path())?;

        let file = HtmlFile::new(filename, content)
            .insert_components(&components);
        let mut output_path = path
            .join("dist")
            .join(file.name);
        output_path.set_extension("html");
        
        fs::write(&output_path, file.content)?
    }
    
    Ok(())
}
