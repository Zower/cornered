use std::path::Path;

use thiserror::Error;
use ureq::Response;

use crate::types::{Document, DocumentT, Epub};

pub fn open_document<P: AsRef<Path>>(path: P) -> anyhow::Result<Document> {
    let path = path.as_ref();

    let doc: Box<dyn DocumentT> = match path
        .extension()
        .ok_or(OpenDocumentError::MissingExtension)?
        .to_str()
        .ok_or(OpenDocumentError::ExtensionNotUtf8)?
    {
        "epub" => Box::new(Epub::new(path)?),
        _ => return Err(OpenDocumentError::InvalidExtension(path.to_string_lossy().into()).into()),
    };

    Ok(Document { inner: doc })
}

#[derive(Debug, Error)]
pub enum OpenDocumentError {
    #[error("file is not a valid document")]
    InvalidExtension(String),
    #[error("file is missing extension")]
    MissingExtension,
    #[error("extension is not valid utf-8")]
    ExtensionNotUtf8,
}

pub trait ResponseOkStatus {
    fn ok_status(self) -> anyhow::Result<Self>
    where
        Self: Sized;
}

impl ResponseOkStatus for Response {
    fn ok_status(self) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        match self.status() {
            200..=299 => Ok(self),
            _ => Err(anyhow::anyhow!(
                "request failed with status code {}",
                self.status()
            )),
        }
    }
}
