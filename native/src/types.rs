use std::{collections::HashMap, fmt::Debug, fs::File, io::BufReader};

use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub uuid: String,
    pub path: String,
    pub position: Position,
}

pub struct Meta {
    pub title: Option<String>,
    pub author: Option<String>,
    pub cover: Option<Vec<u8>>,
}

pub trait DocumentT: Send + Sync + Debug {
    fn get_title(&self) -> String;
    fn get_cover(&mut self) -> Option<Vec<u8>>;
    fn go_next(&mut self) -> Option<ContentBlock>;
    fn go_prev(&mut self) -> Option<ContentBlock>;
    fn go_to(&mut self, index: usize) -> Option<ContentBlock>;
    fn go_url(&mut self, _: &str) -> anyhow::Result<Option<GoUrlResult>> {
        Err(anyhow::anyhow!("Not implemented"))
    }
    fn get_current(&mut self) -> Option<ContentBlock>;
    fn get_resource(&mut self, id: &str) -> Option<Vec<u8>>;
    fn get_spine(&mut self) -> HashMap<String, (String, String)>;
    fn get_toc(&mut self) -> Vec<TocEntry>;
    fn meta(&mut self) -> Meta;
}

#[derive(Debug)]
pub struct Document {
    pub inner: Box<dyn DocumentT>,
}

#[derive(Debug)]
pub struct Epub {
    pub(crate) doc: EpubDoc<BufReader<File>>,
}

impl Epub {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let doc = EpubDoc::new(path)?;
        Ok(Self { doc })
    }

    pub(crate) fn get_content_and_css(&mut self) -> Option<ContentBlock> {
        let css = self.get_css();
        self.doc.get_current_str().map(|s| {
            ContentBlock::new(
                s.0,
                self.doc.get_current_page(),
                ContentType::Html {
                    extra_css: Some(css),
                },
            )
        })
    }

    fn get_css(&mut self) -> String {
        let ids = self
            .doc
            .resources
            .iter_mut()
            .filter_map(|(k, v)| {
                if v.1 == "text/css" {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        ids.iter()
            .map(|id| self.doc.get_resource(&id).expect("resource to exist"))
            .map(|(css, _)| {
                format!(
                    "
                    <style>
                    {}
                    </style>
                    ",
                    String::from_utf8(css).expect("valid utf8")
                )
            })
            .collect::<String>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OpenDocumentId(pub u64);

#[derive(Debug, Clone)]
pub struct ContentBlock {
    pub content: String,
    pub chapter: usize,
    pub content_type: ContentType,
}

impl ContentBlock {
    pub fn new(content: String, chapter: usize, content_type: ContentType) -> Self {
        Self {
            content,
            chapter,
            content_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContentType {
    // Text,
    Html { extra_css: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definitions {
    pub word: String,
    pub meanings: Vec<Meaning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meaning {
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub definition: String,
    pub example: Option<String>,
    pub synonyms: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    pub chapter: usize,
    pub offset: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeviceFlowResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    // expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CodeResponse {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct FileResponse {
    pub name: String,
    pub download_url: String,
    pub sha: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GithubUser {
    pub login: String,
    pub id: u64,
}

pub struct GoUrlResult {
    pub content: ContentBlock,
    pub chapter: usize,
}

pub struct TocEntry {
    pub label: String,
    pub url: String,
    pub count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct UploadedFile {
    pub uuid: String,
    pub file_name: String,
    pub content: Vec<u8>,
    pub position: Position,
}
