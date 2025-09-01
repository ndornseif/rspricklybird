# pricklybirdlib
![GitHub License](https://img.shields.io/github/license/ndornseif/rspricklybird)
[![Crate]][crates.io]

-----

## Overview

[`pricklybird`](https://github.com/ndornseif/pricklybird) is a method for conversion of 
arbitrary binary data into more human-friendly words, where each word represents a single byte.  
A CRC-8 checksum is attached to allow the detection of errors during decoding.  
`0xDEADBEEF` becomes `turf-port-rust-warn-void`, for example.  
`pricklybirdlib` is a rust implementation `pricklybird` version `v1`.

## Documentation

Documentation is hosted on on [docs.rs](https://docs.rs/pricklybirdlib/latest/pricklybirdlib/).

## Usage

Basic conversion functions that fully comply with the specification and 
include the CRC can be used as follows.

```rust
use pricklybirdlib::{convert_to_pricklybird, convert_from_pricklybird};
let data = [0x42_u8, 0x43];
let words = convert_to_pricklybird(&data);
// Notice the third word "full" used to encode the CRC.
assert_eq!("flea-flux-full", words);
let recovered_data = convert_from_pricklybird(&words).unwrap();
assert_eq!(vec![0x42, 0x43], recovered_data);
```

Is is also possible to map word to bytes and bytes to words without the 
full standard implementation and CRC.
The words are encoded as four bytes of ASCII compatible UTF-8, 
since the wordlist contains no non ASCII characters and all words are four letters long.

```rust
use pricklybirdlib::{words_to_bytes, bytes_to_words};
let data = [0x42_u8, 0x43];
let words = bytes_to_words(&data);
 // Notice that no CRC is attached, the bytes represent the words: "flea", "flux"
assert_eq!(vec![[102, 108, 101, 97], [102, 108, 117, 120]], words);
let data = words_to_bytes(&words).unwrap();
assert_eq!(vec![0x42, 0x43], data); 
```

The `constants` module allows direct access to the `WORDLIST` used for 
mapping bytes to words, and the `HASH_TABLE` use to map words to bytes.

```rust
use pricklybirdlib::constants::{word_hash, HASH_TABLE, WORDLIST};
// Confirm that the word flux maps to the byte 0x43 in both directions.
let word = "flux".as_bytes();
let table_index = word_hash(word[0], word[3]);
let byte_value = HASH_TABLE[table_index];
assert_eq!(0x43, byte_value);
assert_eq!("flux", WORDLIST[0x43])
```

## License

`pricklybirdlib` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.

[crates.io]: https://crates.io/crates/pricklybirdlib
[Crate]: https://img.shields.io/crates/v/pricklybirdlib