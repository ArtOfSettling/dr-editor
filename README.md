# Dread Editor

**NOTE**
For now, this project is stand-alone build-able, however, it should primarily be built through the [dread](https://github.com/ArtOfSettling/dread) repo, where it is added as a git submodule. This will change in the future, when this package will ultimately be pushed to [crates.io](https://crates.io).

## Overview

This package represents the editor component of the engine. This package utilizes the [dr-engine](https://github.com/ArtOfSettling/dr-engine) and the [d-runtime](https://github.com/ArtOfSettling/d-runtime) to launch the editor. Once built, you can simply launch the executable to use the editor.

## Prerequisites

1. Install CMake

## How to build

1. `cargo build` / `cargo build --release` {depending on the profile you want to build}
