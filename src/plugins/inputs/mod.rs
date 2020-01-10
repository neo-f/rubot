use crate::error::AppError;
use crate::model::Input;

pub enum Inputs {
    Mock(MockInput),
}

pub struct MockInput;

impl Input for MockInput {
    fn gather() -> Result<(), AppError> {
        Ok(())
    }
}
