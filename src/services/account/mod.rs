pub mod api;
pub mod domain;
pub mod notifications;
pub mod validators;
pub mod controllers;

pub const PASSWORD_BCRYPT_COST: u32 = 13;
pub const PENDING_ACCOUNT_TOKEN_BCRYPT_COST: u32 = 11;
pub const SESSION_TOKEN_BCRYPT_COST: u32 = 5;
pub const SESSION_TOKEN_MIN_LENGTH: u32 = 256;
pub const SESSION_TOKEN_MAX_LENGTH: u32 = 356;
pub const AVATAR_MAX_SIZE: usize = 3_145_728; // 3MB
