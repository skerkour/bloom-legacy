#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ErrorCode {
    INTERNAL,
    UKNOWN_MESSAGE_TYPE,
    UNAUTHORIZED,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
