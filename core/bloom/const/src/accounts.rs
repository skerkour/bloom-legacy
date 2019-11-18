use crypto42::kdf::argon2id;

pub const PASSWORD_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(13)
pub const PASSWORD_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(13)

pub const PENDING_USER_CODE_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(11)
pub const PENDING_USER_CODE_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(11)

pub const PENDING_EMAIL_CODE_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(11)
pub const PENDING_EMAIL_CODE_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(11)

pub const SESSION_TOKEN_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(5)
pub const SESSION_TOKEN_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(5)

pub const PASSWORD_RESET_TOKEN_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(5)
pub const PASSWORD_RESET_TOKEN_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(5)

pub const PASSWORD_BCRYPT_COST: u32 = 13;
pub const PENDING_USER_TOKEN_BCRYPT_COST: u32 = 11;
pub const PENDING_EMAIL_TOKEN_BCRYPT_COST: u32 = 11;
pub const SESSION_TOKEN_BCRYPT_COST: u32 = 5;
pub const PASSWORD_RESET_TOKEN_BCRYPT_COST: u32 = 5;

pub const SESSION_TOKEN_BYTES: u32 = 256;
pub const AVATAR_BYTES_MAX: usize = 3_000_000;
pub const AVATAR_RESIZE: usize = 256;
pub const PASSWORD_RESET_TOKEN_BYTES: u32 = 256;
pub const AVATAR_DEFAULT_PATH: &str = "/imgs/myaccount/profile.jpg";
pub const BIO_MAX_LENGTH: usize = 200;
pub const DISPLAY_NAME_MAX_LENGTH: usize = 42;
