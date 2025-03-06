# Rust Argon2

This project demonstrates how to use the Argon2 hashing algorithm in a Rust application. Argon2 is a password-hashing function that provides resistance against side-channel attacks and high memory requirements to thwart brute-force attacks.

## Installation

To use Argon2 in your project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
argon2 = { version = "0.5", default-features = false, features = ['std'] }
