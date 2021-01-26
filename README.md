<div align=center>

[![Cycle](https://raw.githubusercontent.com/hrkz/cycle/gh-pages/images/cycle_logo.png)](https://cycle-research.org)
# [Ω: interactive Cycle](https://hrkz.github.io/omega)

[![Build](https://img.shields.io/github/workflow/status/hrkz/cycle/CI?style=flat-square)](https://github.com/hrkz/cycle/actions)
[![Crate](https://img.shields.io/crates/v/cycle?style=flat-square)](https://crates.io/crates/cycle)
[![License](https://img.shields.io/github/license/hrkz/cycle.svg?color=informational&style=flat-square)](https://github.com/hrkz/omega/blob/master/LICENSE)

</div>
<hr>

### A WebAssembly front-end for Cycle

Omega is a simple interpreter for the cycle library and language. It supports a subset of expressions that will be documented as soon as possible.
It is mainly useful as a demonstration for both WebAssembly builds and cycle's features.

## Getting started

To use omega, you will need [Git](https://git-scm.com/) for cloning, a modern [Rust](https://www.rust-lang.org/) version with [Cargo](https://doc.rust-lang.org/stable/cargo/) for the compilation and [wasm-bindgen](https://crates.io/crates/wasm-bindgen) to interact with WebAssembly,
```bash
# Clone the repository
$ git clone https://github.com/hrkz/omega && cd omega

# Start the compilation, download dependencies and link with wasm-bindgen
$ cargo build --target wasm32-unknown-unknown
$ wasm-bindgen --target web target/wasm32-unknown-unknown/debug/omega.wasm --out-dir web/pkg/
```
For more details about WebAssembly with Rust, refer to the [book](https://rustwasm.github.io/docs/book/).

| Using Omega |       |
| :----------:| :----:|
| :bookmark_tabs: [reference](https://github.com/hrkz/omega/wiki/) | To get started with the expression syntax of the cycle language and the subset supported by omega |

