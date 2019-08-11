#[derive(Debug)]
pub enum Error {
    Illegal,
    SetError,
    GetError,
    Repeated,
    BatchError,
    DeleteError,
    GetOrderError,
    None
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        match &self {
            Error::Illegal => String::from("Illegal"),
            Error::SetError => String::from("SetError"),
            Error::GetError => String::from("GetError"),
            Error::Repeated => String::from("Repeated"),
            Error::BatchError => String::from("Batcherror"),
            Error::DeleteError => String::from("Deleteerror"),
            Error::GetOrderError => String::from("GetOrderError"),
            Error::None => String::from("None")
        }
    }
}
