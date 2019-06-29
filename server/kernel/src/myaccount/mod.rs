pub mod api;
pub mod controllers;
pub mod domain;
pub mod notifications;
pub mod validators;

pub const PASSWORD_BCRYPT_COST: u32 = 13;
pub const PENDING_USER_TOKEN_BCRYPT_COST: u32 = 11;
pub const PENDING_EMAIL_TOKEN_BCRYPT_COST: u32 = 11;
pub const SESSION_TOKEN_BCRYPT_COST: u32 = 5;
pub const SESSION_TOKEN_MIN_LENGTH: u32 = 100;
pub const SESSION_TOKEN_MAX_LENGTH: u32 = 150;
pub const AVATAR_MAX_SIZE: usize = 3_145_728; // 3MB
pub const AVATAR_RESIZE: usize = 256;
pub const PASSWORD_RESET_TOKEN_MIN_LENGTH: u32 = 200;
pub const PASSWORD_RESET_TOKEN_MAX_LENGTH: u32 = 256;
pub const PASSWORD_RESET_TOKEN_BCRYPT_COST: u32 = 5;
pub const AVATAR_DEFAULT_PATH: &str = "/kernel/static/imgs/profile.jpg";
pub const BIO_MAX_LENGTH: usize = 256;
pub const DISPLAY_NAME_MAX_LENGTH: usize = 42;
