use crate::Region;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GuestError {
    #[error("Invalid flag value {0}")]
    InvalidFlagValue(&'static str),
    #[error("Invalid enum value {0}")]
    InvalidEnumValue(&'static str),
    #[error("Pointer overflow")]
    PtrOverflow,
    #[error("Pointer out of bounds: {0:?}")]
    PtrOutOfBounds(Region),
    #[error("Pointer not aligned to {1}: {0:?}")]
    PtrNotAligned(Region, u32),
    #[error("Pointer already borrowed: {0:?}")]
    PtrBorrowed(Region),
    #[error("Borrow checker out of handles")]
    BorrowCheckerOutOfHandles,
    #[error("Slice length mismatch")]
    SliceLengthsDiffer,
    #[error("Invalid UTF-8 encountered: {0:?}")]
    InvalidUtf8(#[from] ::std::str::Utf8Error),
    #[error("Int conversion error: {0:?}")]
    TryFromIntError(#[from] ::std::num::TryFromIntError),
}

pub struct Error<T> {
    err: anyhow::Error,
    ty: std::marker::PhantomData<T>,
}

impl<T: std::error::Error + Send + Sync + 'static> Error<T> {
    pub fn new(err: T) -> Error<T> {
        Error {
            err: err.into(),
            ty: std::marker::PhantomData,
        }
    }

    pub fn downcast(self) -> Result<T, anyhow::Error> {
        self.err.downcast::<T>()
    }

    pub fn downcast_ref(&self) -> Option<&T> {
        self.err.downcast_ref::<T>()
    }

    pub fn downcast_mut(&mut self) -> Option<&mut T> {
        self.err.downcast_mut::<T>()
    }
}

impl<T> Error<T> {
    pub fn trap(err: anyhow::Error) -> Error<T> {
        Error {
            err,
            ty: std::marker::PhantomData,
        }
    }

    pub fn into_inner(self) -> anyhow::Error {
        self.err
    }

    pub fn context<C>(self, context: C) -> Error<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        Self {
            err: self.err.context(context).into(),
            ty: self.ty,
        }
    }
}

impl<T> std::ops::Deref for Error<T> {
    type Target = dyn std::error::Error + Send + Sync + 'static;
    fn deref(&self) -> &Self::Target {
        self.err.deref()
    }
}
impl<T> std::ops::DerefMut for Error<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.err.deref_mut()
    }
}

impl<T> std::fmt::Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}

impl<T> std::fmt::Debug for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f)
    }
}

impl<T> std::error::Error for Error<T> {}
