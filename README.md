<div align=center>

# Ω

**Symbolic maths in your browser**

A WebAssembly front-end for Cycle

[![Build](https://img.shields.io/github/workflow/status/hrkz/cycle/CI?style=flat-square)](https://github.com/hrkz/cycle/actions)
[![Crate](https://img.shields.io/crates/v/cycle?style=flat-square)](https://crates.io/crates/cycle)
[![License](https://img.shields.io/github/license/hrkz/cycle.svg?color=informational&style=flat-square)](https://github.com/hrkz/omega/blob/master/LICENSE)

[About](#about) •
[Getting started](#getting-started) •
[Reference](#reference) •
[Planned](#planned) •
[Citation](#citation)

[![Screen](https://raw.githubusercontent.com/hrkz/omega/gh-pages/img/screen_v1.png)](https://hrkz.github.io/omega)

</div>
<hr>

# About

Omega is a simple Rust crate that uses the Cycle interpreter and expose its functionalities into a web page.
Its main objective is to demonstrate both WebAssembly builds and Cycle's features.

## Getting started

To use Omega, you will need [Git](https://git-scm.com/) for cloning, a modern [Rust](https://www.rust-lang.org/) version with either [Cargo](https://doc.rust-lang.org/stable/cargo/) for the compilation and [wasm-bindgen](https://crates.io/crates/wasm-bindgen) or [wasm-pack](https://rustwasm.github.io/wasm-pack/) to interact with WebAssembly,
```bash
# Clone the repository
$ git clone https://github.com/hrkz/omega && cd omega

# Start the compilation, download dependencies and link with WebAssembly
$ wasm-pack build
# or with Cargo and wasm-bindgen
```
For more details about WebAssembly with Rust, refer to the [book](https://rustwasm.github.io/docs/book/).

## Planned

The front-end mostly depends on the features developed in [Cycle](https://github.com/hrkz/cycle), but UI-related, we plan to

* Add a LaTeX output formatter, backed by [MathJax](https://www.mathjax.org/) for the rendering.
* Provide step-by-step solutions.
* Accept scripts as files.
* Create a workflow as an exportable notebook.

## Citation

.

