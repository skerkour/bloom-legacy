mod aggregate;
mod commands;

pub use aggregate::{PendingEmail, VerificationFailedReason};
pub use commands::{
    create::{Create, Created},
    delete::{Delete, Deleted},
    fail_verification::{FailVerification, VerificationFailed},
    verify::{Verified, Verify},
};
