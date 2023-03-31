use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use thiserror::Error;
use ureq::Response;

use crate::types::{ContentBlock, Document, DocumentT, Epub, GoUrlResult, Meta, TocEntry};

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

impl DocumentT for Epub {
    fn get_title(&self) -> String {
        self.doc.mdata("title").unwrap()
    }

    fn get_cover(&mut self) -> Option<Vec<u8>> {
        self.doc.get_cover().map(|s| s.0)
    }

    fn go_next(&mut self) -> Option<ContentBlock> {
        self.doc.go_next();
        self.get_content_and_css()
    }

    fn go_prev(&mut self) -> Option<ContentBlock> {
        self.doc.go_prev();
        self.get_content_and_css()
    }

    fn go_to(&mut self, index: usize) -> Option<ContentBlock> {
        self.doc.set_current_page(index);

        self.get_content_and_css()
    }

    fn get_current(&mut self) -> Option<ContentBlock> {
        self.get_content_and_css()
    }

    fn get_resource(&mut self, id: &str) -> Option<Vec<u8>> {
        self.doc.get_resource_by_path(id)
    }

    fn meta(&mut self) -> Meta {
        Meta {
            title: self.doc.mdata("title"),
            author: self.doc.mdata("author"),
            cover: self.doc.get_cover().map(|s| s.0),
        }
    }

    fn get_spine(&mut self) -> HashMap<String, (String, String)> {
        self.doc
            .resources
            .iter()
            .map(|(k, v)| (k.clone(), (v.0.to_str().unwrap().to_string(), v.1.clone())))
            .collect()
    }

    fn get_toc(&mut self) -> Vec<TocEntry> {
        self.doc
            .toc
            .iter()
            .map(|np| TocEntry {
                label: np.label.to_string(),
                url: np.content.to_str().unwrap().to_string().clone(),
                count: np.play_order,
            })
            .collect::<Vec<_>>()
    }

    fn go_url(&mut self, url: &str) -> anyhow::Result<Option<GoUrlResult>> {
        let chap = self
            .doc
            .resource_uri_to_chapter(&PathBuf::from(url))
            .ok_or(anyhow!("Could not find chapter for url {}", url))?;

        self.doc.set_current_page(chap);

        let content = self.get_content_and_css();
        Ok(content.map(|content| GoUrlResult {
            content,
            chapter: chap,
        }))

        // return Ok(self.doc.get_current_str().map(|s| GoUrlResult {
        //     content: ContentBlock::new(
        //         s.0,
        //         ContentType::Html {
        //             extra_css: Some(css),
        //         },
        //     ),
        //     chapter: chap,
        // }));
    }
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
