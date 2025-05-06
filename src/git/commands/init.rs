use std::io::{self, Write};
use std::{fs, path::Path};

pub fn init() -> io::Result<()> {
    let root = Path::new(".gut");

    if root.exists() {
        println!("Repository already initialized at {}", root.display());
        return Ok(());
    }

    fs::create_dir_all(root.join("objects"))?;
    fs::create_dir_all(root.join("refs"))?;

    let head_path = root.join("HEAD");
    fs::write(&head_path, "ref: refs/heads/main\n")?;

    println!("Initialized empty .gut repository in {}", root.display());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_gut_is_not_initialized() {
        let _ = fs::remove_dir_all(".gut");
        let path = Path::new(".gut");
        assert!(!path.exists())
    }

    #[test]
    fn test_initialize_gut() {
        let res = init();
        assert!(res.is_ok());
    }

    #[test]
    fn test_check_gut_folders_are_initialized() {
        assert!(Path::new(".gut/objects").exists());
        assert!(Path::new(".gut/refs").exists());
        assert!(Path::new(".gut/HEAD").exists());
    }
}
