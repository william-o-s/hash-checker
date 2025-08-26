//! Hashing utilities

use log::debug;
use sha2::Sha256;
use sha2::Digest as _;
use std::fs::File;
use std::io;
use std::io::BufReader;

/// Computes the SHA-256 hash of the contents of a file at the given path and returns the result as a Base64-encoded string.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file to be hashed.
///
/// # Returns
///
/// Returns a `Result` containing the Base64-encoded SHA-256 hash as a `String` on success,
/// or a boxed error (`Box<dyn std::error::Error>`) if an error occurs while reading the file or computing the hash.
///
/// # Errors
///
/// This function will return an error if the file cannot be opened, read, or if any I/O error occurs during hashing.
///
/// # Examples
///
/// ```rust
/// use hash_checker::hash_sha256;
/// let result = hash_sha256("examples/valid.txt");
/// if let Ok(hash) = result {
///     println!("SHA-256 hash: {}", hash);
/// }
/// ```
pub fn hash_sha256(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    let n = io::copy(&mut reader, &mut hasher)?;
    debug!("Read {n} bytes from {path}");

    let hash = hasher.finalize();
    let base64 = base16ct::lower::encode_string(&hash);
    debug!("Computed SHA-256 hash for {path}: {base64}");

    Ok(base64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file() {
        let hash = hash_sha256("examples/valid.txt")
            .expect("Expected valid.txt to hash successfully.");
        assert_eq!(hash, "6d78392a5886177fe5b86e585a0b695a2bcd01a05504b3c4e38bc8eeb21e8326");
    }

    #[test]
    fn test_invalid_file() {
        let result = hash_sha256("examples/invalid.txt");
        assert!(result.is_err());
    }
}