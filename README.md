# bongocat-rust

## About
Port of https://github.com/Barkuto/bongocat in 🦀 Rust 🦀

Only tested in Windows 10

## Requirements

- Rust 1.82 or later
- [CMake](https://cmake.org/), and a C++ toolchain for building SFML
- On Linux, you need the following dependencies installed:
  - Window module: `libGL libX11 libXcursor libXrandr`
  - Graphics module: `libfreetype`
  - Audio module: `libopenal libvorbisenc libvorbisfile libvorbis`

## Instruction
- `git clone https://github.com/ItsCacia/bongocat-rust.git`
- `cd bongocat-rust`
- Use the wiki to setup rust-sfml: https://github.com/jeremyletang/rust-sfml/wiki
  - (Windows only) Put the .dll files inside the root folder, they will automatically be copied to the build folder
- `cargo run`
