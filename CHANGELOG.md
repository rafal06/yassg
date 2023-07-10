# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
> **Warning**  
> This version contains breaking changes! Please read the *Changed* section for more information.

### Added
- Implement proper CLI, with help, version info, etc.
- Include a `public` directory in the built site
- Implement logger and a verbosity flag
- Add a `--watch` flag to automatically rebuild the project on file change

### Changed
- [**Breaking**] Look for HTML files in `src/` directory instead of the project root   
  Same goes for the components directory, now it should be located in `src/components`

### Fixed
- Fix detecting component tags with no parameters
- Fix text shown on generating a site from the current directory

## 0.1.1 - 2023-02-18
### Fixed
- Fix detecting component tags with line breaks

### Changed
- Improve performance in projects with larger number of pages

## 0.1.0 - 2023-02-16
### Added
- Support multiple files
- Support multiple components
- Support multiple component parameters
