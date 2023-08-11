# GNOME Online Accounts

[![Crate](https://img.shields.io/crates/v/goa.svg)](https://crates.io/crates/goa)
[![Documentation](https://docs.rs/goa/badge.svg)](https://docs.rs/goa)
[![dependency status](https://deps.rs/crate/goa/0.0.3/status.svg)](https://deps.rs/crate/goa/0.0.3)

The Rust bindings for libgoa generated by [gir](https://github.com/gtk-rs/gir).

## Usage

### Install libgoa

- Fedora

  ```
  # dnf install gnome-online-accounts-devel
  ```

- Debian and derived

  ```
  # apt install libgoa-1.0-dev
  ```

- Arch

  ```
  # pacman -S libgoa
  ```

### Add the dependency

```toml
[dependencies]
goa = "0.0.3"
```

## Generate and build from scratch

You do not need to generate the bindings yourself to use this crate. You can use it just like any other from crates.io. If you do want to regenerate the bindings, follow these steps:

### Get the source

Clone the repository AND the submodules "gir" and "gir-files".

```bash
git clone --recurse-submodules -j8 https://gitlab.gnome.org/World/Rust/libgoa-rs.git
git pull --recurse-submodules
cd ./libgoa-rs
```

### Update Goa gir file
If you have an updated Goa-1.0.gir file, you can just replace the file with the same name in the root of the repo.

### Update the submodules
In order to profit from the constant development of gir, update the submodules.

```bash
git submodule update --remote
```

### Install [GIR](https://github.com/gtk-rs/gir)
Gir is the software that generates the code for us.

```bash
cd gir
cargo install --path .
cd ..
```

If the executable can't be found, you have to add it to your PATH environment variable
`export PATH=$PATH:~/.cargo/bin`.


### Generate the unsafe -sys crate
```bash
cd goa/goa-sys
gir -o .       # Regenerate the bindings
cargo build    # Build the created bindings
cargo test     # Test the created bindings
cd ..
```

There should not have been any errors.

### Generate the safe wrapper

```bash
gir -o .       # Regenerate the wrapper
cargo build    # Build the created wrapper
cargo test     # Test the created wrapper
```

There should not have been any errors.
If you want to learn more about gir, have a look at its [repo](https://github.com/gtk-rs/gir) or its [book](https://gtk-rs.org/gir/book/).

## Maintenance status
[![maintenance-status: passively-maintained (as of 2023-08-06)](https://img.shields.io/badge/maintenance--status-passively--maintained_%28as_of_2023--08--06%29-forestgreen)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)
This crate is just an unsafe wrapper for the C library so it is feature complete and not actively worked on. If you encounter any problems, feel free to open a PR.

## License

Either MIT or Apache 2.0, left to the user's choice.