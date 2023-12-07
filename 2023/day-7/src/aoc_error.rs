use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
