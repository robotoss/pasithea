# Pasithea
## _Communicate with the public in an easy way_

A project that allows you to group messages from viewers you say hello to.

- Say Hello Some Nikname
- See windows with all last meesages from this viewer
- ✨Magic ✨

## Features

- Speech Recognition

## Tech

The project is written in Rust. Modules used in the project:

- [coqui-stt] - Safe wrapper around the Coqui STT C library
- [cpal] - Low-level cross-platform audio I/O library in pure Rust.
- [nnnoiseless] - Audio denoising, derived from Xiph's RNNoise library.
- [dasp_signal] - An iterator-like API for audio PCM DSP streams.
- [dasp_interpolate] - An abstraction for audio PCM DSP rate interpolation, including floor, linear and sinc.
- [anyhow] - Flexible concrete Error type built on std::error::Error
- [clap] - A simple to use, efficient, and full-featured Command Line Argument Parser

## Installation

Install [Rust](https://www.rust-lang.org/tools/install) edition = "2021" to run.

Preparation:

1. Obtain a `native_client` library. The [release announcement](https://github.com/coqui-ai/STT/releases) contains precompiled libraries for various targets. (Simple: native_client.tflite.macOS.tar.xz for MacOs)
2. Downloads language models (.pb/.pbmm/.tflite) and scorer(.scorer) from https://coqui.ai/models/. And put them in models directory.

For Linux: 
Add the directory where the `native_client` library lies to your `LD_LIBRARY_PATH` and `LIBRARY_PATH` environment variables.

For MacOs:
Create folder Developer/`native_client`/ and add Update your path: 
```sh
export LD_LIBRARY_PATH="/Users/userName/Developer/native_client:$LD_LIBRARY_PATH"
export DYLD_LIBRARY_PATH="/Users/userName/Developer/native_client:$DYLD_LIBRARY_PATH"
export LIBRARY_PATH="/Users/userName/Developer/native_client:$LIBRARY_PATH"
```
For Widows:
`native_client` extracted to C:\stt.


For project run:

```sh
cargo run
```
