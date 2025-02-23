# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.1.16] - 2024-04-26

### Changed
- Update `depend/zcash` to version 5.9.0 which includes updated dependencies

## [0.1.15] - 2024-04-19

### Changed
- Update `depend/zcash` to version 5.8.0 which includes updated dependencies
- Update other dependencies to match Zebra
- Restore Windows support

## [0.1.14] - 2023-10-18

### Changed
- Update `depend/zcash` to version 5.7.0 which includes updated dependencies
- Update other dependencies to match Zebra
- Update the `build.rs` to handle subdirs

## [0.1.13] - 2023-06-29

### Changed
- Update `depend/zcash` to version 5.6.1 which includes updated dependencies
- Update other dependencies to match Zebra

## [0.1.12] - 2023-05-03

### Changed
- Update `depend/zcash` to version 5.5.0 which includes updated dependencies
    - This includes additional `zcashd` C++ and Rust code, and its dependencies
    - Fix code generation C++ header paths to avoid conflicts
- Update other dependencies to match Zebra

### Fixed
- Improve error reporting in `build.rs`

## [0.1.11] - 2023-02-24

### Changed
- Make dependencies automatically upgrade to match its parent project if a later minor version is already included
- Upgrade to Rust 2021 edition

## [0.1.10] - 2023-02-23

### Changed
- Updated `bindgen` version from 0.60.1 to 0.64.0

## [0.1.9] - 2023-02-23

### Changed
- Updated `depend/zcash` to version 5.4.0 which includes updated dependencies

## [0.1.8] - 2022-10-31

### Changed
- Updated `depend/zcash` to version 5.3.0 which includes updated dependencies
- Updated `bindgen` to version 0.60.1

## [0.1.7] - 2022-08-31

### Changed
- Updated `depend/zcash` to version 5.2.0 which includes updated dependencies
-
## [0.1.6] - 2022-05-16

### Changed
- Exposed precomputation API
- Updated `depend/zcash` to version 5.0.0 which includes API for V5 transactions

## [0.1.5] - 2020-12-09
### Changed
- Updated `depend/zcash` to new version including a precompute API

## [0.1.4] - 2020-11-17
### Changed
- switched from bindgen `0.55` to bindgen `0.54` to avoid a dependency
  conflict with `rocksdb`

## [0.1.3] - 2020-10-09
### Changed
- Enabled endomorphism optimization
- Changed zcash dependency from a submodule to a subtree

### Fixed
- Can now run `cargo publish` and `cargo release` without errors

## [0.1.2] - 2020-09-21
### Removed
- dependency on `color-eyre` in build.rs

## [0.1.1] - 2020-09-15
### Changed
- enabled the `parallel` feature of `cc` to enable parallel compilation

### Security
- Updated `bindgen` to a non yanked version

<!-- next-url -->
[Unreleased]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.16...HEAD
[0.1.16]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.15...v0.1.16
[0.1.15]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.14...v0.1.15
[0.1.14]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.13...v0.1.14
[0.1.13]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.11...v0.1.12
[0.1.11]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.10...v0.1.11
[0.1.10]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/ZcashFoundation/zcash_script/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/ZcashFoundation/zcash_script/releases/tag/v0.1.0
