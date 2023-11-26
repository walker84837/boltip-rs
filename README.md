# public-ip-getter-rs

This is a public IP address getter, written in Rust. The program will automatically get your public IP address.

## Table of Contents

1.  [Installation](#installation)
2.  [How to Use](#how-to-use)
3.  [License](#license)

## Installation

To utilize this application, it is necessary to have Rust and its package manager, Cargo, installed. You can install them by following the official [Rust installation guide](https://www.rust-lang.org/tools/install).
Once you have successfully installed Rust and Cargo, you can proceed to build and install this program by executing the following command:

```$ cargo install --path .```

Or you can download the precompiled binaries.

## How to Use

To look up an IP, you need to provide the following command-line arguments:

```$ ./public-ip-getter-rs <verbose> optional: -i [ip]```

  - `<verbose>`: More information will be provided.
  - `<-i, --ip-address>`: The IP address to look up.
  - `[<-r, --random>] <true|false>`: Use a random IP or not.

## License

This project is licensed under the GNU GPLv3. You can find the full text of the license in the [LICENSE](LICENSE) file or visit the [GNU website](https://www.gnu.org/licenses/gpl-3.0.html).
This project uses <https://ident.me> for getting the computer's IP address, and the [iplocation\.net](https://api.iplocation.net/)'s API for geolocation. Please read its Terms of Service [here](https://www.iplocation.net/terms-of-service).

