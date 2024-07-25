# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Compatibility

- Update MSRV to 1.70

## [0.2.1] - 2021-12-23

### Features

- Allow building up Roffs from each other

## [0.2.0] - 2021-12-23

### Breaking Changes

The API changed to be a generic ROFF writer, taking care of
escaping, etc.   At the moment, handling of control lines is left up to the
caller

## [0.1.0] - 2018-05-08

<!-- next-url -->
[Unreleased]: https://github.com/rust-cli/roff-rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/rust-cli/roff-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/assert-rs/assert_cmd/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/assert-rs/assert_cmd/compare/21f419c71f025ef596e7954d62506ff8fe3fd7a2...v0.1.0
