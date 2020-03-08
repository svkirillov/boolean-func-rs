use std::fmt;

pub enum BFKindOfError {
    Error,
    ParseError,
    BadVectorSize,
}

impl fmt::Display for BFKindOfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BFKindOfError::Error => write!(f, "BFKindOfError::Error"),
            BFKindOfError::ParseError => write!(f, "BFKindOfError::ParseError"),
            BFKindOfError::BadVectorSize => write!(f, "BFKindOfError::BadVectorSize"),
        }
    }
}

impl fmt::Debug for BFKindOfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct BFError {
    kind: BFKindOfError,
    msg: String,
}

impl BFError {
    pub fn new(kind: BFKindOfError, msg: &str) -> BFError {
        BFError {
            kind: kind,
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for BFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Debug for BFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}
