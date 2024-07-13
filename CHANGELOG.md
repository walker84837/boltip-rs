# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

## [Unreleased]

## [0.2.0] - 2024-07-13

### Added

  - Add clap to parse command line arguments and replace StructOpt.
  - Add logging with `simple_logger`.
  - Add a PKGBUILD for Arch Linux.

### Changed

  - Make the installation steps clearer.
  - Rename project to BoltIP.

## [0.1.2] - 2023-12-15

### Changed

  - Rebranded the repository from `public-ip-getter-rs` to `public-ip-lookup-rs`.
    The motivation behind it is that this repository is a lookup program, not an IP obtainer.
  - Made some improvements to the code for readability:
      - Changed error handling from using `std::error::Error` to `anyhow`.
      - Improved error handling in `response_to_string` function.
      - Added a fallback IP address constant (`FALLBACK_IP`) for error cases.

## [0.1.1] - 2023-11-26

### Added

  - Added random IP address lookup, at the user's choice.

## [0.1.0] - 2023-10-19

### Added

  - Initial release of `public-ip-lookup-rs`.
