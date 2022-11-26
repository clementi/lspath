# lspath

[![CICD](https://github.com/clementi/lspath/actions/workflows/rust.yml/badge.svg)](https://github.com/clementi/lspath/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/lspath-paged)](https://crates.io/crates/lspath-paged)

A command-line app that lists the contents of the PATH environment variable, with optional paging.

## Installation

You can install lspath either by using Cargo, or by downloading a binary from the Releases page.

### Cargo

Run this in your shell:

```sh
$ cargo install lspath-paged
```

### Binaries

See the [Releases](https://github.com/clementi/lspath/releases) page.

## Usage

```
Usage: lspath [OPTIONS]

Options:
  -p, --page
          Page output

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```
