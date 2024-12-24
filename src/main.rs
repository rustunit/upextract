use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context};
use clap::Parser;
use tempfile::tempdir;

#[derive(Parser, Debug)]
struct Arguments {
    #[arg(long, short, help = "unitybundle")]
    bundle: String,

    #[arg(
        long,
        short,
        default_value_t = String::from("out"),
        help = "Output folder",
    )]
    out: String,

    #[arg(
        long,
        short,
        default_value_t = false,
        help = "Flatten folder structure"
    )]
    flatten: bool,

    #[arg(long, help = "Tmp folder to extract to. (defaults to use system tmp)")]
    tmp: Option<String>,
}

fn main() {
    let args = Arguments::parse();

    let (tmp, folder) = if let Some(tmp) = args.tmp.as_ref() {
        (None, PathBuf::from(tmp))
    } else {
        let tmp = tempdir().unwrap();
        let folder = tmp.path().to_path_buf();
        (Some(tmp), folder)
    };

    if let Err(e) = unpack(
        args.bundle.clone(),
        folder.as_path(),
        PathBuf::from(args.out).as_path(),
        args.flatten,
    ) {
        println!("Error: {:?}", e);
    }

    if tmp.is_some() {
        println!("Cleaning up tmp folder");
    }
}

fn unpack(file: String, tmp: &Path, output: &Path, flatten: bool) -> anyhow::Result<()> {
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

    println!("Extracting assets");

    let mut count = 0;

    for entry in fs::read_dir(tmp)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if handle_asset(&path, output, flatten).context(format!("handling '{:?}'", path))? {
                count += 1;
            }
        }
    }

    println!("Extracted {} assets", count);

    Ok(())
}

fn handle_asset(path: &Path, output: &Path, flatten: bool) -> anyhow::Result<bool> {
    let asset = path.join("asset");

    if !asset.exists() {
        return Ok(false);
    }

    let pathname = {
        let mut buf = String::new();
        fs::File::open(path.join("pathname"))?.read_to_string(&mut buf)?;
        buf.lines().next().unwrap().to_string()
    };

    let out_path = if !flatten {
        output.join(pathname)
    } else {
        output.join(pathname.replace("/", "_"))
    };

    fs::create_dir_all(out_path.parent().unwrap())?;

    fs::copy(asset, out_path).context("copy asset failed")?;

    Ok(true)
}
