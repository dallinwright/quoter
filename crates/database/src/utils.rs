use types::error::ErrorType::ParseError;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use base64::engine::general_purpose;
use base64::Engine;
use rand::Rng;
use types::error::Error;

/// Encrypts the provided UTF-8 plaintext using AES-256-GCM.
///
/// - Key format: The `secret_key` is treated as a raw byte sequence (via `as_bytes()`).
///   For AES-256, ensure it is exactly 32 bytes when encoded as bytes.
/// - Nonce: A random 96-bit (12-byte) nonce is generated per call.
/// - Output: Returns base64-encoded bytes of (nonce || ciphertext).
///
/// Parameters:
/// - `secret_key`: Secret used to derive the AEAD key (must be 32 bytes as bytes for AES-256).
/// - `plain_data`: UTF-8 string to encrypt.
///
/// Returns:
/// - `Ok(String)`: Base64 string containing nonce + ciphertext.
/// - `Err(Error)`: If encryption fails or randomness fails to initialize.
///
/// Security notes:
/// - Always use a strong, random 32-byte key. Do not reuse keys across environments.
/// - A fresh random nonce is generated for every encryption call.
/// - The output includes the nonce so it can be used for decryption.
///
/// Example:
/// ```rust
/// # use your_crate::encrypt_data;
/// let key = "0123456789abcdef0123456789abcdef"; // 32 bytes
/// let msg = r#"{"foo":"bar"}"#;
/// let enc = encrypt_data(key, msg).expect("encryption failed");
/// assert_ne!(enc, msg);
/// ```
pub fn encrypt_data(secret_key: &str, plain_data: &str) -> Result<String, Error> {
    let key_bytes = secret_key.as_bytes();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];

    let mut local_rng = rand::rng();
    local_rng.fill(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plain_data.as_bytes())
        .map_err(|e| Error::new(ParseError, &format!("encryption failed: {e:?}")))?;

    // combine nonce + ciphertext and encode
    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    let b64 = general_purpose::STANDARD.encode(combined);
    Ok(b64)
}


/// Decrypts data previously produced by `encrypt_data` using AES-256-GCM.
///
/// - Input: Base64 string of (nonce || ciphertext).
/// - Nonce: Extracted from the first 12 bytes of the decoded input.
///
/// Parameters:
/// - `secret_key`: Same 32-byte (as bytes) key used for encryption.
/// - `encrypted_data`: Base64 string produced by `encrypt_data`.
///
/// Returns:
/// - `Ok(String)`: The decrypted UTF-8 plaintext.
/// - `Err(Error)`: If base64 decoding fails, format is invalid, authentication fails,
///   or the plaintext is not valid UTF-8.
///
/// Security notes:
/// - Authentication failure indicates either a wrong key, corrupted data, or tampering.
/// - Never ignore decryption errors; they protect against forgery.
///
/// Example:
/// ```rust
/// # use your_crate::{encrypt_data, decrypt_data};
/// let key = "0123456789abcdef0123456789abcdef"; // 32 bytes
/// let msg = "hello";
/// let enc = encrypt_data(key, msg).unwrap();
/// let dec = decrypt_data(key, &enc).unwrap();
/// assert_eq!(dec, msg);
/// ```
pub fn decrypt_data(secret_key: &str, encrypted_data: &str) -> Result<String, Error> {
    let key_bytes = secret_key.as_bytes();

    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    let decoded = general_purpose::STANDARD.decode(encrypted_data)
        .map_err(|e| Error::new(ParseError, &format!("decryption failed: {e:?}")))?;

    if decoded.len() < 12 {
        return Err(Error::new(ParseError, "decryption failed: invalid data length"));
    }

    let (nonce_bytes, ciphertext) = decoded.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| Error::new(ParseError, &format!("decryption failed: {e:?}")))?;

    String::from_utf8(decrypted)
        .map_err(|e| Error::new(ParseError, &format!("decryption failed: {e:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_data() {
        let json = r#"{"foo":"bar"}"#;

        let test_secret_key = "4S#u135ayVtQ!naeYzk@NF#GaK&9by&6"; // 32 bytes

        let encrypted = encrypt_data(test_secret_key, json).expect("encryption failed");
        println!("Encrypted: {encrypted}");

        let decrypted = decrypt_data(test_secret_key, &encrypted).expect("decryption failed");
        assert_eq!(decrypted, json);
    }
}
