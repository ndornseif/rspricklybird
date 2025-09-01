# rspricklybird

![GitHub License](https://img.shields.io/github/license/ndornseif/rspricklybird)
[![Crate]][crates.io]

-----

## Overview
[`pricklybird`](https://github.com/ndornseif/pricklybird) is a method for conversion of 
arbitrary binary data into more human-friendly words, where each word represents a single byte.  
A CRC-8 checksum is attached to allow the detection of errors during decoding.  
`0xDEADBEEF` becomes `turf-port-rust-warn-void`, for example.  

This repository contains `pricklybirdlib`, a rust implementation `pricklybird` version `v1`.

It also provides a command line utility to convert from and to pricklybird strings called `prbiconv`. 

## pricklybirdlib

Find the [README](pricklybirdlib/README.md) here.

## prbiconv

### Usage

`prbiconv` is a command line utility for conversion that is written in rust and uses `pricklybirdlib` in the background. 
Input is read from stdin and output written to stdout.

By default conversion from pricklybird string to bytes is attempted.
This can be explicitly set using the `-b` flag.
We use `xxd` in these examples to convert raw binary to hexadecimal.

```console
$ echo "flea-flux-full" | prbiconv -b | xxd -ps
4243
```

To convert bytes to a pricklybird string use the `-p` flag.
```console
$ echo "4243" | xxd -r -p | prbiconv -p
flea-flux-full
```

### Building from source

```console
$ git clone https://github.com/ndornseif/rspricklybird.git
$ cd rspricklybird/prbiconv
$ cargo build --release
```

## License

`pricklybirdlib` and `prbiconv` are distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.

[crates.io]: https://crates.io/crates/pricklybirdlib
[Crate]: https://img.shields.io/crates/v/pricklybirdlib