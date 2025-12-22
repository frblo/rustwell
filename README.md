# Rustwell

Like *Fountain*, but written in *Rust* and it works *well*.

Rustwell is a tool for creating formatted screen- and stageplays, based on [Fountain](https://fountain.io/). Rustwell is both a parser and compiler crate for *Fountain*, but also contains a CLI tool for parsing and exporting formatted documents from *Fountain* script.

This repository contains both the Rustwell library, and the Rustwell-CLI tool as an interface for Rustwell.

## Usage

Please see the individual crates' README-files for information on how to use them.

## Acknowledgements

The original CSS code for the html output was created by [Jonathan Poritsky](https://www.candlerblog.com/), but has been expanded upon for this endeavour.

The typst template is based on the [celluloid](https://typst.app/universe/package/celluloid) template for typst by [Casey Dahlin](mailto:sadmac@google.com), licensed under [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0) license. The template used for Rustwell is a heavily modified version of celluloid, and is not considered fit for use outside of the scope of Rustwell. The original license can be found [here](./licenses/OFL-Courier-Prime.txt).

[Courier Prime](https://quoteunquoteapps.com/courierprime/), designed by Alan Dague-Greene, is distributed along with Rustwell for the `pdf`-exporter to have a baseline font. The original license can be found [here](./licenses/OFL-Courier-Prime.txt).
