# Rust Programming


### Install

+ Create `rust` directory `$HOME/.config/`

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


### (Minimal) Emacs Configuration for Rust

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


### CLI Commands

+ Create directory for a program:

  `mkdir hello_world && cd hello_world`

+ Compile the file to get an executable:

  `rustc main.rs`

+ Execute the binary executable:

  `./main`
