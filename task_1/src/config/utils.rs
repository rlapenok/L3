use std::{error::Error, fs::File, io::Read, path::PathBuf};

pub fn get_migrations(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let migrations = path
        .read_dir()?
        .into_iter()
        .filter_map(|read_dir| {
            if let Ok(dir_entry) = read_dir {
                if let Ok(mut file) = File::open(dir_entry.path()) {
                    let mut buff = String::new();
                    if let Ok(_) = file.read_to_string(&mut buff) {
                        let migration = buff.trim().trim_end_matches(";");
                        if !migration.is_empty() {
                            return Some(migration.to_owned());
                        }
                        return None;
                    }
                    return None;
                }
                return None;
            }
            None
        })
        .collect::<Vec<String>>()
        .join(";\n")
        + ";";
    Ok(migrations)
}
