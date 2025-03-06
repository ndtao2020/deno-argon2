use argon2::{
  password_hash::{PasswordHasher, PasswordVerifier, SaltString},
  Argon2,
};
use getrandom::getrandom;
use wasm_bindgen::prelude::*;

// ================================================= [Argon2] =================================================
// https://github.com/napi-rs/node-rs/blob/main/packages/argon2/src/lib.rs
pub fn to_argon2(
  memory_cost: Option<u32>,
  time_cost: Option<u32>,
  output_len: Option<u32>,
  parallelism: Option<u32>,
) -> Argon2<'static> {
  let algorithm = argon2::Algorithm::Argon2id;
  let version = argon2::Version::V0x13;
  let params = argon2::Params::new(
    memory_cost.unwrap_or(argon2::Params::DEFAULT_M_COST),
    time_cost.unwrap_or(argon2::Params::DEFAULT_T_COST),
    parallelism.unwrap_or(argon2::Params::DEFAULT_P_COST),
    output_len.map(|o| o as usize),
  )
  .unwrap();
  // let sa: SaltString = SaltString::generate(&mut OsRng);
  Argon2::new(algorithm, version, params)
}

#[wasm_bindgen]
pub fn hash_password(
  password: &str,
  memory_cost: Option<u32>,
  time_cost: Option<u32>,
  output_len: Option<u32>,
  parallelism: Option<u32>,
) -> String {
  // Then invoke the fill function on a byte buffer to fill it with random data:
  let mut salt_bytes = [0u8; 16]; // 16 bytes for salt
  getrandom(&mut salt_bytes).expect("Failed to generate random salt");
  let salt =
    SaltString::encode_b64(&salt_bytes).expect("Failed to encode salt");

  // let sa: SaltString = SaltString::generate(&mut OsRng);
  to_argon2(memory_cost, time_cost, output_len, parallelism)
    .hash_password(password.as_bytes(), &salt)
    .unwrap()
    .to_string()
}

#[wasm_bindgen]
pub fn verify_password(
  password: &str,
  hash: &str,
  memory_cost: Option<u32>,
  time_cost: Option<u32>,
  output_len: Option<u32>,
  parallelism: Option<u32>,
) -> bool {
  let parsed_hash = argon2::PasswordHash::new(hash).unwrap();
  to_argon2(memory_cost, time_cost, output_len, parallelism)
    .verify_password(password.as_bytes(), &parsed_hash)
    .is_ok()
}
