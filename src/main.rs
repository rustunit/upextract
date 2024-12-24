mod extract;
mod inspect;
mod list;

use clap::{Parser, Subcommand};

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
        Commands::Extract(args) => extract::command(args),
        Commands::List { assets_folder } => list::command(assets_folder),
        Commands::Inspect(args) => inspect::command(args),
    }
}
