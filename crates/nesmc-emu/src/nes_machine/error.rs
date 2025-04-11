#[derive(Debug)]
pub enum NesMachineError {
    FileIo,
    FileInvalidSig,
    UnsupportedMapper,
}

impl std::fmt::Display for NesMachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NesMachineError::FileIo => write!(f, "Couldn't read file"),
            NesMachineError::FileInvalidSig => write!(f, "Invalid file signature"),
            NesMachineError::UnsupportedMapper => write!(f, "Unsupported mapper type"),
        }
    }
}

impl std::error::Error for NesMachineError {}

impl From<std::io::Error> for NesMachineError {
    fn from(_value: std::io::Error) -> Self {
        NesMachineError::FileIo
    }
}
