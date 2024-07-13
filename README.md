# BoltIP: find information about public IP addresses

## Table of Contents

1.  [Installation](#installation)
2.  [Usage](#usage)
3.  [Contributing](#contributing)
4.  [License](#license)

## Installation

You need Rust to build this app. You can download the binaries if you don't want
to build from source. After installing Rust and Cargo, run this command to build
and install the program.

``` console
$ cargo build --release
```

## Usage

To look up an IP, you need to provide the following command-line arguments:

  - `-i, --addr`: The IP address to look up
  - `-r, --random`: Get a random IP for testing
  - `-v, --verbose`: Get more info on the provided IP address
  - `-l, --logging`: Enable logging

## Contributing

Contributions to BoltIP are always welcome! If you want to contribute:

  - Follow the [code of conduct](CODE_OF_CONDUCT.md).
  - Follow the [Rust Style Guide](https://doc.rust-lang.org/beta/style-guide/index.html).
  - If proposing significant changes, such as a new feature, please open an issue.

## License

This project is licensed under the GNU General Public License, version 3. You
can find the full text of the license in the [license](LICENSE.md) file or in
the [GNU website](https://www.gnu.org/licenses/gpl-3.0.html).

This project uses <https://ident.me> for getting the device's IP address, and
the [iplocation.net's API](https://api.iplocation.net/) for geolocation. Please
read its [Terms of Service](https://www.iplocation.net/terms-of-service).
