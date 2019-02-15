# Cross Commander (xcmd)

[![CircleCI](https://circleci.com/gh/xcmd-io/xcmd.svg?style=svg)](https://circleci.com/gh/xcmd-io/xcmd)

Cross Commander is classic dual-pane file manager that allows you to work with files efficiently.

![Screenshot](docs/screenshot.png)

For design documents, see [wiki](https://github.com/xcmd-io/xcmd/wiki).

## Building

~~~
cargo build --release
~~~

To build in WSL, install `pkg-config`, `libssl-dev`, `libgtk-3-dev` packages first.

~~~
sudo apt install pkg-config libssl-dev libgtk-3-dev
~~~
