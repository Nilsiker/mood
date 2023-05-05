#[derive(Debug)]
pub enum MoodError {
    ConfigFileError(String),
    JournalFileError(String),
}

impl std::error::Error for MoodError {}

impl std::fmt::Display for MoodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoodError::ConfigFileError(string) => write!(
                f,
                "Failed an operation related to the configuration file: {string}"
            ),
            MoodError::JournalFileError(string) => write!(
                f,
                "Failed an operation related to the journal file: {string}"
            ),
        }
    }
}
