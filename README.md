# public-ip-lookup-rs

This is a public IP address lookup program, written in Rust, which will automatically find information about public IP addresses.

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)

## Installation

To build this application from source, you need to have [Rust](https://rustup.rs) installed. If you prefer not to build from source, you can download the [precompiled binaries](https://github.com/walker84837/public-ip-lookup-rs/releases). After installing Rust and Cargo successfully, execute the following command to build and install the program:

``` console
$ cargo build --release
```

## Usage

To look up an IP, you need to provide the following command-line arguments:

  - `-i, --ip-address`: The IP address to look up.
  - `-r, --random`: Use a random IP or not.
  - `-v, --verbose`: More information will be provided.

## Contributing

Contributions to the `public-ip-lookup-rs` project are always welcome\! If you want to contribute:

  - Follow the [code of conduct](CODE_OF_CONDUCT.md).

  - Follow the same code style. Format your code with `rustfmt` and use `clippy` to catch common errors and polish the modified Rust code:

    ``` console
    $ rustfmt --edition 2021 src/*
    $ cargo clippy
    ```

  - For a reliable development experience suitable for production and compatibility
    with the broader ecosystem, it is recommended to use Rust stable instead of Rust nightly.
    If Rust nightly features are present in this repository's code, they should be replaced with stable equivalents.

  - When using external libraries, it is recommended to choose lightweight options,
    such as `ureq` over `reqwest`, or `async-std` in place of `tokio`.

  - It is recommended to use the standard library instead of creating new solutions from scratch.

  - If proposing significant changes, such as a new feature, please open an issue and provide the
    following information:
    
      - Why should it be added?
      - What value does it bring and why should it be considered?
      - What is the difference between using it and not using it?

## License

This project is licensed under the GNU General Public License, version 3. You can find the full text of the license in the [license](LICENSE.md) file or in the [GNU website](https://www.gnu.org/licenses/gpl-3.0.html).
This project uses <https://ident.me> for getting the device's IP address, and the [iplocation.net's API](https://api.iplocation.net/) for geolocation. Please read its [Terms of Service](https://www.iplocation.net/terms-of-service).
