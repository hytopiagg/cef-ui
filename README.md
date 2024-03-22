# cef-ui

This repository attempts to wrap the Chromium Embedded Framework (CEF) in Rust. CEF is a project that allows you to embed Chromium web views in native applications. You can find out more about it [here](https://github.com/chromiumembedded/cef). A high level overview of CEF can be found [here](https://bitbucket.org/chromiumembedded/cef/wiki/GeneralUsage). Details about C interop can be found [here](https://bitbucket.org/chromiumembedded/cef/wiki/UsingTheCAPI.md).

# Approach

We use Rust `bindgen` to generate bindings around CEF's C API. Unit tests and comments have been stripped out of the generated Rust bindings. Some C types (specifically `#define`'s) have to be added manually.

We partition the generated bindings into separate crates by platform to reduce compilation time. The main crate is called `cef-ui`.

# Installation

More to come.

# Status

This is a work in progress and is nowhere near complete. Many `C` handlers and types have not yet been wrapped in Rust. This will be an ongoing endeavor.

At present, we're targeting version `121.3.15+g4d3b0b4+chromium-121.0.6167.184`. Once we have wrapped the full CEF `C` API, we will consider supporting newer versions.

| Platform | Arch | Builds | Downloads |
| --- | --- | :---: | --- |
| Linux | `x86_64` | :white_check_mark: | [cef_binary_121.3.15+g4d3b0b4+chromium-121.0.6167.184_linux64_minimal](https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_linux64_minimal.tar.bz2) |
| macOS | `arm64` | :white_square_button: | [cef_binary_121.3.15+g4d3b0b4+chromium-121.0.6167.184_macosarm64_minimal](https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_macosarm64_minimal.tar.bz2)
| Windows | `x86_64` | :white_square_button: | [cef_binary_121.3.15+g4d3b0b4+chromium-121.0.6167.184_windows64_minimal](https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_windows64_minimal.tar.bz2) |
