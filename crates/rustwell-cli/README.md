# Rustwell-CLI

Rustwell-CLI is a CLI (command line interface) tool for parsing and formatting screenplays written as [Fountain](https://fountain.io/) files. This means this tool can, provided a *Fountain* file, generate a formatted screenplay as, for example, a `pdf` or a `html` file.

## Usage

```sh
cargo run -- script.fountain -o script.pdf
```

More details on the CLI tool can be found by running:

```sh
cargo run -- -h
```

which will show all current possible output formats.
