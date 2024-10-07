#[derive(Debug)]
pub enum SetErrorType {
    DuplicateElement,
    FailedToRemoveElement,
    None,
}

#[derive(Debug)]
pub struct SetError {
    error_type: SetErrorType,
    msg: String,
}

impl SetError {
    pub fn new(msg: String, kind: SetErrorType) -> SetError {
        SetError {
            error_type: kind,
            msg,
        }
    }
}