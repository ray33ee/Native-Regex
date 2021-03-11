# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Double and triple check ALL bounds checks
- Make sure `CharacterSet::from` contains no overlaps
- Support shorthand character sets (within character sets and as stand alone)
- Support all literal characters
- Add support for {N} and {N,M} repetition

### Unfinished Ideas
- We can split regexes up into 3 different types based on the maximum number of possible captures in a regex:
    - 0: Contains no captures. Return type Option<(index, index, slice)> showing the start and finish index of the match
    - 1<=N<=31: Contains between 1 and 31 captures. Return type Option<[(start, finish, slice); N+1]>
    - N>31: Contains more than 31 captures. Return type Option<Vec<(start, finish, slice)>>.
- Allow multiple literal matching
  - Keep searching until we find a character that's not a literal character
  - If the terminating character is a repeater then stop before the last character and ship as LiteralList
  - Otherwise ship as a LiteralSingle
  - If we have a single charcter with no repeater, ship as LiteralSingle.
  - This one might take a bit more thought...
  - Don't forget escaped characters like \+

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
