# GNOME Online Accounts

[![Documentation](https://docs.rs/goa/badge.svg)](https://docs.rs/goa)

The Rust bindings for libgoa generated by GIR.

A simpler API is planed.

## Usage

### Install libgoa

- Fedora

  ```none
  # dnf install gnome-online-accounts-devel
  ```

- Debian and derived

  ```none
  # apt install libgoa-1.0-dev
  ```

### Add the dependency

```toml
[dependencies]
goa = "0.0.3"
```

## Generate and build from scratch

Install [GIR](https://github.com/gtk-rs/gir).

```none
git clone --recurse https://gitlab.gnome.org/World/Rust/libgoa-rs
cd libgoa-rs/goa-sys && gir # Generates the goa-sys crate
cd .. && gir # Generates the goa crate
cargo build
```

## License

Either MIT or Apache 2.0, left to the user's choice.
