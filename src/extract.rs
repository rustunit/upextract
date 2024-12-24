use crate::Extract;
use anyhow::{bail, Context};
use std::{
    collections::HashSet,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::tempdir;

pub fn command(args: Extract) {
    let (tmp, folder) = tmp_folder(args.tmp.as_ref());

    if let Err(e) = extract(
        args.bundle.clone(),
        folder.as_path(),
        PathBuf::from(args.out).as_path(),
        args.flatten,
        args.include.map(|v| v.into_iter().collect()),
    ) {
        println!("Error: {:?}", e);
    }

    if tmp.is_some() {
        println!("Cleaning up tmp folder");
    }
}

pub fn tmp_folder(tmp: Option<&String>) -> (Option<tempfile::TempDir>, PathBuf) {
    let (tmp, folder) = if let Some(tmp) = tmp.as_ref() {
        (None, PathBuf::from(tmp))
    } else {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().to_path_buf();
        (Some(tmp), folder)
    };
    (tmp, folder)
}

fn extract(
    file: String,
    tmp: &Path,
    output: &Path,
    flatten: bool,
    include: Option<HashSet<String>>,
) -> anyhow::Result<()> {
    unpack(file, tmp)?;

    println!("Extracting assets");

    if let Some(include) = include.as_ref() {
        println!("Only extracting: [{:?}]", include);
    }

    let mut count = 0;

    for entry in fs::read_dir(tmp)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
            && handle_asset(&path, output, flatten, include.as_ref())
                .context(format!("handling '{:?}'", path))?
        {
            count += 1;
        }
    }

    println!("Extracted {} assets", count);

    Ok(())
}

pub fn unpack(file: String, tmp: &Path) -> Result<(), anyhow::Error> {
    println!("Unpacking '{}' to '{}'", file, tmp.display());

    let out = Command::new("tar")
        .arg("zxvf")
        .arg(file)
        .arg("-C")
        .arg(tmp)
        .output()
        .context("untar")?;

    if !out.status.success() {
        bail!("Error unpacking: {}", String::from_utf8_lossy(&out.stderr));
    }

    Ok(())
}

fn handle_asset(
    path: &Path,
    output: &Path,
    flatten: bool,
    include: Option<&HashSet<String>>,
) -> anyhow::Result<bool> {
    let pathname = asset_folder_pathname(path)?;

    let Some(pathname) = pathname else {
        return Ok(false);
    };

    let out_path = if !flatten {
        output.join(pathname)
    } else {
        output.join(pathname.to_str().unwrap().replace("/", "_"))
    };

    let Some(extension) = out_path.extension() else {
        return Ok(false);
    };

    if include.map_or(false, |include| {
        !include.contains(&extension.to_string_lossy().to_string())
    }) {
        println!(
            "Skipping {:?} - {:?}",
            path.components().last().unwrap().as_os_str(),
            out_path.file_name().unwrap()
        );
        return Ok(false);
    }

    fs::create_dir_all(out_path.parent().unwrap())?;

    fs::copy(path.join("asset"), out_path).context("copy asset failed")?;

    Ok(true)
}

pub fn asset_folder_pathname(path: &Path) -> Result<Option<PathBuf>, anyhow::Error> {
    let asset = path.join("asset");

    if !asset.exists() {
        return Ok(None);
    }

    let pathname = {
        let mut buf = String::new();
        fs::File::open(path.join("pathname"))?.read_to_string(&mut buf)?;
        PathBuf::from(buf.lines().next().unwrap())
    };

    Ok(Some(pathname))
}
