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

```sh
Usage: upextract [OPTIONS] --bundle <BUNDLE>

Options:
  -b, --bundle <BUNDLE>  unitybundle
  -o, --out <OUT>        Output folder [default: out]
  -f, --flatten          Flatten folder structure
      --tmp <TMP>        Tmp folder to extract to. (defaults to use system tmp)
  -h, --help             Print help
```

## Example

```sh
upextract -b demoasset/test.unitypackage
# or
upextract -b demoasset/test.unitypackage -o output
```