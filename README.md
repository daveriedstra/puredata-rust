# Pure Data Externals in Pure Rust

Does not require any C, builds entirely with Cargo.

## Requirements

* [Rust and Cargo](https://www.rust-lang.org/tools/install), at least rust 1.34
* [Pure Data](https://puredata.info), to run, not to build.

## Crates/Subdirs

* [pd-sys](pd-sys) The basic generated bindings to `m_pd.h`, all you need to roll your own interface to puredata.
* [external](external) The Rust specific wrappers and ease of use methods for creating externals.
* [macros](macros) The macros that generate all the guts for externals you don't want to have to deal with yourself, used the examples.
* [macros/examples](macros/examples) See below.
* [utils](utils/README.md) Utilities for building and packaging externals.

## Examples

The examples can all be built with `cargo build` but you can also use `cargo-make` for more helpful features.
See [utils/README.md](utils/README.md) for more details on the `cargo-make` setup.

* [helloworld](macros/examples/helloworld/src/lib.rs) based on the HOWTO [my first external: helloworld](https://github.com/pure-data/externals-howto#my-first-external-helloworld) example
* [counter](macros/examples/counter/src/lib.rs) based on the HOWTO [a simple external: counter](https://github.com/pure-data/externals-howto#a-simple-external-counter) example
* [complex_counter](macros/examples/complex_counter/src/lib.rs) based on the HOWTO [a complex external: counter](https://github.com/pure-data/externals-howto#a-complex-external-counter) example
* [xfade](macros/examples/xfade/src/lib.rs) based on the, misnamed, HOWTO [a signal-external pan~](https://github.com/pure-data/externals-howto#a-signal-external-pan) example

```
cd macros/examples/helloworld && cargo build
```

or

```
cd macros/examples/helloworld && cargo make run
```

If you want to see what code the macro actually produces, use `cargo-expand`.

## TODO

* Documentation
* Expose pointer methods
* Support more creation argument configurations
* Clean up macros
* [crates.io](https://crates.io/) release

## References

* [Pure Data](https://puredata.info)
* [HOWTO write an External for Pure Data](https://github.com/pure-data/externals-howto)

## Links

* [kmtr pd_ext_rust](https://github.com/kmtr/pd_ext_rust): another Pure Data External solution for rust, requires C.
* [RustAudio](https://github.com/RustAudio)
