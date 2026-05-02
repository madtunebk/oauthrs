use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{rand_core::OsRng, SaltString},
};

pub fn hash(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn verify(password: &str, hash: &str) -> bool {
    let parsed = PasswordHash::new(hash).expect("Invalid password hash");
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}
