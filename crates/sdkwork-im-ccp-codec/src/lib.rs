use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodecError {
    message: String,
}

impl CodecError {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for CodecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CodecError {}

pub trait CcpCodec<T> {
    fn codec_name(&self) -> &'static str;

    fn content_type(&self) -> &'static str;

    fn encode(&self, value: &T) -> Result<Vec<u8>, CodecError>;

    fn decode(&self, bytes: &[u8]) -> Result<T, CodecError>;
}
