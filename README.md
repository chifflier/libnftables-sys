# libnftables Rust Bindings

[![License: GPL-2.0](https://img.shields.io/badge/License-GPL%202.0-yellow.svg)](./LICENSE)
[![Build Status](https://travis-ci.org/chifflier/libnftables-sys.svg?branch=master)](https://travis-ci.org/chifflier/libnftables-sys)
[![Crates.io Version](https://img.shields.io/crates/v/libnftables-sys.svg)](https://crates.io/crates/libnftables-sys)

The `libnftables-sys` crate provides declarations and linkage for the
`libnftables` C library.
The `libnftables-sys` crate provides minimal abstractions over the native
`libnftables` library functions.

## Dependencies
In order to use the `libnftables-sys` crate, you must have the `libnftables`
library installed.

## Usage
Add `libnftables-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
libnftables-sys = "0.1"
```

Import the `libnftables_sys` crate and use the functions as they're defined in
the native `libnftables` library. You can also use the `Nftables` struct, a very
thin layer on top of the native functions.

```rust
extern crate libnftables_sys;
use libnftables_sys::*;

fn main() {
    let mut nft = Nftables::new();

    let cmd = CStr::from_bytes_with_nul(b"list ruleset\0").unwrap();
    let (rc,output,error) = nft.run_cmd(cmd.as_ptr());
}
```

See the [examples](examples/) directory for complete examples.

## License

This crate has the same license as
[libnftables](https://netfilter.org/projects/nftables/), which is GPL version 2
only.
