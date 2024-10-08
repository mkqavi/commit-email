use std::{
    error::Error,
    fmt::{Debug, Display},
    sync::PoisonError,
};

#[derive(Debug)]
pub enum RepoError<T> {
    LockPoisoned(PoisonError<T>),
    Git2(git2::Error),
}

impl<T> Display for RepoError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::LockPoisoned(_) => write!(f, "Lock was poisoned"),
            RepoError::Git2(e) => write!(f, "Git2 error: {}", e),
        }
    }
}

impl<T: Debug> Error for RepoError<T> {}

impl<T> From<PoisonError<T>> for RepoError<T> {
    fn from(value: PoisonError<T>) -> Self {
        RepoError::LockPoisoned(value)
    }
}

impl<T> From<git2::Error> for RepoError<T> {
    fn from(value: git2::Error) -> Self {
        RepoError::Git2(value)
    }
}
