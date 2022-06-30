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

use failure::{Fail};
use serde::{Serialize,Deserialize};
use std::convert::Infallible;
use std::sync::{PoisonError,MutexGuard,RwLockReadGuard};

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
        // match e.kind() {
        //     std::io::ErrorKind::NotFound => Error::Io(0),
        //     std::io::ErrorKind::PermissionDenied => Error::Io(1),
        //     std::io::ErrorKind::ConnectionRefused => Error::Io(2),
        //     std::io::ErrorKind::ConnectionReset => Error::Io(3),
        //     // std::io::ErrorKind::HostUnreachable => Error::Io(4),
        //     // std::io::ErrorKind::NetworkUnreachable => Error::Io(5),
        //     std::io::ErrorKind::ConnectionAborted => Error::Io(6),
        //     std::io::ErrorKind::NotConnected => Error::Io(7),
        //     std::io::ErrorKind::AddrInUse => Error::Io(8),
        //     std::io::ErrorKind::AddrNotAvailable => Error::Io(9),
        //     // std::io::ErrorKind::NetworkDown => Error::Io(10),
        //     std::io::ErrorKind::BrokenPipe => Error::Io(11),
        //     std::io::ErrorKind::AlreadyExists => Error::Io(12),
        //     std::io::ErrorKind::WouldBlock => Error::Io(13),
        //     // std::io::ErrorKind::NotADirectory => Error::Io(14),
        //     // std::io::ErrorKind::IsADirectory => Error::Io(15),
        //     // std::io::ErrorKind::DirectoryNotEmpty => Error::Io(16),
        //     // std::io::ErrorKind::ReadOnlyFilesystem => Error::Io(17),
        //     // std::io::ErrorKind::FilesystemLoop => Error::Io(18),
        //     // std::io::ErrorKind::StaleNetworkFileHandle => Error::Io(19),
        //     std::io::ErrorKind::InvalidInput => Error::Io(20),
        //     std::io::ErrorKind::InvalidData => Error::Io(21),
        //     std::io::ErrorKind::TimedOut => Error::Io(22),
        //     std::io::ErrorKind::WriteZero => Error::Io(23),
        //     // std::io::ErrorKind::StorageFull => Error::Io(24),
        //     // std::io::ErrorKind::NotSeekable => Error::Io(25),
        //     // std::io::ErrorKind::FilesystemQuotaExceeded => Error::Io(26),
        //     // std::io::ErrorKind::FileTooLarge => Error::Io(27),
        //     // std::io::ErrorKind::ResourceBusy => Error::Io(28),
        //     // std::io::ErrorKind::ExecutableFileBusy => Error::Io(29),
        //     // std::io::ErrorKind::Deadlock => Error::Io(30),
        //     // std::io::ErrorKind::CrossesDevices => Error::Io(31),
        //     // std::io::ErrorKind::TooManyLinks => Error::Io(32),
        //     // std::io::ErrorKind::FilenameTooLong => Error::Io(33),
        //     // std::io::ErrorKind::ArgumentListTooLong => Error::Io(34),
        //     std::io::ErrorKind::Interrupted => Error::Io(35),
        //     std::io::ErrorKind::Unsupported => Error::Io(36),
        //     std::io::ErrorKind::UnexpectedEof => Error::Io(37),
        //     std::io::ErrorKind::OutOfMemory => Error::Io(38),
        //     std::io::ErrorKind::Other => Error::Io(39),
        //     _ => Error::Io(40),
        // }
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
impl <'a, T: Sized + 'a> From<PoisonError<MutexGuard<'a,T>>> for Error {
    fn from(_e: PoisonError<MutexGuard<'a,T>>) -> Error {
        Error::PoisonedMutex
    }
}
impl <'a, T: Sized + 'a> From<PoisonError<RwLockReadGuard<'a,T>>> for Error {
    fn from(_e: PoisonError<RwLockReadGuard<'a,T>>) -> Error {
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
            UnknownErrno => Error::NixError(0),
            EPERM       => Error::NixError(1),
            ENOENT      => Error::NixError(2),
            ESRCH       => Error::NixError(3),
            EINTR       => Error::NixError(4),
            EIO         => Error::NixError(5),
            ENXIO       => Error::NixError(6),
            E2BIG       => Error::NixError(7),
            ENOEXEC     => Error::NixError(8),
            EBADF       => Error::NixError(9),
            ECHILD      => Error::NixError(10),
            EAGAIN      => Error::NixError(11),
            ENOMEM      => Error::NixError(12),
            EACCES      => Error::NixError(13),
            EFAULT      => Error::NixError(14),
            ENOTBLK     => Error::NixError(15),
            EBUSY       => Error::NixError(16),
            EEXIST      => Error::NixError(17),
            EXDEV       => Error::NixError(18),
            ENODEV      => Error::NixError(19),
            ENOTDIR     => Error::NixError(20),
            EISDIR      => Error::NixError(21),
            EINVAL      => Error::NixError(22),
            ENFILE      => Error::NixError(23),
            EMFILE      => Error::NixError(24),
            ENOTTY      => Error::NixError(25),
            ETXTBSY     => Error::NixError(26),
            EFBIG       => Error::NixError(27),
            ENOSPC      => Error::NixError(28),
            ESPIPE      => Error::NixError(29),
            EROFS       => Error::NixError(30),
            EMLINK      => Error::NixError(31),
            EPIPE       => Error::NixError(32),
            EDOM        => Error::NixError(33),
            ERANGE      => Error::NixError(34),
            EDEADLK     => Error::NixError(35),
            ENAMETOOLONG => Error::NixError(36),
            ENOLCK      => Error::NixError(37),
            ENOSYS      => Error::NixError(38),
            ENOTEMPTY   => Error::NixError(39),
            ELOOP       => Error::NixError(40),
            ENOMSG      => Error::NixError(41),
            EIDRM       => Error::NixError(42),
            ECHRNG      => Error::NixError(43),
            EL2NSYNC    => Error::NixError(44),
            EL3HLT      => Error::NixError(45),
            EL3RST      => Error::NixError(46),
            ELNRNG      => Error::NixError(47),
            EUNATCH     => Error::NixError(48),
            ENOCSI      => Error::NixError(49),
            EL2HLT      => Error::NixError(50),
            EBADE       => Error::NixError(51),
            EBADR       => Error::NixError(52),
            EXFULL      => Error::NixError(53),
            ENOANO      => Error::NixError(54),
            EBADRQC     => Error::NixError(55),
            EBADSLT     => Error::NixError(56),
            EBFONT      => Error::NixError(57),
            ENOSTR      => Error::NixError(58),
            ENODATA     => Error::NixError(59),
            ETIME       => Error::NixError(60),
            ENOSR       => Error::NixError(61),
            ENONET      => Error::NixError(62),
            ENOPKG      => Error::NixError(63),
            EREMOTE     => Error::NixError(64),
            ENOLINK     => Error::NixError(65),
            EADV        => Error::NixError(66),
            ESRMNT      => Error::NixError(67),
            ECOMM       => Error::NixError(68),
            EPROTO      => Error::NixError(69),
            EMULTIHOP   => Error::NixError(70),
            EDOTDOT     => Error::NixError(71),
            EBADMSG     => Error::NixError(72),
            EOVERFLOW   => Error::NixError(73),
            ENOTUNIQ    => Error::NixError(74),
            EBADFD      => Error::NixError(75),
            EREMCHG     => Error::NixError(76),
            ELIBACC     => Error::NixError(77),
            ELIBBAD     => Error::NixError(78),
            ELIBSCN     => Error::NixError(79),
            ELIBMAX     => Error::NixError(80),
            ELIBEXEC    => Error::NixError(81),
            EILSEQ      => Error::NixError(82),
            ERESTART    => Error::NixError(83),
            ESTRPIPE    => Error::NixError(84),
            EUSERS      => Error::NixError(85),
            ENOTSOCK    => Error::NixError(86),
            EDESTADDRREQ => Error::NixError(87),
            EMSGSIZE    => Error::NixError(88),
            EPROTOTYPE  => Error::NixError(89),
            ENOPROTOOPT => Error::NixError(90),
            EPROTONOSUPPORT => Error::NixError(91),
            ESOCKTNOSUPPORT => Error::NixError(92),
            EOPNOTSUPP  => Error::NixError(93),
            EPFNOSUPPORT => Error::NixError(94),
            EAFNOSUPPORT => Error::NixError(95),
            EADDRINUSE  => Error::NixError(96),
            EADDRNOTAVAIL => Error::NixError(97),
            ENETDOWN    => Error::NixError(98),
            ENETUNREACH => Error::NixError(99),
            ENETRESET   => Error::NixError(100),
            ECONNABORTED => Error::NixError(101),
            ECONNRESET  => Error::NixError(102),
            ENOBUFS     => Error::NixError(103),
            EISCONN     => Error::NixError(104),
            ENOTCONN    => Error::NixError(105),
            ESHUTDOWN   => Error::NixError(106),
            ETOOMANYREFS => Error::NixError(107),
            ETIMEDOUT   => Error::NixError(108),
            ECONNREFUSED => Error::NixError(109),
            EHOSTDOWN   => Error::NixError(110),
            EHOSTUNREACH => Error::NixError(111),
            EALREADY    => Error::NixError(112),
            EINPROGRESS => Error::NixError(113),
            ESTALE      => Error::NixError(114)),
            EUCLEAN     => Error::NixError(115),
            ENOTNAM     => Error::NixError(116),
            ENAVAIL     => Error::NixError(117),
            EISNAM      => Error::NixError(118),
            EREMOTEIO   => Error::NixError(119),
            EDQUOT      => Error::NixError(120),
            ENOMEDIUM   => Error::NixError(121),
            EMEDIUMTYPE => Error::NixError(122),
            ECANCELED   => Error::NixError(123),
            ENOKEY      => Error::NixError(124),
            EKEYEXPIRED => Error::NixError(125),
            EKEYREVOKED => Error::NixError(126),
            EKEYREJECTED => Error::NixError(127),
            EOWNERDEAD  => Error::NixError(128),
            ENOTRECOVERABLE => Error::NixError(129),
            ERFKILL     => Error::NixError(130),
            EHWPOISON   => Error::NixError(131),
    }
}

pub type Result<T> = core::result::Result<T,Error>;