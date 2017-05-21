# Thermite 
Thermite is a basic kernel I'm implementing to learn about
low level OS development. I don't expect much to come of it.

For now I'm only targeting x86_64, and the core of the kernel
is implemented in [Rust](https://rust-lang.org).

# Build
To build Thermite as a .iso on Ubuntu:

Install Rustup and Thermite dependencies
```
# Rustup install script
$ curl https://sh.rustup.rs -sSf | sh   

# Add rustup binaries to path. You can add this to your .*rc
# file if you'd like.
$ source $HOME/.cargo/env

# Uses nightly rust compiler. We use some bleeding edge
# features which aren't in stable releases yet.
$ rustup override set nightly

# Rust source is needed to build Rust Core for Thermite.
$ rustup component add rust-src

# Rust cross-compilation tool
$ cargo install xargo
```

Clone repository and build:
```
$ git clone https://github.com/evandhiggins/thermite.git $PROJ_DIR

$ cd $PROJ_DIR

$ make iso
```
This will build an iso to `build/thermite-x86_64.iso`.

To run in Qemu on Ubuntu:
```
$ sudo apt install qemu

$ make run
```
