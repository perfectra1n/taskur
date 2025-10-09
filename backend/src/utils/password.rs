use crate::errors::AppResult;

pub fn hash_password(password: &str) -> AppResult<String> {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    let valid = bcrypt::verify(password, hash)?;
    Ok(valid)
}
