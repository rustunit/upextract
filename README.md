# upextract

UnityPackage Asset extract tool

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
      --bundle <BUNDLE>  unitybundle
      --out <OUT>        Output folder [default: out]
      --tmp <TMP>        Tmp folder to extract to. (defaults to use system tmp)
  -h, --help             Print help
```

## Example

```sh
upextract --bundle demoasset/test.unitypackage
# or
upextract --bundle demoasset/test.unitypackage --out output
```