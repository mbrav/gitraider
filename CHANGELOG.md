# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] - 2023-04-10

### Added

- More functional `--dry` run flag;
- Preliminary push support
  - SSH authentication with remote through *libgit2* still needs to be figured out.

## [0.1.5] - 2023-04-09

### Fixed

- Check for changed to avoid creating empty commits on every run

## [0.1.4] - 2023-04-08

### Added

- Checkout and commit error checks;
- Stage only files that were matched rather than staging all files before creating a commit.

## [0.1.3] - 2023-04-07

### Changed

- Rename binary from `git-raider` to `gitraider`
  
## [0.1.2] - 2023-04-04

Test release

## [0.1.1] - 2023-04-04

### Added

- Added `--push` tag (not yet functional)
- Added unittest to `raider.rs`

## [0.1.0] - 2023-04-03

This release marks the first full realization of project's description.

### Fixed

- Fix git commit error

## [0.0.3] - 2023-04-03

### Added

- Added better README.md

## [0.0.2] - 2023-04-01

### Fixed

- Applied some `clippy` recommendations
- Fixed GitHub actions cache for *target/* directory

## [0.0.1] - 2023-04-01

### Added

- Add project

### Removed

- None

[0.1.6]: https://github.com/mbrav/git_raider/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/mbrav/git_raider/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/mbrav/git_raider/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/mbrav/git_raider/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/mbrav/git_raider/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/mbrav/git_raider/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/mbrav/git_raider/compare/0.0.3...0.1.0
[0.0.3]: https://github.com/mbrav/git_raider/compare/0.0.2...0.0.3
[0.0.2]: https://github.com/mbrav/git_raider/compare/0.0.1...0.0.2
[0.0.1]: https://github.com/mbrav/git_raider/releases/tag/0.0.1
