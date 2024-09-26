# cef-ui

This repository wraps the Chromium Embedded Framework (CEF) in Rust. CEF is a project that allows you to embed Chromium web views in native applications. You can read more about it [here](https://github.com/chromiumembedded/cef). A high level overview of CEF can be found [here](https://bitbucket.org/chromiumembedded/cef/wiki/GeneralUsage). Details about CEF C interop can be found [here](https://bitbucket.org/chromiumembedded/cef/wiki/UsingTheCAPI.md).

## Approach

Rust `bindgen` is used to generate bindings against CEF's C API. Unit tests and comments have been stripped from the generated Rust bindings. Missing C types (like `#define`'s) have been added manually. All generated CEF types are wrapped in sane Rust types that respect CEF's reference counting rules.

Here is an overview of all crates in the repository:

| Crate | Purpose |
| --- | --- |
| `cef-ui` | The main CEF wrapper. |
| `cef-ui-helper` | The CEF helper wrapper (required on macOS). |
| `cef-ui-tools` | Tools for building the various binaries, and for generating the app bundle on macOS. |
| `cef-ui-util` | A library that contains tooling commands, functions to simplify linking, etc. |
| `cef-ui-simple` | The main binary for the simple example. |
| `cef-ui-simple-helper` | The helper binary for the simple example (required on macOS). |

The `cef-ui-tools` crate exists to deal with platform-specific issues in pure Rust. For example, on macOS, CEF will only function if it adheres to its [app bundle structure](https://bitbucket.org/chromiumembedded/cef/wiki/GeneralUsage.md#markdown-header-macos).

## Installation

If you're using VSCode, there are `Debug` and `Release` configurations. If you're on the command line, you can do the following:

```
cargo cef-build-debug
cargo cef-build-release
```

Once built, you can find the `cef-ui-simple` binary in the appropriate `target` folder. On macOS, you must run the generated app bundle.

The CEF binaries themselves are very large. We have provided stripped, postprocessed binaries as release artifacts on GitHub. When you create your binary targets that link against CEF, you must call `link_cef` and `link_cef_helper` (if on macOS) in your `build.rs` scripts. These functions automatically download the necessary CEF binaries into a folder in your repository (controlled by the `CEF_WORKSPACE_DIR` and `CEF_ARTIFACTS_DIR` environment variables) and copies the necessary binaries and files to the target directory alongside your executable.

The simple examples are built via binary targets in `cef-ui-tools`. On macOS, this builds not only `cef-ui-simple`, but also `cef-ui-simple-helper`, and generates the app bundle which is required by CEF.

## Status

This is a work in progress and is not complete. Many `C` types have not yet been wrapped. At present, we target version `121.3.15+g4d3b0b4+chromium-121.0.6167.184`.

| Platform | Arch | Builds | Downloads |
| --- | --- | :---: | --- |
| Linux | `x86_64` | :heavy_check_mark: | [cef_binary_121.3.15+g4d3b0b4+chromium-121.0.6167.184_linux64_minimal](https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_linux64_minimal.tar.bz2) |
| macOS | `arm64` | :heavy_check_mark: | [cef_binary_121.3.15+g4d3b0b4+chromium-121.0.6167.184_macosarm64_minimal](https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_macosarm64_minimal.tar.bz2)
| Windows | `x86_64` | :heavy_check_mark: | [cef_binary_121.3.15+g4d3b0b4+chromium-121.0.6167.184_windows64_minimal](https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_windows64_minimal.tar.bz2) |
