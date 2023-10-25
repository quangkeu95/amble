# amble

[![CI Build Status]][actions]
[![Release]][actions]
[![Tag Build Status]][actions]
[![License]][mit-license]
[![Docs]][Docs-rs]
[![Latest Version]][crates.io]
[![rustc 1.31+]][Rust 1.31]

[CI Build Status]: https://img.shields.io/github/actions/workflow/status/refcell/amble/ci.yml?branch=main&label=build
[Tag Build Status]: https://img.shields.io/github/actions/workflow/status/refcell/amble/tag.yml?branch=main&label=tag
[Release]: https://img.shields.io/github/actions/workflow/status/refcell/amble/release.yml?branch=main&label=release
[actions]: https://github.com/refcell/amble/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/amble.svg
[crates.io]: https://crates.io/crates/amble
[rustc 1.31+]: https://img.shields.io/badge/rustc_1.31+-lightgray.svg
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[License]: https://img.shields.io/badge/license-MIT-7795AF.svg
[mit-license]: https://github.com/refcell/amble/blob/main/LICENSE.md
[Docs-rs]: https://docs.rs/amble/
[Docs]: https://img.shields.io/docsrs/amble.svg?color=319e8c&label=docs.rs


**First class, scalable rust project generator with batteries included.** Amble is https://github.com/refcell/amble/labels/stable

![](./etc/banner.png)

**[Install](#usage)**
| [User Docs](#what-is-amble)
| [Crate Docs][crates.io]
| [Reference][Docs-rs]
| [Contributing](#contributing)
| [License](#license)

## What is amble?

`amble` is a cli application for generating rust projects
with batteries included. By using a workspace and
sub-crates, `amble` scales to any number of crates.

You can think of `amble` as an extension of `cargo init`.
Cargo provides [convenience methods][c_new]
for generating new project structures, the two primary
types being a library or a binary project. Below is an
example of the directory structure generated by
`cargo init --lib`. (A binary project would only differ
in having a `main.rs` file instead of `lib.rs`).

```yaml
project
├─ Cargo.toml
└─ src
   └─ lib.rs
```

`amble` provides a more extensive cli command for generating
project directory structures with [workspaces][Workspaces].
Library sub-crates are nested inside the cannonical
`crates/` directory, while binary crates are nested inside
the `bin/` directory. A top-level `Cargo.toml` file defines
a workspace including the binaries and libraries as well as
shared package metadata and dependencies, minimizing
inconsistencies across the sub-crates. `amble` also exposes
the `--lib` and `--bin` flags which fallback to `cargo init`
and create the default `cargo init` file structure with all
`amble` batteries included and available through cli flags.

Below is an example waterfall directory structure output
for when `amble` is run with the argument `project` as
the project name.

```yaml
project
├─ Cargo.toml
├─ bin
│  └─ example
│     ├─ Cargo.toml
│     └─ src
│        └─ main.rs
└─ crates
   └─ common
      ├─ Cargo.toml
      └─ src
         └─ lib.rs
```

*As detailed in [Usage](#usage) below, this output is generated by running `amble project --dry-run`.*

[c_new]: https://github.com/rust-lang/cargo/blob/master/src/cargo/ops/cargo_new.rs
[Workspaces]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

## Usage

Install `amble` using cargo.

```yaml
cargo install amble
```

Alternatively, `amble` can be built from source.

```yaml
git clone git@github.com:refcell/amble.git && cd amble
cargo build --release
```

To run `amble`, it is recommended to first run it in dry mode which will print
a waterfall directory structure for the files that would be created when run
without dry mode. Simply run `amble <project_name> --dry-run`, where `<project_name>`
is the name of the project/directory you want to create.

By default, `amble` uses the current directory for the project name, so it
is perfectly acceptable to run `amble --dry-run`, which will just use `.`
as the project path. *Note, amble will exit if it finds cargo artifacts
in the directory you chose to execute in.*

To run `amble` out of dry mode, just run `amble`!

A pre-defined github action ci workflow can be automatically generated by
passing in the `--with-ci` flag to `amble`. This would generate a directory
structure as follows (`amble --dry-run --with-ci`).

```yaml
.
├─ Cargo.toml
├─ bin
│  └─ example
│     ├─ Cargo.toml
│     └─ src
│        └─ main.rs
├─ crates
│  └─ common
│     ├─ Cargo.toml
│     └─ src
│        └─ lib.rs
└─ .github
   └─ workflows
      └─ ci.yml
```

Amble also provides fallthrough methods for generating
`cargo init` library and binary projects with batteries such as
a templated readme, github action ci workflows, and licensing.
The flags to pass through to `cargo init` operations are injective,
meaning `amble --bin` will pass through to `cargo init --bin` and
`amble --lib` will pass through to `cargo init --lib`. As usual,
any other valid amble flag can be provided along with the `--lib`
and `--bin` flags.

#### CLI Flags

Below is an inexhaustive list of the main cli flags.
These are subject to change and new ones can be added.
To view a more up-to-date list, run the `amble --help` command locally.

```yaml
First class, scalable rust project generator with batteries included.

Usage: amble [OPTIONS] [PROJECT_DIR]

Arguments:
  [PROJECT_DIR]  The path to the project directory. By default, the current working directory is used. If any rust artifacts are detected in the specified or unspecified directory, an error will be thrown [default: .]

Options:
  -v, --v...                         Verbosity level (0-4)
      --dry-run                      Dry run mode. If this flag is provided, the cli will not execute commands, printing the directories and files that would be created instead
      --overwrite                    Overwrite existing files. If this flag is provided, the cli will overwrite existing files
  -n, --name <NAME>                  The project name. This will be used for the binary application name [default: example]
  -w, --with-ci                      Add github actions ci workflow
  -c, --ci-yml <CI_YML>              Copy the specified ci workflow file to the project's `.github/workflows/` directory
  -a, --authors <AUTHORS>            Override the project authors
  -b, --bin                          Builds a cargo binary project
  -l, --lib                          Builds a cargo library project
      --full                         Full generates a full project structure including license, ci, gitignore, etc
      --etc                          Adds an `etc/` directory to the project. This _Et Cetera_ directory is used for storing miscellaneous files
      --license                      Adds an MIT License to the project. The MIT License type can be overridden with the `--with-license` flag
      --gitignore                    Adds a Gitignore file to the project
  -d, --description <DESCRIPTION>    Specifies the description of the project in the top-level `Cargo.toml` workspace
      --dependencies <DEPENDENCIES>  Adds these dependencies to the top-level `Cargo.toml` workspace alongside the default dependencies
      --list                         Lists the default dependencies
      --with-license <WITH_LICENSE>  License Override. This will override the default MIT License. The license type must be a valid SPDX license identifier
  -h, --help                         Print help
  -V, --version                      Print version
```

*You can generate this output by running `amble --help`.*

## Contributing

All contributions are welcome! Experimentation is highly encouraged and new issues are welcome.

## Troubleshooting & Bug Reports

Please check existing issues for similar bugs or
[open an issue](https://github.com/refcell/amble/issues/new)
if no relevant issue already exists.

## License

This project is licensed under the [MIT License](LICENSE.md).
Free and open-source, forever.
*All our rust are belong to you.*
