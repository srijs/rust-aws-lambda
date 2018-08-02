use std::fmt;

/// Error that can occur in the runtime.
#[derive(Debug)]
pub struct RuntimeError {
    inner: RuntimeErrorInner,
}

impl RuntimeError {
    pub(crate) fn from_io(err: ::std::io::Error) -> Self {
        RuntimeError {
            inner: RuntimeErrorInner::Io(err),
        }
    }

    pub(crate) fn environment(reason: &'static str) -> Self {
        RuntimeError {
            inner: RuntimeErrorInner::Environment(reason),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            RuntimeErrorInner::Io(ref err) => write!(f, "i/o error: {}", err),
            RuntimeErrorInner::Environment(reason) => write!(f, "environment error: {}", reason),
        }
    }
}

impl ::std::error::Error for RuntimeError {
    fn cause(&self) -> Option<&::std::error::Error> {
        match self.inner {
            RuntimeErrorInner::Io(ref err) => Some(err),
            RuntimeErrorInner::Environment(_) => None,
        }
    }
}

#[derive(Debug)]
enum RuntimeErrorInner {
    Io(::std::io::Error),
    Environment(&'static str),
}

#[derive(Debug)]
pub(crate) enum ConnectionError {
    Io(::std::io::Error),
    Gob(::gob::Error),
    UnexpectedEndOfStream,
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectionError::Gob(ref err) => fmt::Display::fmt(err, f),
            ConnectionError::Io(ref err) => write!(f, "i/o error: {}", err),
            ConnectionError::UnexpectedEndOfStream => write!(f, "unexpected end of stream"),
        }
    }
}

impl From<::gob::Error> for ConnectionError {
    fn from(err: ::gob::Error) -> ConnectionError {
        ConnectionError::Gob(err)
    }
}

impl From<::std::io::Error> for ConnectionError {
    fn from(err: ::std::io::Error) -> ConnectionError {
        ConnectionError::Io(err)
    }
}

impl From<::void::Void> for ConnectionError {
    fn from(err: ::void::Void) -> ConnectionError {
        ::void::unreachable(err)
    }
}
