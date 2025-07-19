use thiserror::Error;

#[derive(Debug, Error)]
pub enum OnvifError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("XML serialization error: {0}")]
    XmlSerialization(#[from] quick_xml::se::SeError),
    
    #[error("SOAP Fault: {code} - {reason}")]
    SoapFault {
        code: String,
        reason: String,
        detail: Option<String>,
    },

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}