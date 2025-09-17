use faststr::FastStr;

#[derive(thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(std::io::Error),
    #[error("File not found: {0}")]
    FileNotFound(std::path::PathBuf),
    #[error("Syntax error: {source}")]
    Syntax {
        summary: FastStr,
        #[source]
        source: anyhow::Error,
    },
}

// for unwrap
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Syntax { summary, .. } => {
                write!(f, "{}", summary)
            }
            Error::IO(error) => write!(f, "{}", error),
            Error::FileNotFound(path_buf) => write!(f, "{}", path_buf.display()),
        }
    }
}
