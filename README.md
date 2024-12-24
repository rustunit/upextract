# upextract

UnityPackage Asset extract tool.

## Requirements

* `rust` installed (cargo specfically)
* `tar` installed on `PATH`

## Installation

```sh
cargo install upextract
```

## Usage

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

## Example

```sh
upextract extract -b demoasset/test.unitypackage
# or
upextract extract -b demoasset/test.unitypackage -o output
```