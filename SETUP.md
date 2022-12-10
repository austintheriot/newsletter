# Setup

- [Setup](#setup)
  - [Getting Set Up](#getting-set-up)
  - [Prerequisites](#prerequisites)
    - [Rust](#rust)
    - [Linkers](#linkers)
    - [Docker](#docker)
    - [PostgreSQL](#postgresql)
    - [sqlx-cli](#sqlx-cli)
  - [Remaining Setup](#remaining-setup)
  - [Troubleshooting](#troubleshooting)
    - [Linux](#linux)
      - [Docker error: Permission denied while connecting to daemon socket](#docker-error-permission-denied-while-connecting-to-daemon-socket)
      - [Docker error: Port in use](#docker-error-port-in-use)
      - [Missing OpenSSL installation](#missing-openssl-installation)
  - [Nice-to-haves](#nice-to-haves)

## Getting Set Up

You will need to have the following things installed locally on your machine:

## Prerequisites

### [Rust](https://www.rust-lang.org/tools/install)

### [Linkers](.cargo/config.toml)

### [Docker](https://docs.docker.com/engine/install/)

### [PostgreSQL](https://www.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows/)

### sqlx-cli

```sh
cargo install --version="~0.6" sqlx-cli --no-default-features --features rustls,postgres
```

Check installation with

```sh
sqlx --help
```

## Remaining Setup

Give execute permission to database script:

```sh
chmod +x scripts/init_db.sh
```

Setup database:

```sh
scripts/init_db.sh
```

Give permission to git hooks to run:

```sh
chmod ug+x .git/hooks/*
```

Giver permission to dev script to execute:

```sh
chmod +x scripts/dev.sh
```

Copy files from .git-hook-samples into local .git folder:

```sh
cp .git-hook-samples/pre-commit .git/hooks && cp .git-hook-samples/pre-push .git/hooks
```

## Troubleshooting

### Linux

#### Docker error: Permission denied while connecting to daemon socket

```sh
Got permission denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock: Get "http://%2Fvar%2Frun%2Fdocker.sock/v1.24/containers/json?filters=%7B%22name%22%3A%7B%22postgres%22%3Atrue%7D%7D": dial unix /var/run/docker.sock: connect: permission denied
```

Give Docker permissions to run without the `sudo` command (see [this article](https://linuxhandbook.com/docker-permission-denied/)):

```sh
sudo usermod -aG docker $USER
```

#### Docker error: Port in use

If you run into a similar error to this:

```sh
docker: Error response from daemon: driver failed programming external connectivity on endpoint postgres_1670526252 (ff0025ac986365a798e374eb74ab74dcf3d2db686e6c938d97cea9c3e9f2a390): Error starting userland proxy: listen tcp4 0.0.0.0:5432: bind: address already in use.
```

You can see what is running on port 5432:

```sh
sudo ss -lptn 'sport = :5432'
```

and you can kill whatever process is running on that port with that process' `pid`:

```sh
sudo kill <pid>
```

#### Missing OpenSSL installation

```sh
error: failed to run custom build command for `openssl-sys v0.9.78`

Caused by:
  process didn't exit successfully: `/home/austin/Code/newsletter-api/target/debug/build/openssl-sys-fbb4cbe5ef8d9301/build-script-main` (exit status: 101)
  --- stdout
  cargo:rustc-cfg=const_fn
  cargo:rustc-cfg=openssl
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
    
  ...

  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  run pkg_config fail: "`\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"` did not exit successfully: exit status: 1\nerror: could not find system library 'openssl' required by the 'openssl-sys' crate\n\n--- stderr\nPackage openssl was not found in the pkg-config search path.\nPerhaps you should add the directory containing `openssl.pc'\nto the PKG_CONFIG_PATH environment variable\nNo package 'openssl' found\n"

  --- stderr
  thread 'main' panicked at '

  Could not find directory of OpenSSL installation, and this `-sys` crate cannot
  proceed without this knowledge. If OpenSSL is installed and this crate had
  trouble finding it,  you can set the `OPENSSL_DIR` environment variable for the
  compilation process.

  Make sure you also have the development packages of openssl installed.
  For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

  If you're in a situation where you think the directory *should* be found
  automatically, please open a bug at https://github.com/sfackler/rust-openssl
  and include information about your system as well as this message.

  $HOST = x86_64-unknown-linux-gnu
  $TARGET = x86_64-unknown-linux-gnu
  openssl-sys = 0.9.78

  ', /home/austin/.cargo/registry/src/github.com-1ecc6299db9ec823/openssl-sys-0.9.78/build/find_normal.rs:191:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
```

Install `libssl-dev`:

```sh
sudo apt-get install libssl-dev
```

## Nice-to-haves

- cargo-watch (for re-compiling and re-running on changes)

```sh
cargo install cargo-watch
```

- cargo-tarpaulin (code coverage for x86_64 Linux only) (for checking test coverage)

```sh
cargo install cargo-tarpaulin
```

- cargo-audit (for detecting security vulnerabilities)

```sh
cargo install cargo-audit
```

- cargo expand (for inspecting the generated code from macros)

install:

```sh
# --allow-downgrade allows rustup to find latest nightly with all available components
rustup toolchain install nightly --allow-downgrade && cargo install cargo-expand
```

run:

```sh
cargo +nightly expand
```

- cargo-udeps (for checking for unused dependencies)

install:

```sh
cargo install cargo-udeps
```

run:

```sh
cargo +nightly udeps
```
