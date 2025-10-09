//! Password hashing and verification utilities.
//!
//! Provides secure password hashing using bcrypt with default cost factor.

use crate::errors::AppResult;

/// Hash a plaintext password using bcrypt.
///
/// Uses bcrypt's default cost factor (currently 12) for security.
///
/// # Arguments
///
/// * `password` - Plaintext password to hash
///
/// # Returns
///
/// A bcrypt hash string that can be safely stored in the database
///
/// # Errors
///
/// Returns an error if the hashing operation fails
///
/// # Example
///
/// ```no_run
/// # use taskur_backend::utils::hash_password;
/// let password = "my_secure_password";
/// let hash = hash_password(password).unwrap();
/// // Store hash in database
/// ```
pub fn hash_password(password: &str) -> AppResult<String> {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hash)
}

/// Verify a plaintext password against a bcrypt hash.
///
/// Compares the plaintext password with the stored hash in constant time
/// to prevent timing attacks.
///
/// # Arguments
///
/// * `password` - Plaintext password to verify
/// * `hash` - Bcrypt hash to compare against
///
/// # Returns
///
/// `true` if the password matches the hash, `false` otherwise
///
/// # Errors
///
/// Returns an error if the verification operation fails
///
/// # Example
///
/// ```no_run
/// # use taskur_backend::utils::verify_password;
/// let password = "my_secure_password";
/// let hash = "$2b$12$..."; // Hash from database
/// let is_valid = verify_password(password, hash).unwrap();
/// if is_valid {
///     // Password is correct
/// }
/// ```
pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    let valid = bcrypt::verify(password, hash)?;
    Ok(valid)
}
