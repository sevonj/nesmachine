#[derive(Debug)]
pub enum NesMachineError {
    FileIo,
    FileInvalidSig,
    MapperUnsupportedId(usize),
    MapperUnsupportedFeatures,
    MapperUnexpectedChrRomLen(usize),
    MapperUnexpectedPrgRomLen(usize),
}

impl std::fmt::Display for NesMachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NesMachineError::FileIo => write!(f, "Couldn't read file"),
            NesMachineError::FileInvalidSig => write!(f, "Invalid file signature"),
            NesMachineError::MapperUnsupportedId(id) => write!(f, "Unsupported mapper id: {id}"),
            NesMachineError::MapperUnsupportedFeatures => {
                write!(f, "Mapper has unsupported features")
            }
            NesMachineError::MapperUnexpectedChrRomLen(len) => {
                write!(f, "Unexpected CHR ROM length: {len:x}")
            }
            NesMachineError::MapperUnexpectedPrgRomLen(len) => {
                write!(f, "Unexpected PRG ROM length: {len:x}")
            }
        }
    }
}

impl std::error::Error for NesMachineError {}

impl From<std::io::Error> for NesMachineError {
    fn from(_value: std::io::Error) -> Self {
        NesMachineError::FileIo
    }
}
