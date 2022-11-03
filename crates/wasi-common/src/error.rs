use std::fmt;

/// Internal error type for the `wasi-common` crate.
///
/// This Contains variants of the WASI `$errno` type that are used internally
/// by the crate, and which aren't one-to-one with a `std::io::ErrorKind`
/// error.
///
/// When the Rust [io_error_more] feature is stabilized, that will enable
/// us to replace several more of these codes with `std::io::ErrorKind` codes.
///
/// [io_error_more]: https://doc.rust-lang.org/beta/unstable-book/library-features/io-error-more.html
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Trap execution
    #[error("Trap: {0}")]
    Trap(#[source] anyhow::Error),
    /// Argument list too long
    #[error("TooBig: Argument list too long")]
    TooBig,
    /// Resource temporarily unavailable
    #[error("Again: Resource temporarily unavailable")]
    Again,
    /// Permission denied
    #[error("Acces: Permission denied")]
    Acces,
    /// Address in use
    #[error("Addrinuse: Address in use")]
    Addrinuse,
    /// Address family not supported
    #[error("Afnosupport: Address family not supported")]
    Afnosupport,
    /// Address not available
    #[error("Addrnotavail: Address not available")]
    Addrnotavail,
    /// Connection already in progress
    #[error("Already: Connection already in progress")]
    Already,
    /// Bad file descriptor
    #[error("Badf: Bad file descriptor")]
    Badf,
    /// Device or resource busy
    #[error("Busy: Device or resource busy")]
    Busy,
    /// Operation canceled
    #[error("Canceled: Operation canceled")]
    Canceled,
    /// Connection aborted
    #[error("Connaborted: Connection aborted")]
    Connaborted,
    /// Connection refused
    #[error("Connrefused: Connection refused")]
    Connrefused,
    /// Connection reset
    #[error("Connreset: Connection reset")]
    Connreset,
    /// Destination address required
    #[error("Destaddrreq: Destination address required")]
    Destaddrreq,
    /// Reserved
    #[error("Dquot: Reserved")]
    Dquot,
    /// File exists
    #[error("Exist: File exists")]
    Exist,
    /// Bad address
    #[error("Fault: Bad address")]
    Fault,
    /// File too large
    #[error("Fbig: File too large")]
    Fbig,
    /// Host is unreachable
    #[error("Hostunreach: Host is unreachable")]
    Hostunreach,
    /// Illegal byte sequence
    #[error("Ilseq: Illegal byte sequence")]
    Ilseq,
    /// Operation in progress
    #[error("Inprogress: Operation in progress")]
    Inprogress,
    /// Interrupted function
    #[error("Intr: Interrrupted function")]
    Intr,
    /// Invalid argument
    #[error("Inval: Invalid argument")]
    Inval,
    /// I/O error
    #[error("Io: I/O error")]
    Io,
    /// Socket is connected
    #[error("Isconn: Socket is connected")]
    Isconn,
    /// Is a directory
    #[error("Isdir: Is a directory")]
    Isdir,
    /// Too many levels of symbolic links
    #[error("Loop: Too many levels of symbolic links")]
    Loop,
    /// File descriptor value too large
    #[error("Mfile: File descriptor value too large")]
    Mfile,
    /// Too many links
    #[error("Mlink: Too many links")]
    Mlink,
    /// Message too large
    #[error("Msgsize: Message too large")]
    Msgsize,
    /// Filename too long
    #[error("Nametoolong: Filename too long")]
    Nametoolong,
    /// Network down
    #[error("Netdown: Network down")]
    Netdown,
    /// Connection aborted by network
    #[error("Netreset: Connection aborted by network")]
    Netreset,
    /// Network unreachable
    #[error("Netunreach: Network unreachable")]
    Netunreach,
    /// Too many files open in system
    #[error("Nfile: Too many files open in system")]
    Nfile,
    /// No buffer space available
    #[error("Nobufs: No buffer space available")]
    Nobufs,
    /// Not found
    #[error("Noent: Not found")]
    Noent,
    /// Not enough space
    #[error("Nomem: Not enough space")]
    Nomem,
    /// Protocol not available
    #[error("Noprotoopt: Protocol not available")]
    Noprotoopt,
    /// No space left on device
    #[error("Nospc: No space left on device")]
    Nospc,
    /// The socket is not connected
    #[error("Notconn: The socket is not connected")]
    Notconn,
    /// Not a directory or a symbolic link to a directory.
    #[error("Notdir: Not a directory or a symbolic link to a directory")]
    Notdir,
    /// Directory not empty
    #[error("Notempty: Directory not empty")]
    Notempty,
    /// Not a socket
    #[error("Notsock: Not a socket")]
    Notsock,
    /// Not supported, or operation not supported on socket.
    #[error("Notsup: Not supported, or operation not supported on socket")]
    Notsup,
    /// Value too large to be stored in data type.
    #[error("Overflow: Value too large to be stored in data type")]
    Overflow,
    /// Permission denied
    #[error("Perm: Permission denied")]
    Perm,
    /// Broken pipe
    #[error("Pipe: Broken pipe")]
    Pipe,
    /// Protocol not supported
    #[error("Protonosupport: Protocol not supported")]
    Protonosupport,
    /// Protocol wrong type for socket
    #[error("Prototype: Protocol wrong type for socket")]
    Prototype,
    /// Result too large
    #[error("Range: Result too large")]
    Range,
    /// Invalid seek
    #[error("Spipe: Invalid seek")]
    Spipe,
    /// Reserved
    #[error("Stale: Reserved")]
    Stale,
    /// Connection timed out
    #[error("Timedout: Connection timed out")]
    Timedout,
}

impl Error {
    pub fn trap(e: anyhow::Error) -> Self {
        Self::Trap(e)
    }
    pub fn not_found() -> Self {
        Self::Noent
    }
    pub fn too_big() -> Self {
        Self::TooBig
    }
    pub fn badf() -> Self {
        Self::Badf
    }
    pub fn exist() -> Self {
        Self::Exist
    }
    pub fn illegal_byte_sequence() -> Self {
        Self::Ilseq
    }
    pub fn invalid_argument() -> Self {
        Self::Inval
    }
    pub fn io() -> Self {
        Self::Io
    }
    pub fn name_too_long() -> Self {
        Self::Nametoolong
    }
    pub fn not_dir() -> Self {
        Self::Notdir
    }
    pub fn not_supported() -> Self {
        Self::Notsup
    }
    pub fn overflow() -> Self {
        Self::Overflow
    }
    pub fn range() -> Self {
        Self::Range
    }
    pub fn seek_pipe() -> Self {
        Self::Spipe
    }
    pub fn perm() -> Self {
        Self::Perm
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Error {
        Error::Overflow
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        #[cfg(unix)]
        fn raw_error_code(err: &std::io::Error) -> Option<Error> {
            use rustix::io::Errno;
            match Errno::from_io_error(err) {
                Some(Errno::AGAIN) => Some(Error::Again),
                Some(Errno::PIPE) => Some(Error::Pipe),
                Some(Errno::PERM) => Some(Error::Perm),
                Some(Errno::NOENT) => Some(Error::Noent),
                Some(Errno::NOMEM) => Some(Error::Nomem),
                Some(Errno::TOOBIG) => Some(Error::TooBig),
                Some(Errno::IO) => Some(Error::Io),
                Some(Errno::BADF) => Some(Error::Badf),
                Some(Errno::BUSY) => Some(Error::Busy),
                Some(Errno::ACCESS) => Some(Error::Acces),
                Some(Errno::FAULT) => Some(Error::Fault),
                Some(Errno::NOTDIR) => Some(Error::Notdir),
                Some(Errno::ISDIR) => Some(Error::Isdir),
                Some(Errno::INVAL) => Some(Error::Inval),
                Some(Errno::EXIST) => Some(Error::Exist),
                Some(Errno::FBIG) => Some(Error::Fbig),
                Some(Errno::NOSPC) => Some(Error::Nospc),
                Some(Errno::SPIPE) => Some(Error::Spipe),
                Some(Errno::MFILE) => Some(Error::Mfile),
                Some(Errno::MLINK) => Some(Error::Mlink),
                Some(Errno::NAMETOOLONG) => Some(Error::Nametoolong),
                Some(Errno::NFILE) => Some(Error::Nfile),
                Some(Errno::NOTEMPTY) => Some(Error::Notempty),
                Some(Errno::LOOP) => Some(Error::Loop),
                Some(Errno::OVERFLOW) => Some(Error::Overflow),
                Some(Errno::ILSEQ) => Some(Error::Ilseq),
                Some(Errno::NOTSUP) => Some(Error::Notsup),
                Some(Errno::ADDRINUSE) => Some(Error::Addrinuse),
                Some(Errno::CANCELED) => Some(Error::Canceled),
                Some(Errno::ADDRNOTAVAIL) => Some(Error::Addrnotavail),
                Some(Errno::AFNOSUPPORT) => Some(Error::Afnosupport),
                Some(Errno::ALREADY) => Some(Error::Already),
                Some(Errno::CONNABORTED) => Some(Error::Connaborted),
                Some(Errno::CONNREFUSED) => Some(Error::Connrefused),
                Some(Errno::CONNRESET) => Some(Error::Connreset),
                Some(Errno::DESTADDRREQ) => Some(Error::Destaddrreq),
                Some(Errno::DQUOT) => Some(Error::Dquot),
                Some(Errno::HOSTUNREACH) => Some(Error::Hostunreach),
                Some(Errno::INPROGRESS) => Some(Error::Inprogress),
                Some(Errno::INTR) => Some(Error::Intr),
                Some(Errno::ISCONN) => Some(Error::Isconn),
                Some(Errno::MSGSIZE) => Some(Error::Msgsize),
                Some(Errno::NETDOWN) => Some(Error::Netdown),
                Some(Errno::NETRESET) => Some(Error::Netreset),
                Some(Errno::NETUNREACH) => Some(Error::Netunreach),
                Some(Errno::NOBUFS) => Some(Error::Nobufs),
                Some(Errno::NOPROTOOPT) => Some(Error::Noprotoopt),
                Some(Errno::NOTCONN) => Some(Error::Notconn),
                Some(Errno::NOTSOCK) => Some(Error::Notsock),
                Some(Errno::PROTONOSUPPORT) => Some(Error::Protonosupport),
                Some(Errno::PROTOTYPE) => Some(Error::Prototype),
                Some(Errno::STALE) => Some(Error::Stale),
                Some(Errno::TIMEDOUT) => Some(Error::Timedout),

                // On some platforms, these have the same value as other errno values.
                #[allow(unreachable_patterns)]
                Some(Errno::WOULDBLOCK) => Some(Error::Again),
                #[allow(unreachable_patterns)]
                Some(Errno::OPNOTSUPP) => Some(Error::Notsup),

                _ => None,
            }
        }
        #[cfg(windows)]
        fn raw_error_code(err: &std::io::Error) -> Option<Error> {
            use windows_sys::Win32::Foundation;
            use windows_sys::Win32::Networking::WinSock;

            match err.raw_os_error().map(|code| code as u32) {
                Some(Foundation::ERROR_BAD_ENVIRONMENT) => return Some(Error::TooBig),
                Some(Foundation::ERROR_FILE_NOT_FOUND) => return Some(Error::Noent),
                Some(Foundation::ERROR_PATH_NOT_FOUND) => return Some(Error::Noent),
                Some(Foundation::ERROR_TOO_MANY_OPEN_FILES) => return Some(Error::Nfile),
                Some(Foundation::ERROR_ACCESS_DENIED) => return Some(Error::Acces),
                Some(Foundation::ERROR_SHARING_VIOLATION) => return Some(Error::Acces),
                Some(Foundation::ERROR_PRIVILEGE_NOT_HELD) => return Some(Error::Perm),
                Some(Foundation::ERROR_INVALID_HANDLE) => return Some(Error::Badf),
                Some(Foundation::ERROR_INVALID_NAME) => return Some(Error::Noent),
                Some(Foundation::ERROR_NOT_ENOUGH_MEMORY) => return Some(Error::Nomem),
                Some(Foundation::ERROR_OUTOFMEMORY) => return Some(Error::Nomem),
                Some(Foundation::ERROR_DIR_NOT_EMPTY) => return Some(Error::Notempty),
                Some(Foundation::ERROR_NOT_READY) => return Some(Error::Busy),
                Some(Foundation::ERROR_BUSY) => return Some(Error::Busy),
                Some(Foundation::ERROR_NOT_SUPPORTED) => return Some(Error::Notsup),
                Some(Foundation::ERROR_FILE_EXISTS) => return Some(Error::Exist),
                Some(Foundation::ERROR_BROKEN_PIPE) => return Some(Error::Pipe),
                Some(Foundation::ERROR_BUFFER_OVERFLOW) => return Some(Error::Nametoolong),
                Some(Foundation::ERROR_NOT_A_REPARSE_POINT) => return Some(Error::Inval),
                Some(Foundation::ERROR_NEGATIVE_SEEK) => return Some(Error::Inval),
                Some(Foundation::ERROR_DIRECTORY) => return Some(Error::Notdir),
                Some(Foundation::ERROR_ALREADY_EXISTS) => return Some(Error::Exist),
                Some(Foundation::ERROR_STOPPED_ON_SYMLINK) => return Some(Error::Loop),
                Some(Foundation::ERROR_DIRECTORY_NOT_SUPPORTED) => return Some(Error::Isdir),
                _ => {}
            }

            match err.raw_os_error() {
                Some(WinSock::WSAEWOULDBLOCK) => Some(Error::Again),
                Some(WinSock::WSAECANCELLED) => Some(Error::Canceled),
                Some(WinSock::WSA_E_CANCELLED) => Some(Error::Canceled),
                Some(WinSock::WSAEBADF) => Some(Error::Badf),
                Some(WinSock::WSAEFAULT) => Some(Error::Fault),
                Some(WinSock::WSAEINVAL) => Some(Error::Inval),
                Some(WinSock::WSAEMFILE) => Some(Error::Mfile),
                Some(WinSock::WSAENAMETOOLONG) => Some(Error::Nametoolong),
                Some(WinSock::WSAENOTEMPTY) => Some(Error::Notempty),
                Some(WinSock::WSAELOOP) => Some(Error::Loop),
                Some(WinSock::WSAEOPNOTSUPP) => Some(Error::Notsup),
                Some(WinSock::WSAEADDRINUSE) => Some(Error::Addrinuse),
                Some(WinSock::WSAEACCES) => Some(Error::Acces),
                Some(WinSock::WSAEADDRNOTAVAIL) => Some(Error::Addrnotavail),
                Some(WinSock::WSAEAFNOSUPPORT) => Some(Error::Afnosupport),
                Some(WinSock::WSAEALREADY) => Some(Error::Already),
                Some(WinSock::WSAECONNABORTED) => Some(Error::Connaborted),
                Some(WinSock::WSAECONNREFUSED) => Some(Error::Connrefused),
                Some(WinSock::WSAECONNRESET) => Some(Error::Connreset),
                Some(WinSock::WSAEDESTADDRREQ) => Some(Error::Destaddrreq),
                Some(WinSock::WSAEDQUOT) => Some(Error::Dquot),
                Some(WinSock::WSAEHOSTUNREACH) => Some(Error::Hostunreach),
                Some(WinSock::WSAEINPROGRESS) => Some(Error::Inprogress),
                Some(WinSock::WSAEINTR) => Some(Error::Intr),
                Some(WinSock::WSAEISCONN) => Some(Error::Isconn),
                Some(WinSock::WSAEMSGSIZE) => Some(Error::Msgsize),
                Some(WinSock::WSAENETDOWN) => Some(Error::Netdown),
                Some(WinSock::WSAENETRESET) => Some(Error::Netreset),
                Some(WinSock::WSAENETUNREACH) => Some(Error::Netunreach),
                Some(WinSock::WSAENOBUFS) => Some(Error::Nobufs),
                Some(WinSock::WSAENOPROTOOPT) => Some(Error::Noprotoopt),
                Some(WinSock::WSAENOTCONN) => Some(Error::Notconn),
                Some(WinSock::WSAENOTSOCK) => Some(Error::Notsock),
                Some(WinSock::WSAEPROTONOSUPPORT) => Some(Error::Protonosupport),
                Some(WinSock::WSAEPROTOTYPE) => Some(Error::Prototype),
                Some(WinSock::WSAESTALE) => Some(Error::Stale),
                Some(WinSock::WSAETIMEDOUT) => Some(Error::Timedout),
                _ => None,
            }
        }

        match raw_error_code(&err) {
            Some(errno) => errno,
            None => match err.kind() {
                std::io::ErrorKind::NotFound => Error::Noent,
                std::io::ErrorKind::PermissionDenied => Error::Perm,
                std::io::ErrorKind::AlreadyExists => Error::Exist,
                std::io::ErrorKind::InvalidInput => Error::Inval,
                _ => Error::Trap(anyhow::anyhow!(err).context("Unknown OS error")),
            },
        }
    }
}
/// An error returned from the `proc_exit` host syscall.
///
/// Embedders can test if an error returned from wasm is this error, in which
/// case it may signal a non-fatal trap.
#[derive(Debug)]
pub struct I32Exit(pub i32);

impl fmt::Display for I32Exit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Exited with i32 exit status {}", self.0)
    }
}

impl std::error::Error for I32Exit {}
