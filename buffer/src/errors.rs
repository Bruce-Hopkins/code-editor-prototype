use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum EngineErrors {
    #[error("Cannot run operation `{operation_name:?}` because it is out of bounds: {explaination:?}`")]
    InvalidPosition{
        operation_name: String,
        explaination: String
    },
    #[error("File error {0:?}")]
    FileError(String),
    #[error("{0:?}")]
    InvalidString(String)
}


impl EngineErrors {
    pub fn from_file_error<ToSringStatic>(operation: ToSringStatic) -> impl FnOnce(io::Error) -> Self
    where ToSringStatic: ToString + 'static {
        move |error: io::Error| {
            let error_message = format!("File operation {} failed: {}", operation.to_string(), error);
            EngineErrors::FileError(error_message)
        }
    }
}
