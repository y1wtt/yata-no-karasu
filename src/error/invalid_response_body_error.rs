use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub(crate) struct InvalidResponseBodyError;

impl Display for InvalidResponseBodyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fail to find ip from response")
    }
}

impl std::error::Error for InvalidResponseBodyError {}