use failure::{Fail};
use serde::Serialize;

/// Common Error for UDP Command Handling
#[derive(Serialize, Debug, Fail, Clone, PartialEq)]
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
    /// Other Error,
    #[fail(display = "Other Error")]
    Other,
    /// Failure Error
    #[fail(display = "Failure Error")]
    Failure(String),
    /// IO Error
    #[fail(display = "IO Error")]
    Io(u8),
}
impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Error {
        Error::Failure(e.name().unwrap().to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        match e.kind() {
            std::io::ErrorKind::NotFound => Error::Io(0),
            std::io::ErrorKind::PermissionDenied => Error::Io(1),
            std::io::ErrorKind::ConnectionRefused => Error::Io(2),
            std::io::ErrorKind::ConnectionReset => Error::Io(3),
            // std::io::ErrorKind::HostUnreachable => Error::Io(4),
            // std::io::ErrorKind::NetworkUnreachable => Error::Io(5),
            std::io::ErrorKind::ConnectionAborted => Error::Io(6),
            std::io::ErrorKind::NotConnected => Error::Io(7),
            std::io::ErrorKind::AddrInUse => Error::Io(8),
            std::io::ErrorKind::AddrNotAvailable => Error::Io(9),
            // std::io::ErrorKind::NetworkDown => Error::Io(10),
            std::io::ErrorKind::BrokenPipe => Error::Io(11),
            std::io::ErrorKind::AlreadyExists => Error::Io(12),
            std::io::ErrorKind::WouldBlock => Error::Io(13),
            // std::io::ErrorKind::NotADirectory => Error::Io(14),
            // std::io::ErrorKind::IsADirectory => Error::Io(15),
            // std::io::ErrorKind::DirectoryNotEmpty => Error::Io(16),
            // std::io::ErrorKind::ReadOnlyFilesystem => Error::Io(17),
            // std::io::ErrorKind::FilesystemLoop => Error::Io(18),
            // std::io::ErrorKind::StaleNetworkFileHandle => Error::Io(19),
            std::io::ErrorKind::InvalidInput => Error::Io(20),
            std::io::ErrorKind::InvalidData => Error::Io(21),
            std::io::ErrorKind::TimedOut => Error::Io(22),
            std::io::ErrorKind::WriteZero => Error::Io(23),
            // std::io::ErrorKind::StorageFull => Error::Io(24),
            // std::io::ErrorKind::NotSeekable => Error::Io(25),
            // std::io::ErrorKind::FilesystemQuotaExceeded => Error::Io(26),
            // std::io::ErrorKind::FileTooLarge => Error::Io(27),
            // std::io::ErrorKind::ResourceBusy => Error::Io(28),
            // std::io::ErrorKind::ExecutableFileBusy => Error::Io(29),
            // std::io::ErrorKind::Deadlock => Error::Io(30),
            // std::io::ErrorKind::CrossesDevices => Error::Io(31),
            // std::io::ErrorKind::TooManyLinks => Error::Io(32),
            // std::io::ErrorKind::FilenameTooLong => Error::Io(33),
            // std::io::ErrorKind::ArgumentListTooLong => Error::Io(34),
            std::io::ErrorKind::Interrupted => Error::Io(35),
            std::io::ErrorKind::Unsupported => Error::Io(36),
            std::io::ErrorKind::UnexpectedEof => Error::Io(37),
            std::io::ErrorKind::OutOfMemory => Error::Io(38),
            std::io::ErrorKind::Other => Error::Io(39),
            _ => Error::Io(40),
        }
    }
}
pub type Result<T> = core::result::Result<T,Error>;