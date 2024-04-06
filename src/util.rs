use anyhow::{anyhow, Result};
use std::path::Path;
///If backup directory provided, ensure it exists, throw an error if it exists and is not a
///directory
pub fn confirm_backup_directory_if_provided(maybe_backup_path: &Option<String>) -> Result<()> {
    if let Some(ref backup_path_str) = maybe_backup_path {
        let backup_path = Path::new(backup_path_str);
        //create a directory if it does not exists
        std::fs::create_dir_all(backup_path_str)?;
        if !backup_path.is_dir() {
            return Err(anyhow!(
                "Backup path should be a directory, check {} exists and is directory",
                backup_path_str
            ));
        }
    }
    Ok(())
}
