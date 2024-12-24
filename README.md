# upextract

UnityPackage Asset extract tool.

[![demo](https://asciinema.org/a/696012.svg)](https://asciinema.org/a/696012?autoplay=1)

## Requirements

* `rust` installed (cargo specfically)
* `tar` installed on `PATH`

## Installation

```sh
cargo install upextract
```

## Usage

```sh
UnityPackage Asset extract tool

Usage: upextract <COMMAND>

Commands:
  extract  Extracts contents of a unitypackage
  list     Lists unitypackages in the Unity Asset Store folder
  inspect  List contents of a unitypackage
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### upextract extract

```sh
Usage: upextract extract [OPTIONS] --bundle <BUNDLE>

Options:
  -b, --bundle <BUNDLE>    unitybundle
  -o, --out <OUT>          Output folder [default: out]
  -f, --flatten            Flatten folder structure
      --tmp <TMP>          Tmp folder to extract to. (defaults to use system tmp)
  -i, --include <INCLUDE>  What asset files (extensions) to extract. Defaults to all
  -h, --help               Print help
```

### upextract list

```sh
Usage: upextract list [OPTIONS]

Options:
      --assets-folder <ASSETS_FOLDER>  Unity Asset Store folder
  -h, --help                           Print help
```

### upextract inspect

```sh
List contents of a unitypackage

Usage: upextract inspect [OPTIONS] --bundle <BUNDLE>

Options:
  -b, --bundle <BUNDLE>  unitybundle
      --tmp <TMP>        Tmp folder to extract to. (defaults to use system tmp)
  -h, --help             Print help
```

## Example

```sh
upextract extract -b demoasset/test.unitypackage
# or
upextract extract -b demoasset/test.unitypackage -o output
```