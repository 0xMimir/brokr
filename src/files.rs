use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

pub(crate) fn recurse_files(
    path: impl AsRef<Path>,
    file_extensions: &[&str],
) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            buf.append(&mut recurse_files(entry.path(), file_extensions)?);
            continue;
        }


        if meta.is_file() {
            let entry = entry.path();
            if let Some(extension) = entry.extension() {
                if file_extensions.iter().any(|ext| extension == *ext) {
                    buf.push(entry);
                }
            }
        }
    }

    Ok(buf)
}

pub(crate) fn read_file(path: &PathBuf) -> std::io::Result<String> {
    read_to_string(path)
}
