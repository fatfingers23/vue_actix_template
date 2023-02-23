use crate::domain::error::RepositoryError;
pub use actix_threadpool::{run, BlockingError};
use bb8::RunError;
use diesel::r2d2;

use diesel_async::pooled_connection::PoolError;

pub type AsyncPoolError<T> = BlockingError<T>;

#[derive(Debug)]
pub struct DieselRepositoryError(RepositoryError);

impl DieselRepositoryError {
    pub fn into_inner(self) -> RepositoryError {
        self.0
    }
}

impl From<r2d2::Error> for DieselRepositoryError {
    fn from(error: r2d2::Error) -> DieselRepositoryError {
        DieselRepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}

impl From<diesel::result::Error> for DieselRepositoryError {
    fn from(error: diesel::result::Error) -> DieselRepositoryError {
        DieselRepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}

impl<T: std::fmt::Debug> From<AsyncPoolError<T>> for DieselRepositoryError {
    fn from(error: AsyncPoolError<T>) -> DieselRepositoryError {
        DieselRepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}

impl From<RunError<PoolError>> for DieselRepositoryError {
    fn from(error: RunError<PoolError>) -> DieselRepositoryError {
        DieselRepositoryError(RepositoryError {
            message: error.to_string(),
        })
    }
}
