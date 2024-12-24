use std::{
    collections::{HashMap, HashSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use tempfile::tempdir;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Extract(Extract),
    #[command(about = "Lists unitypackages in the Unity Asset Store folder")]
    List {
        #[arg(long, help = "Unity Asset Store folder")]
        assets_folder: Option<String>,
    },
    #[command(about = "List contents of a unitypackage")]
    Inspect(Inspect),
}

#[derive(Parser)]
#[command(about = "Extracts contents of a unitypackage")]
struct Inspect {
    #[arg(long, short, help = "unitybundle")]
    bundle: String,

    #[arg(long, help = "Tmp folder to extract to. (defaults to use system tmp)")]
    tmp: Option<String>,
}

#[derive(Parser)]
#[command(about = "Extracts contents of a unitypackage")]
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
        Commands::Extract(args) => extract_cmd(args),
        Commands::List { assets_folder } => list(assets_folder),
        Commands::Inspect(args) => inspect(args),
    }
}

fn inspect(args: Inspect) {
    let mut types: HashMap<String, usize> = HashMap::new();

    let (tmp, folder) = tmp_folder(args.tmp.as_ref());

    unpack(args.bundle.clone(), folder.as_path()).unwrap();

    for entry in fs::read_dir(folder).unwrap() {
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

fn extract_cmd(args: Extract) {
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

fn tmp_folder(tmp: Option<&String>) -> (Option<tempfile::TempDir>, PathBuf) {
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

fn unpack(file: String, tmp: &Path) -> Result<(), anyhow::Error> {
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

fn asset_folder_pathname(path: &Path) -> Result<Option<PathBuf>, anyhow::Error> {
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
