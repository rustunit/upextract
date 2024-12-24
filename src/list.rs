use std::path::{Path, PathBuf};

pub fn command(assets_folder: Option<String>) {
    let assets_folder = assets_folder.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap()
            .join("Library/Unity/Asset Store-5.x")
            .to_string_lossy()
            .to_string()
    });

    println!("Listing unitypackage in {:?}", assets_folder);

    list_unitypackages(PathBuf::from(assets_folder).as_path()).unwrap();
}

fn list_unitypackages(path: &Path) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            list_unitypackages(&path)?;
        } else if is_package(path.as_path()) {
            println!("{:?}", path);
        }
    }

    Ok(())
}

fn is_package(as_path: &Path) -> bool {
    as_path
        .extension()
        .map_or(false, |ext| ext == "unitypackage")
}
