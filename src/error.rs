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
    #[fail(display = "Unauthorized: {}", 0)]
    Unauthorized(String),
    #[fail(display = "Io: {}", 0)]
    Io(String),
    #[fail(display = "Io: {}", 0)]
    Image(String),
}


impl std::convert::From<actix::MailboxError> for KernelError {
    fn from(_e: actix::MailboxError) -> Self {
        KernelError::ActixMailbox
    }
}

impl std::convert::From<diesel::result::Error> for KernelError {
    fn from(err: diesel::result::Error) -> Self {
        KernelError::Diesel(format!("{:?}", err))
    }
}

impl std::convert::From<image::ImageError> for KernelError {
    fn from(err: image::ImageError) -> Self {
        KernelError::Image(format!("{:?}", err))
    }
}


impl std::convert::From<diesel::r2d2::Error> for KernelError {
    fn from(_err: diesel::r2d2::Error) -> Self {
        KernelError::R2d2
    }
}

impl std::convert::From<tokio_timer::Error> for KernelError {
    fn from(_err: tokio_timer::Error) -> Self {
        KernelError::Tokio
    }
}

impl std::convert::From<bcrypt::BcryptError> for KernelError {
    fn from(_err: bcrypt::BcryptError) -> Self {
        KernelError::Bcrypt
    }
}
