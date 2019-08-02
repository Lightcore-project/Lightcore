#[derive(Debug)]
pub enum NetworkError {
    Timeout,
    Other { error: Box<dyn std::error::Error + Send + 'static> }
}

impl ::std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            NetworkError::Timeout => f.write_str("Network Timeout"),
            NetworkError::Other { error } => write!(f, "Network error: {}", error)
        }
    }
}

impl ::std::error::Error for NetworkError {
    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            NetworkError::Timeout => None,
            NetworkError::Other { error } => Some(&**error)
        }
    }
}
