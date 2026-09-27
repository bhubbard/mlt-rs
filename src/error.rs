use thiserror::Error;

#[derive(Error, Debug)]
pub enum MltError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("XML deserialization error: {0}")]
    XmlDe(#[from] quick_xml::DeError),

    #[error("XML serialization error: {0}")]
    XmlSe(#[from] quick_xml::SeError),

    #[error("Invalid profile: {0}")]
    InvalidProfile(String),

    #[error("Producer error: {0}")]
    Producer(String),

    #[error("Filter error: {0}")]
    Filter(String),

    #[error("Transition error: {0}")]
    Transition(String),

    #[error("Render error: {0}")]
    Render(String),
}

pub type Result<T> = std::result::Result<T, MltError>;
