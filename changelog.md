# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Make sure `CharacterSet::from` contains no overlaps
- Finish off AST translator
  - Support repeaters
  - Make sure to break out of for loops and continue in main loop when rejecting

### Unfinished Ideas

## [0.1.5] - 2021-03-12

### Added
- Support for repeaters
- NO_MATCH now works correctly
- Wrapper function to add the base code to translator 
- Correct handling of captures using `vec!`

### Fixed
- Inverter bug fixed on fixed character token translation
- Double brackets around Some
- Added one to `capture_index` since index 0 is saved for the first capture, the entire match

### Changed
- Moved examples to docs folder

### Removed
- Macros and tests

## [0.1.4] - 2021-03-11

### Added
- Support for literal characters
- Support for a list of literals
- Added a simple function `NativeRegexAST::tree` that displays the contents of the AST
- Added process.md
- translate.rs to convert AST into Rust source
- Figured out bounds checks (see process.md)
- Figured out return value of Rust functions (see process.md)


## [0.1.3] - 2021-03-11

### Added
- Add support for {N}, {N,} and {N,M} repetition
- Support for '^' in character classes
- Support for shorthand classes within character classes
- Support for shorthand classes outside of character classes and literal escaped characters

## [0.1.2] - 2021-03-09

### Added
- Structure and most of the code for AST including
  - Recursively walking the regex
  - Obtaining repetition suffixes
  - Parsing character sets (not shorthand sets yet)

## [0.1.1] - 2021-03-09

### Added
- Added MIT license
- Tests for macros
- Some basic hard-coded regexes

### Changed
- Macros moved to dedicated macro.rs module

### Fixed
- Logic and `SUBSTR_LEN` issues within `indexed_native_searcher` fixed.

## [0.1.0] - 2021-03-09

### Added
- Initial commit
- Very basic macro to search for literal strings (represented as a sequence of characters sent to the macro)
