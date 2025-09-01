//! 
//! ![GitHub License](https://img.shields.io/github/license/ndornseif/rspricklybird)
//! [![Crate]][crates.io]
//! 
//! -----
//! 
//! ## Overview
//! 
//! [`pricklybird`](https://github.com/ndornseif/pricklybird) is a method for conversion of 
//! arbitrary binary data into more human-friendly words, where each word represents a single byte.  
//! A CRC-8 checksum is attached to allow the detection of errors during decoding.  
//! `0xDEADBEEF` becomes `turf-port-rust-warn-void`, for example.  
//!
//! `pricklybirdlib` is a rust implementation `pricklybird` version `v1`.
//! 
//! ## Documentation
//!
//! Documentation is hosted on on [docs.rs](https://docs.rs/pricklybirdlib/latest/pricklybirdlib/).
//!
//! ## Usage
//! 
//! Basic conversion functions that fully comply with the specification and 
//! include the CRC can be used as follows.
//! 
//! ```rust
//! use pricklybirdlib::{convert_to_pricklybird, convert_from_pricklybird};
//! let data = [0x42_u8, 0x43];
//! let words = convert_to_pricklybird(&data);
//! // Notice the third word "full" used to encode the CRC.
//! assert_eq!("flea-flux-full", words);
//! let recovered_data = convert_from_pricklybird(&words).unwrap();
//! assert_eq!(vec![0x42, 0x43], recovered_data);
//! ```
//! 
//!
//! Is is also possible to map word to bytes and bytes to words without the 
//! full standard implementation and CRC.
//! The words are encoded as four bytes of ASCII compatible UTF-8, 
//! since the wordlist contains no non ASCII characters and all words are four letters long.
//! 
//! ```rust
//! use pricklybirdlib::{words_to_bytes, bytes_to_words};
//! let data = [0x42_u8, 0x43];
//! let words = bytes_to_words(&data);
//!  // Notice that no CRC is attached, the bytes represent the words: "flea", "flux"
//! assert_eq!(vec![[102, 108, 101, 97], [102, 108, 117, 120]], words);
//! let data = words_to_bytes(&words).unwrap();
//! assert_eq!(vec![0x42, 0x43], data); 
//! ```
//! 
//!
//! The `constants` module allows direct access to the `WORDLIST` used for 
//! mapping bytes to words, and the `HASH_TABLE` use to map words to bytes.
//! 
//! ```rust
//! use pricklybirdlib::constants::{word_hash, HASH_TABLE, WORDLIST};
//! // Confirm that the word flux maps to the byte 0x43 in both directions.
//! let word = "flux".as_bytes();
//! let table_index = word_hash(word[0], word[3]);
//! let byte_value = HASH_TABLE[table_index];
//! assert_eq!(0x43, byte_value);
//! assert_eq!("flux", WORDLIST[0x43])
//! ```
//! 
//! ## License
//! 
//! `pricklybirdlib` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.
//! 
//! [crates.io]: https://crates.io/crates/pricklybirdlib
//! [Crate]: https://img.shields.io/crates/v/pricklybirdlib

/// Contains pricklybird wordlist, reverse wordlist hashmap and CRC-8 lookup table.
pub mod constants;

use crate::constants::{BYTE_WORDLIST, CRC8_TABLE, HASH_TABLE, word_hash};
use std::fmt;

/// Version of the pricklybird specification that this implementation complies with.
pub const PRICKLYBIRD_VERSION: &str = "v1";

/// An error occured while trying to decode pricklybird words.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum DecodeError {
    /// General decoding error
    General(String),
    /// Invalid CRC
    CRCError,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::General(msg) => write!(f, "Unable to decode pricklybird words. {msg}"),
            Self::CRCError => write!(f, "Invalid CRC detected."),
        }
    }
}

impl fmt::Debug for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to Display implementation
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for DecodeError {}

/// Result used in decode functions that can fail.
type Result<T> = std::result::Result<T, DecodeError>;

/// Calculate the CRC-8 used by pricklybird based on a precomputed table.
///
/// # CRC parameters
/// - Output width of 8 bits
/// - Division using the polynomial `0x1D`
/// - An initial value of zero
/// - No input or output reflection
/// - No XOR operation on the output
/// - Remainder after division of data with correct CRC appended is zero
///
/// # Usage
/// ```
/// use pricklybirdlib::calculate_crc8;
/// let crc = calculate_crc8(b"123456789");
/// assert_eq!(0x37, crc);
/// ```
#[must_use]
#[inline]
pub fn calculate_crc8(data: &[u8]) -> u8 {
    let mut crc = 0_u8;

    for &byte in data {
        crc = CRC8_TABLE[(crc ^ byte) as usize];
    }
    crc
}

/// Convert bytearray to list of pricklybird words.
///
/// Return a list of words with each input byte mapped to the matching pricklybird word.
/// The words are encoded as a vec of four byte arrays containing ASCII compatible UTF-8.
///
/// # Usage
/// ```
/// use pricklybirdlib::bytes_to_words;
/// let data = [0x42_u8, 0x43];
/// let words = bytes_to_words(&data);
/// assert_eq!(vec![[102, 108, 101, 97], [102, 108, 117, 120]], words);
/// ```
#[must_use]
pub fn bytes_to_words(data: &[u8]) -> Vec<[u8; 4]> {
    let mut result_words = Vec::with_capacity(data.len());

    for &byte in data {
        result_words.push(BYTE_WORDLIST[byte as usize]);
    }
    result_words
}

/// Return a vector of bytes coresponding to the pricklybird words supplied as input.
///
/// # Errors
/// Will return `DecodeError::General` if:
/// - The input contains non ASCII compatible characters
/// - The words in the input are not all four characters long
/// - Words in the input dont appear in the wordlist
///
/// # Usage
/// ```
/// use pricklybirdlib::words_to_bytes;
/// let words = vec!["flea", "flux"];
/// let data = words_to_bytes(&words).unwrap();
/// assert_eq!(vec![0x42, 0x43], data);
/// ```
pub fn words_to_bytes(words: &Vec<&str>) -> Result<Vec<u8>> {
    let mut bytevector = Vec::<u8>::with_capacity(words.len());

    for &word in words {
        let word_lower = word.to_lowercase();
        let word_bytes = word_lower.as_bytes();
        if word_bytes.len() != 4 {
            return Err(DecodeError::General(
                "Input words must be four characters long.".into(),
            ));
        }
        let recovered_byte = HASH_TABLE[word_hash(word_bytes[0], word_bytes[3])];

        // Verify that the byte from the lookup operation matches the word.
        if word_bytes != BYTE_WORDLIST[recovered_byte as usize] {
            return Err(DecodeError::General(
                "Invalid word detected in input.".into(),
            ));
        }
        bytevector.push(recovered_byte);
    }
    Ok(bytevector)
}

/// Convert arbitrary data to a pricklybird string and attach CRC.
///
/// # Usage
/// ```
/// use pricklybirdlib::convert_to_pricklybird;
/// let data = [0x42_u8, 0x43];
/// let code = convert_to_pricklybird(&data);
/// assert_eq!("flea-flux-full", code);
/// ```
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn convert_to_pricklybird(data: &[u8]) -> String {
    if data.is_empty() {
        return String::new();
    }
    let crc = calculate_crc8(data);
    let mut data_with_crc = Vec::with_capacity(data.len() + 1);
    data_with_crc.extend_from_slice(data);
    data_with_crc.push(crc);

    // Unwrap is safe here since we know the wordlist and seperator are valid UTF-8.
    String::from_utf8(bytes_to_words(&data_with_crc).join(&b'-')).unwrap()
}

/// Convert a pricklybird string to bytes and check CRC.
///
/// # Errors
/// Will return `DecodeError::General` if:
/// - The input is less than two words long,
/// - The input contains non ASCII compatible characters
/// - The words in the input are not all four characters long
/// - Words in the input dont appear in the wordlist
///
/// Will return `DecodeError::CRCError` if the CRC value does not match the input.
///
/// # Usage
/// ```
/// use pricklybirdlib::convert_from_pricklybird;
/// let code = "flea-flux-full";
/// let data = convert_from_pricklybird(code).unwrap();
/// assert_eq!(vec![0x42, 0x43], data);
/// ```
pub fn convert_from_pricklybird(words: &str) -> Result<Vec<u8>> {
    let word_vec: Vec<&str> = words.trim().split('-').collect();

    if word_vec.len() < 2 {
        return Err(DecodeError::General(
            "Input must be at least two words long.".into(),
        ));
    }

    let mut data = words_to_bytes(&word_vec)?;
    if calculate_crc8(&data) != 0 {
        return Err(DecodeError::CRCError);
    }
    // Remove CRC
    let _ = data.pop();
    Ok(data)
}

/// Test the conversion from and to pricklybird.
#[cfg(test)]
mod pricklybird_tests {
    use super::*;
    /// Seed used to generate test data using the PRNG implemented in `generate_test_data`.
    const TEST_DATA_SEED: u128 = 1;
    /// How many byes of test data to use for conversion tests.
    const TEST_DATA_BYTES: usize = 4096;
    /// Pseudorandom data used to test conversion functions.
    const TEST_DATA: [u8; TEST_DATA_BYTES] = generate_test_data(TEST_DATA_SEED);

    /// Generates pseudorandom test data using the Lehmer64 LCG.
    #[allow(clippy::cast_possible_truncation)]
    const fn generate_test_data(seed: u128) -> [u8; TEST_DATA_BYTES] {
        const MULTIPLIER: u128 = 0xDA942042E4DD58B5;
        const WARMUP_ITERATIONS: usize = 128;
        let mut state = seed;
        // Mix up the state a little to compensate for potentialy small seed.
        let mut i: usize = 0;
        while i < WARMUP_ITERATIONS {
            state = state.wrapping_mul(MULTIPLIER);
            i += 1;
        }

        let mut j: usize = 0;
        let mut result = [0_u8; TEST_DATA_BYTES];
        while j < TEST_DATA_BYTES {
            state = state.wrapping_mul(MULTIPLIER);
            let random_val = (state >> 64) as u64;
            result[j] = random_val as u8;
            result[j + 1] = (random_val >> 8) as u8;
            result[j + 2] = (random_val >> 16) as u8;
            result[j + 3] = (random_val >> 24) as u8;
            result[j + 4] = (random_val >> 32) as u8;
            result[j + 5] = (random_val >> 40) as u8;
            result[j + 6] = (random_val >> 48) as u8;
            result[j + 7] = (random_val >> 56) as u8;
            j += 8;
        }
        result
    }

    /// Test the standard vectors supplied with the specification.
    #[test]
    fn test_vectors() {
        let test_vectors = vec![
            (vec![0xDE_u8, 0xAD, 0xBE, 0xEF], "turf-port-rust-warn-void"),
            (vec![0x42_u8, 0x43], "flea-flux-full"),
            (
                vec![0x12_u8, 0x34, 0x56, 0x78, 0x90],
                "blob-eggs-hair-king-meta-yell",
            ),
            (vec![0_u8; 5], "acid-acid-acid-acid-acid-acid"),
            (vec![0xFF_u8; 5], "zone-zone-zone-zone-zone-sand"),
        ];

        for (data, words) in test_vectors {
            // Test converting bytes to pricklybird.
            assert_eq!(
                words,
                convert_to_pricklybird(&data),
                "Failed to convert {:?} test vector to pricklybird.",
                data
            );

            // Test converting pricklybird to bytes.
            assert_eq!(
                data,
                convert_from_pricklybird(words).unwrap(),
                "Failed to convert {} test vector to bytes.",
                words
            );
        }
    }

    /// Test conversion to and from pricklybird on pseudorandom test data.
    #[test]
    fn test_simple_conversion() {
        let coded_words = convert_to_pricklybird(&TEST_DATA);
        let decoded_data = convert_from_pricklybird(&coded_words).unwrap();
        assert_eq!(
            TEST_DATA.to_vec(),
            decoded_data,
            "Converter did not correctly encode or decode data."
        );
    }

    /// Test that pricklybird input containing mixed case is properly decoded.
    #[test]
    fn test_uppercase() {
        assert_eq!(
            vec![0xDE_u8, 0xAD, 0xBE, 0xEF],
            convert_from_pricklybird("TUrF-Port-RUST-warn-vOid").unwrap(),
            "Converter did not correctly decode uppercase data."
        );
    }

    /// Test that replacing a pricklybird word is detected using the CRC-8.
    #[test]
    fn test_error_detection_bit_flip() {
        let coded_words = convert_to_pricklybird(&TEST_DATA);
        let mut corrupt_data = TEST_DATA;
        corrupt_data[0] ^= 1;
        let incorrect_word =
            String::from_utf8(bytes_to_words(&corrupt_data[0..1])[0].to_vec()).unwrap();
        let incorrect_coded_words = format!("{}{}", &incorrect_word[..4], &coded_words[4..]);
        assert!(
            convert_from_pricklybird(&incorrect_coded_words).is_err(),
            "Converter did not detect error in corrupted input."
        );
    }

    /// Check that swapping two adjacent words is detected using the CRC-8.
    #[test]
    fn test_error_detection_adjacent_swap() {
        let coded_words = convert_to_pricklybird(&TEST_DATA);
        let mut word_vec: Vec<&str> = coded_words.split('-').collect();
        word_vec.swap(0, 1);
        let swapped_coded_words = word_vec.join("-");
        assert!(
            matches!(
                convert_from_pricklybird(&swapped_coded_words),
                Err(DecodeError::CRCError)
            ),
            "Converter did not detect error caused by word swap."
        );
    }

    /// Check that whitespace is correctly trimmed.
    #[test]
    fn test_whitespace_trim() {
        assert_eq!(
            vec![0x42_u8, 0x43],
            convert_from_pricklybird(" \t\n\r\x0b\x0c flea-flux-full \t\n\r\x0b\x0c ").unwrap()
        );
    }

    /// Check that edge cases result in the correct errors.
    #[test]
    fn test_unusual_input() {
        let edge_cases = vec![
            ("", "empty input"),
            ("orca", "input to short"),
            ("a®¿a-orca", "invalid characters in input"),
            ("gäsp-risk-king-orca-husk", "invalid characters in input"),
            ("-risk-king-orca-husk", "incorrectly formatted input"),
            ("gasp-rock-king-orca-husk", "incorrect word in input"),
            ("flea- \t \t-full", "whitespace in input"),
            ("flea-aaa\0-full", "null bytes in input"),
            ("flea-\0aaa-full", "null bytes in input"),
            ("flea-\x7faaa-full", "ASCII control character in input"),
            ("flea-aaa\x7f-full", "ASCII control character in input"),
            // Check that no index out of bound error is thrown when the highest
            // possible value is used to index the hash table.
            ("zzzz-king", "incorrect word in input"),
        ];
        for (edge_case_input, error_reason) in edge_cases {
            assert!(
                convert_from_pricklybird(edge_case_input).is_err(),
                "Converter did not return error for: {} ({})",
                error_reason,
                edge_case_input
            );
        }
    }

    /// Check that empty input results in empty output.
    #[test]
    fn test_empty_input() {
        assert_eq!("", convert_to_pricklybird(&[]));
        assert!(bytes_to_words(&[]).is_empty());
        assert!(words_to_bytes(&Vec::<&str>::new()).unwrap().is_empty());
    }
}

/// Check functionality of the cyclic redundancy check.
#[cfg(test)]
mod crc8_tests {
    use super::*;

    /// Check that CRC-8 of empty input is zero.
    #[test]
    fn test_empty_input() {
        let test_data: &[u8] = &[];
        let result = calculate_crc8(test_data);
        assert_eq!(0, result, "CRC-8 of empty data should be 0.");
    }

    /// Check that CRC-8 of a byte is equal to the matching table value.
    #[test]
    fn test_table_lookup() {
        let test_data = &[0x42_u8];
        let result = calculate_crc8(test_data);
        let expected = CRC8_TABLE[test_data[0] as usize];
        assert_eq!(
            expected, result,
            "CRC-8 of single byte should match table value."
        );
    }

    /// Check that data with appended correct CRC-8 has a remainder of zero.
    #[test]
    fn test_with_appended_crc() {
        let mut test_data = b"Test data".to_vec();
        let crc_value = calculate_crc8(&test_data);
        test_data.push(crc_value);

        let result = calculate_crc8(&test_data);
        assert_eq!(
            0, result,
            "Data with appended correct CRC-8 should result in remainder 0."
        );
    }
}
