# Rustwell

Like *Fountain*, but written in *Rust* and it works *well*.

Rustwell is a tool for creating formatted screen- and stageplays, based on [Fountain](https://fountain.io/). Rustwell is both a parser and compiler crate for *Fountain*, but also contains a CLI tool for parsing and exporting formatted documents from *Fountain* script.

This repository contains both the Rustwell library, and the Rustwell-CLI tool as an interface for Rustwell.

## Usage

Please see the individual crates' README-files for information on how to use them.

## Features

Rustwell currently implements the entirely of the *Fountain* specification. It offers a blazing fast *Fountain* parser, and currently supports *PDF* and *HTML* exports natively.

## Acknowledgements

The original CSS code for the html output was created by [Jonathan Poritsky](https://www.candlerblog.com/), but has been expanded upon for this endeavour.

[Courier Prime](https://quoteunquoteapps.com/courierprime/), designed by Alan Dague-Greene, is distributed along with Rustwell for the `pdf`-exporter to have a baseline font. The original license can be found [here](./licenses/OFL-Courier-Prime.txt).
