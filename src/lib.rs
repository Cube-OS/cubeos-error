//
// Copyright (C) 2022 CUAVA
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 
// Contributed by: Patrick Oppel (patrick.oppel94@gmail.com)
//
// Code in parts generated with GPT-3 (see disclaimer in README)

use failure::{Fail};
use serde::{Serialize,Deserialize};
use std::convert::Infallible;
use std::sync::{PoisonError,MutexGuard,RwLockReadGuard};
use nix::errno::Errno;
use syslog::Error as SyslogError;

/// Common Error for UDP Command Handling
#[derive(Serialize, Deserialize, Debug, Fail, Clone, PartialEq)]
// pub enum Error<E: Fail + Clone + PartialEq> {
pub enum Error {
    /// None
    #[fail(display = "None")]
    None,
    /// Wrong Number of Arguments
    #[fail(display = "Wrong Number of Arguments")]
    WrongNoArgs,
    /// Wrong CommandID
    #[fail(display = "No Command with matching CommandID")]
    NoCmd,
    /// Service specific Error
    #[fail(display = "Service Error")]
    ServiceError(u8),
    /// Service specific Error multiple fields
    #[fail(display = "Service Error")]
    ServiceErrorX(u8,f64),
    /// Other Error,
    #[fail(display = "Other Error")]
    Other,
    /// Failure Error
    #[fail(display = "Failure Error")]
    Failure(String),
    /// IO Error
    #[fail(display = "IO Error")]
    Io(u8),
    /// Infallible
    #[fail(display = "Infallible")]
    Infallible,
    /// Bincode Error
    #[fail(display = "bincode Error")]
    Bincode(u8),
    /// Poisoned Mutex Error
    #[fail(display = "Poisened Mutex")]
    PoisonedMutex,
    /// PoisonError RwLockReadGuard
    #[fail(display = "Poisened RwLockReadGuard")]
    PoisonedRwLock,
    /// UART
    #[fail(display = "UART Error")]
    Uart(u8),
    /// Nix
    #[fail(display = "Nix Error")]
    NixError(u8),
    /// Syslog
    #[fail(display = "Syslog Error")]
    Syslog(u8),
    /// Diesel
    #[fail(display = "Diesel Error")]
    Diesel(u8),
}
impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Error {
        Error::Failure(e.name().unwrap().to_string())
    }
}
impl From<std::io::ErrorKind> for Error {
    fn from(e: std::io::ErrorKind) -> Error {
        match e {
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
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::from(e.kind())
    }
}
impl From<Infallible> for Error {
    fn from(_i: Infallible) -> Error {
        Error::Infallible
    }
}
impl From<bincode::Error> for Error {
    fn from(b: bincode::Error) -> Error {
        match *b {
            bincode::ErrorKind::Io(_) => Error::Bincode(0),
            bincode::ErrorKind::InvalidUtf8Encoding(_) => Error::Bincode(1),
            bincode::ErrorKind::InvalidBoolEncoding(_) => Error::Bincode(2),
            bincode::ErrorKind::InvalidCharEncoding => Error::Bincode(3),
            bincode::ErrorKind::InvalidTagEncoding(_) => Error::Bincode(4),
            bincode::ErrorKind::DeserializeAnyNotSupported => Error::Bincode(5),
            bincode::ErrorKind::SizeLimit => Error::Bincode(6),
            bincode::ErrorKind::SequenceMustHaveLength => Error::Bincode(7),
            bincode::ErrorKind::Custom(_) => Error::Bincode(8),            
        }
    }
}
impl From<PoisonError<MutexGuard<'_,()>>> for Error {
    fn from(_e: PoisonError<MutexGuard<'_,()>>) -> Error {
        Error::PoisonedMutex
    }
}
impl From<PoisonError<MutexGuard<'_,cubeos_telemetry_db::Database>>> for Error {
    fn from(_e: PoisonError<MutexGuard<'_,cubeos_telemetry_db::Database>>) -> Error {
        Error::PoisonedMutex
    }
}
impl From<PoisonError<RwLockReadGuard<'_,()>>> for Error {
    fn from(_e: PoisonError<RwLockReadGuard<'_,()>>) -> Error {
        Error::PoisonedRwLock
    }
}
impl From<rust_uart::UartError> for Error {
    fn from(e: rust_uart::UartError) -> Error {
        match e {
            rust_uart::UartError::GenericError => Error::Uart(0),
            rust_uart::UartError::PortBusy => Error::Uart(1),
            rust_uart::UartError::SerialError(s) => {
                match s {
                    serial::ErrorKind::NoDevice => Error::Uart(2),
                    serial::ErrorKind::InvalidInput => Error::Uart(3),
                    // serial::ErrorKind::Unknown => Error::Uart(4),
                    serial::ErrorKind::Io(io) => Error::from(io),
                }
            }
        }
    }
}

impl From<Errno> for Error {
    fn from(e: Errno) -> Error {
        match e {            
            Errno::EPERM       => Error::NixError(1),
            Errno::ENOENT      => Error::NixError(2),
            Errno::ESRCH       => Error::NixError(3),
            Errno::EINTR       => Error::NixError(4),
            Errno::EIO         => Error::NixError(5),
            Errno::ENXIO       => Error::NixError(6),
            Errno::E2BIG       => Error::NixError(7),
            Errno::ENOEXEC     => Error::NixError(8),
            Errno::EBADF       => Error::NixError(9),
            Errno::ECHILD      => Error::NixError(10),
            Errno::EAGAIN      => Error::NixError(11),
            Errno::ENOMEM      => Error::NixError(12),
            Errno::EACCES      => Error::NixError(13),
            Errno::EFAULT      => Error::NixError(14),
            Errno::ENOTBLK     => Error::NixError(15),
            Errno::EBUSY       => Error::NixError(16),
            Errno::EEXIST      => Error::NixError(17),
            Errno::EXDEV       => Error::NixError(18),
            Errno::ENODEV      => Error::NixError(19),
            Errno::ENOTDIR     => Error::NixError(20),
            Errno::EISDIR      => Error::NixError(21),
            Errno::EINVAL      => Error::NixError(22),
            Errno::ENFILE      => Error::NixError(23),
            Errno::EMFILE      => Error::NixError(24),
            Errno::ENOTTY      => Error::NixError(25),
            Errno::ETXTBSY     => Error::NixError(26),
            Errno::EFBIG       => Error::NixError(27),
            Errno::ENOSPC      => Error::NixError(28),
            Errno::ESPIPE      => Error::NixError(29),
            Errno::EROFS       => Error::NixError(30),
            Errno::EMLINK      => Error::NixError(31),
            Errno::EPIPE       => Error::NixError(32),
            Errno::EDOM        => Error::NixError(33),
            Errno::ERANGE      => Error::NixError(34),
            Errno::EDEADLK     => Error::NixError(35),
            Errno::ENAMETOOLONG => Error::NixError(36),
            Errno::ENOLCK      => Error::NixError(37),
            Errno::ENOSYS      => Error::NixError(38),
            Errno::ENOTEMPTY   => Error::NixError(39),
            Errno::ELOOP       => Error::NixError(40),
            Errno::ENOMSG      => Error::NixError(41),
            Errno::EIDRM       => Error::NixError(42),
            Errno::ECHRNG      => Error::NixError(43),
            Errno::EL2NSYNC    => Error::NixError(44),
            Errno::EL3HLT      => Error::NixError(45),
            Errno::EL3RST      => Error::NixError(46),
            Errno::ELNRNG      => Error::NixError(47),
            Errno::EUNATCH     => Error::NixError(48),
            Errno::ENOCSI      => Error::NixError(49),
            Errno::EL2HLT      => Error::NixError(50),
            Errno::EBADE       => Error::NixError(51),
            Errno::EBADR       => Error::NixError(52),
            Errno::EXFULL      => Error::NixError(53),
            Errno::ENOANO      => Error::NixError(54),
            Errno::EBADRQC     => Error::NixError(55),
            Errno::EBADSLT     => Error::NixError(56),
            Errno::EBFONT      => Error::NixError(57),
            Errno::ENOSTR      => Error::NixError(58),
            Errno::ENODATA     => Error::NixError(59),
            Errno::ETIME       => Error::NixError(60),
            Errno::ENOSR       => Error::NixError(61),
            Errno::ENONET      => Error::NixError(62),
            Errno::ENOPKG      => Error::NixError(63),
            Errno::EREMOTE     => Error::NixError(64),
            Errno::ENOLINK     => Error::NixError(65),
            Errno::EADV        => Error::NixError(66),
            Errno::ESRMNT      => Error::NixError(67),
            Errno::ECOMM       => Error::NixError(68),
            Errno::EPROTO      => Error::NixError(69),
            Errno::EMULTIHOP   => Error::NixError(70),
            Errno::EDOTDOT     => Error::NixError(71),
            Errno::EBADMSG     => Error::NixError(72),
            Errno::EOVERFLOW   => Error::NixError(73),
            Errno::ENOTUNIQ    => Error::NixError(74),
            Errno::EBADFD      => Error::NixError(75),
            Errno::EREMCHG     => Error::NixError(76),
            Errno::ELIBACC     => Error::NixError(77),
            Errno::ELIBBAD     => Error::NixError(78),
            Errno::ELIBSCN     => Error::NixError(79),
            Errno::ELIBMAX     => Error::NixError(80),
            Errno::ELIBEXEC    => Error::NixError(81),
            Errno::EILSEQ      => Error::NixError(82),
            Errno::ERESTART    => Error::NixError(83),
            Errno::ESTRPIPE    => Error::NixError(84),
            Errno::EUSERS      => Error::NixError(85),
            Errno::ENOTSOCK    => Error::NixError(86),
            Errno::EDESTADDRREQ => Error::NixError(87),
            Errno::EMSGSIZE    => Error::NixError(88),
            Errno::EPROTOTYPE  => Error::NixError(89),
            Errno::ENOPROTOOPT => Error::NixError(90),
            Errno::EPROTONOSUPPORT => Error::NixError(91),
            Errno::ESOCKTNOSUPPORT => Error::NixError(92),
            Errno::EOPNOTSUPP  => Error::NixError(93),
            Errno::EPFNOSUPPORT => Error::NixError(94),
            Errno::EAFNOSUPPORT => Error::NixError(95),
            Errno::EADDRINUSE  => Error::NixError(96),
            Errno::EADDRNOTAVAIL => Error::NixError(97),
            Errno::ENETDOWN    => Error::NixError(98),
            Errno::ENETUNREACH => Error::NixError(99),
            Errno::ENETRESET   => Error::NixError(100),
            Errno::ECONNABORTED => Error::NixError(101),
            Errno::ECONNRESET  => Error::NixError(102),
            Errno::ENOBUFS     => Error::NixError(103),
            Errno::EISCONN     => Error::NixError(104),
            Errno::ENOTCONN    => Error::NixError(105),
            Errno::ESHUTDOWN   => Error::NixError(106),
            Errno::ETOOMANYREFS => Error::NixError(107),
            Errno::ETIMEDOUT   => Error::NixError(108),
            Errno::ECONNREFUSED => Error::NixError(109),
            Errno::EHOSTDOWN   => Error::NixError(110),
            Errno::EHOSTUNREACH => Error::NixError(111),
            Errno::EALREADY    => Error::NixError(112),
            Errno::EINPROGRESS => Error::NixError(113),
            Errno::ESTALE      => Error::NixError(114),
            Errno::EUCLEAN     => Error::NixError(115),
            Errno::ENOTNAM     => Error::NixError(116),
            Errno::ENAVAIL     => Error::NixError(117),
            Errno::EISNAM      => Error::NixError(118),
            Errno::EREMOTEIO   => Error::NixError(119),
            Errno::EDQUOT      => Error::NixError(120),
            Errno::ENOMEDIUM   => Error::NixError(121),
            Errno::EMEDIUMTYPE => Error::NixError(122),
            Errno::ECANCELED   => Error::NixError(123),
            Errno::ENOKEY      => Error::NixError(124),
            Errno::EKEYEXPIRED => Error::NixError(125),
            Errno::EKEYREVOKED => Error::NixError(126),
            Errno::EKEYREJECTED => Error::NixError(127),
            Errno::EOWNERDEAD  => Error::NixError(128),
            Errno::ENOTRECOVERABLE => Error::NixError(129),
            Errno::ERFKILL     => Error::NixError(130),
            Errno::EHWPOISON   => Error::NixError(131),
            Errno::UnknownErrno => Error::NixError(0),
            _ => Error::NixError(0),
        }
    }
}

impl From<SyslogError> for Error {
    fn from(e: SyslogError) -> Error {
        match e.kind() {
            syslog::ErrorKind::Io(i) => Error::from(i.kind()),
            syslog::ErrorKind::Msg(_) => Error::Syslog(0),
            syslog::ErrorKind::Initialization => Error::Syslog(1),
            syslog::ErrorKind::UnsupportedPlatform => Error::Syslog(2),
            syslog::ErrorKind::Format => Error::Syslog(3),
            syslog::ErrorKind::Write => Error::Syslog(4),
            &_ => Error::Syslog(5),
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Error {
        match e {
            diesel::result::Error::InvalidCString(_) => Error::Diesel(0),
            diesel::result::Error::DatabaseError(_,_) => Error::Diesel(1),
            diesel::result::Error::NotFound => Error::Diesel(2),
            diesel::result::Error::QueryBuilderError(_) => Error::Diesel(3),
            diesel::result::Error::DeserializationError(_) => Error::Diesel(4),
            diesel::result::Error::SerializationError(_) => Error::Diesel(5),
            diesel::result::Error::RollbackTransaction => Error::Diesel(6),
            diesel::result::Error::AlreadyInTransaction => Error::Diesel(7),
            _ => Error::Diesel(8),
        }
    }
}

pub type Result<T> = core::result::Result<T,Error>;
