use std::{
    collections::HashSet,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use tempfile::tempdir;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Extract(Extract),
    List {
        #[arg(long, help = "Unity Asset Store folder")]
        assets_folder: Option<String>,
    },
}

#[derive(Parser)]
struct Extract {
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

    #[arg(
        long,
        short,
        help = "What asset files (extensions) to extract. Defaults to all"
    )]
    include: Option<Vec<String>>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Extract(args) => extract(args),
        Commands::List { assets_folder } => list(assets_folder),
    }
}

fn list(assets_folder: Option<String>) {
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
    for entry in fs::read_dir(path)? {
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

fn extract(args: Extract) {
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
        args.include.map(|v| v.into_iter().collect()),
    ) {
        println!("Error: {:?}", e);
    }

    if tmp.is_some() {
        println!("Cleaning up tmp folder");
    }
}

fn unpack(
    file: String,
    tmp: &Path,
    output: &Path,
    flatten: bool,
    include: Option<HashSet<String>>,
) -> anyhow::Result<()> {
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

fn handle_asset(
    path: &Path,
    output: &Path,
    flatten: bool,
    include: Option<&HashSet<String>>,
) -> anyhow::Result<bool> {
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

    let Some(extension) = out_path.extension() else {
        return Ok(false);
    };

    if include.map_or(false, |include| {
        !include.contains(&extension.to_string_lossy().to_string())
    }) {
        println!(
            "Skipping {:?} - {:?}",
            asset
                .parent()
                .unwrap()
                .components()
                .last()
                .unwrap()
                .as_os_str(),
            out_path.file_name().unwrap()
        );
        return Ok(false);
    }

    fs::create_dir_all(out_path.parent().unwrap())?;

    fs::copy(asset, out_path).context("copy asset failed")?;

    Ok(true)
}
