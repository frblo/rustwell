# Rustwell-CLI

Rustwell-CLI is a CLI (command line interface) tool for parsing and formatting screenplays written as [Fountain](https://fountain.io/) files. This means this tool can, provided a *Fountain* file, generate a formatted screenplay as, for example, a `pdf` or a `html` file.

## Install

Running the following command will globally install the `rustwell` command.

```sh
cargo install rustwell-cli
```

## Usage

```sh
rustwell script.fountain
```

Will produce `script.pdf`. You can also specify the output name with the `-o` flag.

```sh
rustwell script.fountain -o big_play.html
```

More details on usage can be found by running:

```sh
rustwell -h
```
