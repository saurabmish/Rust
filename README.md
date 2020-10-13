# Rust Programming


### Install

+ Create `rust` directory in `$HOME/.config/`

+ Add the following in `.zshrc`:

  ```
  export RUSTUP_HOME=$HOME/.config/rust/rustup
  export CARGO_HOME=$HOME/.config/rust/cargo
  ```

  | Environment Variable  | Directory                    |  Description             |
  |:---------------------:|:----------------------------:|:------------------------:|
  | `RUSTUP_HOME`         | `$HOME/.config/rust/rustup`  | metadata and toolchains  |
  | `CARGO_HOME`          | `$HOME/.config/rust/cargo`   | package manager          |

+ Install Rust

  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

  Executables for `cargo`, `rustc`, `rustup` and other commands will be located in:
  `$HOME/.config/rust/cargo/bin`

+ Restart terminal and verify the above `cargo`, `rustc`, and `rustup` directory:

  ```
  which cargo
  which rustc
  which rustup
  ```


### CLI Commands

+ Create directory for the program:

  `mkdir hello_world && cd hello_world`

+ Source code has to be in `main.rs`

+ Compile the file to get an executable:

  `rustc main.rs`

+ Execute the binary executable:

  `./main`


### Using `Cargo`

Cargo is Rust's build system and package manager and is used for project management. It handles a lot of tasks such as building code, downloading libraries the code depends on, as well as building those libraries.

+ Create project:

  `cargo new hello_cargo && cd hello_cargo`

  + `Cargo.toml` contains a list of configurations

  + Inside the `src` directory, `main.rs` contains the source code

+ Create an executable file in target/debug/hello_cargo

  `cargo build`

+ Run executable:

  `./target/debug/hello_cargo`

+ The above two operations (compile and run) can also be done in one command:

  `cargo run`

  **NOTE**: Any change in the source file would make cargo rebuild it.

+ **Additionally**:

  + For large projects, `cargo build` runs slow. To ensure faster compilation, without producing an executable, `cargo check` can be used.

  + For production builds, a project can be compiled with optimizations using `cargo build --release`. The code will run faster but will increase the project's compile time.
  This will create an executable in `target/release` instead of `target/debug`, which can be benchmarked with `./target/release/hello_cargo`


### (Minimal) Emacs Configuration

+ Add stable repository for MELPA in `init.el`:

  ```
  (require 'package)
  (add-to-list 'package-archives
               '("melpa-stable" . "https://stable.melpa.org/packages/") t)
  (package-initialize)
  ```

+ Fetch package list:

  `M-x package-refresh-contents`

+ Install `rust-mode` so that `.rs` files are **not** in 'Fundamental' mode:

  ```
  M-x package-install RET
  rust-mode RET
  ```

  **Or** using Emacs GUI:

  + `M-x list-packages RET`

  + `C-s rust-mode C-s` ... (press `RET` once the cursor is at rust-mode under "Packages")

  + Go to the beginning of the line and press `i` to mark the package(s) for installation

  + Press `x` to install the selected package(s)

