use failure::{Fail};
use serde::{Serialize,Deserialize};

/// Common Error for UDP Command Handling
#[derive(Serialize, Deserialize, Debug, Fail, Clone, PartialEq)]
// pub enum Error<E: Fail + Clone + PartialEq> {
pub enum Error {
    /// Wrong Number of Arguments
    #[fail(display = "Wrong Number of Arguments")]
    WrongNoArgs,
    /// Wrong CommandID
    #[fail(display = "No Command with matching CommandID")]
    NoCmd,
    /// Service specific Error
    #[fail(display = "Service Error")]
    ServiceError(u8),
    // Other Error,
    #[fail(display = "Other Error")]
    Other,
}

pub type Result<T> = core::result::Result<T,Error>;
