# cef-ui

This repository wraps the Chromium Embedded Framework (CEF) in Rust. CEF is a project that allows you to embed Chromium web views in native applications. You can read more about it [here](https://github.com/chromiumembedded/cef). A high level overview of CEF can be found [here](https://bitbucket.org/chromiumembedded/cef/wiki/GeneralUsage). Details about CEF C interop can be found [here](https://bitbucket.org/chromiumembedded/cef/wiki/UsingTheCAPI.md).

## Approach

We use Rust `bindgen` to generate bindings around CEF's C API. Unit tests and comments have been stripped out of the generated Rust bindings. Some C types (like `#define`'s) have been added manually.

All generated CEF types are wrapped in sane Rust types. This is to simplify things like reference counting and make things easy to use.

## Crates

Here is an overview of all crates in the repository:

| Crate | Purpose |
| --- | --- |
| `cef-ui` | The full CEF implementation. |
| `cef-ui-simple` | A simple CEF example. |
| `cef-ui-simple-helper` | The helper executable for the simple CEF example (required on macOS). |
| `cef-ui-tools` | Binary targets to build the project. |

The `cef-ui-tools` crate exists to deal with platform-specific issues in pure Rust. For example, on macOS, CEF will only function if it adheres to its [app bundle structure](https://bitbucket.org/chromiumembedded/cef/wiki/GeneralUsage.md#markdown-header-macos).

## Installation

Run configurations exist if you're using VSCode. More to come.

## Status

This is a work in progress and is nowhere near complete. Many `C` types have not yet been wrapped. At present, we target version `121.3.15+g4d3b0b4+chromium-121.0.6167.184`.

| Platform | Arch | Builds | Downloads |
| --- | --- | :---: | --- |
| Linux | `x86_64` | :white_check_mark: | [cef_binary_121.3.15+g4d3b0b4+chromium-121.0.6167.184_linux64_minimal](https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_linux64_minimal.tar.bz2) |
| macOS | `arm64` | :white_check_mark: | [cef_binary_121.3.15+g4d3b0b4+chromium-121.0.6167.184_macosarm64_minimal](https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_macosarm64_minimal.tar.bz2)
| Windows | `x86_64` | :white_check_mark: | [cef_binary_121.3.15+g4d3b0b4+chromium-121.0.6167.184_windows64_minimal](https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_windows64_minimal.tar.bz2) |
