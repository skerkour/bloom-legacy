use failure::Fail;

#[derive(Clone, Debug, Fail)]
pub enum KernelError {
    #[fail(display="ActixMailboxError")]
    ActixMailbox,
    #[fail(display="DieselError: {}", 0)]
    Diesel(String),
    #[fail(display="R2d2Error")]
    R2d2,
    #[fail(display="TokioError")]
    Tokio,
    #[fail(display="BcryptError")]
    Bcrypt,
    #[fail(display = "validation error: {}", 0)]
    Validation(String),
}


impl std::convert::From<actix::MailboxError> for KernelError {
    fn from(e: actix::MailboxError) -> Self {
        KernelError::ActixMailbox
    }
}

impl std::convert::From<diesel::result::Error> for KernelError {
    fn from(e: diesel::result::Error) -> Self {
        KernelError::Diesel(format!("{:?}", e))
    }
}


impl std::convert::From<diesel::r2d2::Error> for KernelError {
    fn from(e: diesel::r2d2::Error) -> Self {
        KernelError::R2d2
    }
}

impl std::convert::From<tokio_timer::Error> for KernelError {
    fn from(e: tokio_timer::Error) -> Self {
        KernelError::Tokio
    }
}
