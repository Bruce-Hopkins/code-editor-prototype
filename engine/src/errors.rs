use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum EngineErrors {
    #[error("Cannot run operation `{operation_name:?}` because it is out of bounds: {explaination:?}`")]
    InvalidPosition{
        operation_name: String,
        explaination: String
    },
    #[error("Failed run file operation {operation_name:?}. Explaination: {explaination:?}")]
    FileError{
        operation_name: String,
        explaination: String
    },
    #[error("{0:?}")]
    InvalidString(String)
}


impl EngineErrors {
    pub fn from_file_error<ToSringStatic>(operation: ToSringStatic) -> impl FnOnce(io::Error) -> Self
    where ToSringStatic: ToString + 'static {
        move |error: io::Error| {
            EngineErrors::FileError { 
                operation_name: operation.to_string(), 
                explaination: error.to_string()
            }
        }
    }
}
