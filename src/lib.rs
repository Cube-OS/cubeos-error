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

pub type Result<T> = core::result::Result<T,Error>;