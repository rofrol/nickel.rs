use std::str::CowString;
use http::status::Status;

pub use self::NickelErrorKind::{ErrorWithStatusCode, UserDefinedError, Other};

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.

#[deriving(Show)]
pub struct NickelError {
    pub kind: NickelErrorKind,
    pub message: CowString<'static>
}

impl NickelError {
    /// Creates a new `NickelError` instance
    ///
    /// # Example
    /// ```{rust,ignore}
    /// NickelError::new("Error Parsing JSON", ErrorWithStatusCode(BadRequest));
    /// ```
    pub fn new<T: IntoCow<'static, String, str>>(message: T, kind: NickelErrorKind) -> NickelError {
        NickelError {
            message: message.into_cow(),
            kind: kind
        }
    }
}

#[deriving(Show)]
pub enum NickelErrorKind {
    // FIXME: Should probably re-export http::status::Status
    ErrorWithStatusCode(Status),
    UserDefinedError(int, String),
    Other
}
