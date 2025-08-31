# rspricklybird

## Overview

This repository contains a rust implementation of the [pricklybird](https://github.com/ndornseif/pricklybird) format version `v1` in the `pricklybirdlib` crate.
It also provides a command line utility to convert pricklybird strings to binary data and binary data to pricklybird string called `prbiconv`. 


## Using prbiconv

`prbiconv` is a command line utility for conversion that is written in rust and uses `pricklybirdlib` in the background. 
Input is read from stdin and output written to stdout.

By default conversion from pricklybird string to bytes is attempted.
This can be explicitly set using the `-b` flag.
We use `xxd` in these examples to convert raw binary to hexadecimal.

```console
% echo "flea-flux-full" | prbiconv -b | xxd -ps
4243
```

To convert bytes to a pricklybird string use the `-p` flag.
```console
% echo "4243" | xxd -r -p | prbiconv -p
flea-flux-full
```

## Using pricklybirdlib

```rust
use pricklybirdlib::{convert_to_pricklybird, convert_from_pricklybird};
let data = [0x42_u8, 0x43];
let code = convert_to_pricklybird(&data);
assert_eq!("flea-flux-full", code);
let recovered_data = convert_from_pricklybird(&code).unwrap();
assert_eq!(vec![0x42, 0x43], recovered_data);
```

## License

`rspricklybird`, `pricklybirdlib` and `prbiconv` are distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.