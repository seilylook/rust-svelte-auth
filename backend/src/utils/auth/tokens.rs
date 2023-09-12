use argon2::password_hash::rand_core::{OsRng, RngCore};
use core::convert::TryFrom;
use deadpool_redis::redis::AsyncCommands;
use hex;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::{local, version::V4, Local};

// Store the session key prefix as a const 
// So it can not be typo'd anywhere it's used
const SESSION_KEY_PREFIX: &str = "valid_session_key_for_{}";

