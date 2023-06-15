use std::fs;
use std::path::Path;

pub fn copy_directory(src: &Path, dest: &Path) -> std::io::Result<()> {
    // Create the destination directory if it doesn't exist
    fs::create_dir_all(dest)?;

    // Iterate over the entries in the source directory
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry.file_name();

        // Determine the new path in the destination directory
        let dest_path = fs::canonicalize(dest)?.join(&file_name);

        if entry_path.is_dir() {
            // Recursively copy subdirectories
            copy_directory(&entry_path, &dest_path)?;
        } else {
            // Copy the file
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}
