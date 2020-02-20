package consts

// pub const PASSWORD_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(13)
// pub const PASSWORD_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(13)

// pub const PENDING_USER_CODE_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(11)
// pub const PENDING_USER_CODE_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(11)

// pub const PENDING_EMAIL_CODE_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(11)
// pub const PENDING_EMAIL_CODE_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(11)

// pub const SESSION_TOKEN_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(5)
// pub const SESSION_TOKEN_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(5)

// pub const PASSWORD_RESET_TOKEN_ARGON2_OPSLIMIT: argon2id::OpsLimit = argon2id::OPSLIMIT_INTERACTIVE; // TODO: change bcrypt(5)
// pub const PASSWORD_RESET_TOKEN_ARGON2_MEMLIMIT: argon2id::MemLimit = argon2id::MEMLIMIT_INTERACTIVE; // TODO: change bcrypt(5)

const (
	PENDING_ACCOUNT_CODE_SALT_LENGTH = 32
)

const PASSWORD_BCRYPT_COST int = 13
const PENDING_USER_TOKEN_BCRYPT_COST int = 11
const PENDING_EMAIL_TOKEN_BCRYPT_COST int = 11
const SESSION_TOKEN_BCRYPT_COST int = 5
const PASSWORD_RESET_TOKEN_BCRYPT_COST int = 5

const SESSION_TOKEN_BYTES int = 256
const AVATAR_BYTES_MAX int = 3000000
const AVATAR_RESIZE int = 256
const PASSWORD_RESET_TOKEN_BYTES int = 256
const AVATAR_DEFAULT_PATH = "/imgs/myaccount/profile.jpg"

const BIO_MAX_LENGTH int = 200
const DISPLAY_NAME_MAX_LENGTH int = 42
const FIRST_NAME_MAX_LENGTH int = 30
const LAST_NAME_MAX_LENGTH int = 42

const USERNAME_MIN_LENGTH int = 5
const USERNAME_MAX_LENGTH int = 16

var INVALID_USERNAMES = []string{
	"admin",
	"sysy",
	"administrator",
	"bloom",
	"bloom42",
	"support",
	"help",
	"settings",
	"security",
	"profile",
	"42bloom",
	"sylvain.kerkour",
	"sylvainkerkour",
	"kerkour.sylvain",
	"kerkoursylvain",
	"hello",
}

const PASSWORD_MIN_LENGTH int = 8
const PASSWORD_MAX_LENGTH int = 256
