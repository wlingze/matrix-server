pub mod error;

use rand::thread_rng;
use rand::Rng;

/// Parses the bytes into a string.
pub fn string_from_bytes(bytes: Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(bytes)
}

pub fn random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
