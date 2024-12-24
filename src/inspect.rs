use std::{collections::HashMap, path::Path};

use crate::{
    extract::{asset_folder_pathname, tmp_folder, unpack},
    Inspect,
};

pub fn command(args: Inspect) {
    let mut types: HashMap<String, usize> = HashMap::new();

    let (tmp, folder) = tmp_folder(args.tmp.as_ref());

    unpack(args.bundle.clone(), folder.as_path()).unwrap();

    for entry in std::fs::read_dir(folder).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            inspect_asset(&path, &mut types).unwrap();
        }
    }

    if let Some(tmp) = tmp {
        println!("Cleaning up tmp folder");
        tmp.close().unwrap();
    }

    println!("\nContents of unitypackage: {}", args.bundle.clone());

    for (k, v) in types.iter() {
        println!("{}: {}", k, v);
    }
}

fn inspect_asset(path: &Path, types: &mut HashMap<String, usize>) -> anyhow::Result<()> {
    let pathname = asset_folder_pathname(path)?;

    let Some(pathname) = pathname else {
        return Ok(());
    };

    if let Some(extension) = pathname.extension() {
        let count = types
            .entry(extension.to_string_lossy().to_string())
            .or_insert(0);
        *count += 1;
    }

    Ok(())
}
