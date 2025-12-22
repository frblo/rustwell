# rustwell

Like *Fountain*, but written in *Rust* and it works *well*.

Rustwell is a tool for creating formatted screen- and stageplays, based on [Fountain](https://fountain.io/). Rustwell is both a parser and compiler crate for *Fountain*, but also contains a CLI tool for parsing and exporting formatted documents from *Fountain* script.

## Usage

### Crate

Add this dependency to your `Cargo.toml`:

```toml
[dependencies]
rustwell = "0.1"
```

and then start using the library. Please read the documentation for more details on how to use the library.

### CLI

The CLI is most easily used via cargo, and can for example be used to compile a formatted pdf version of a script by running:

```sh
cargo run -- script.fountain -o script.pdf
```

More details on the CLI tool can be found by running:

```sh
cargo run -- -h
```


## Acknowledgements

The original CSS code for the html output was created by [Jonathan Poritsky](https://www.candlerblog.com/), but has been expanded upon for this endeavour.

The typst template is based on the [celluloid](https://typst.app/universe/package/celluloid) template for typst by [Casey Dahlin](mailto:sadmac@google.com), licensed under [Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0) license. The template used for Rustwell is a heavily modified version of celluloid, and is not considered fit for use outside of the scope of Rustwell. The original license can be found [here](./licenses/OFL-Courier-Prime.txt).

[Courier Prime](https://quoteunquoteapps.com/courierprime/), designed by Alan Dague-Greene, is distributed along with Rustwell for the `pdf`-exporter to have a baseline font. The original license can be found [here](./licenses/OFL-Courier-Prime.txt).
