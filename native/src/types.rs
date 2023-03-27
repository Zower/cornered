use std::{collections::HashMap, fmt::Debug, fs::File, io::BufReader};

use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};

use crate::api::{Meta, T};

pub trait DocumentT: Send + Sync + Debug {
    fn get_title(&self) -> String;
    fn get_cover(&mut self) -> Option<Vec<u8>>;
    fn go_next(&mut self) -> Option<ContentBlock>;
    fn go_prev(&mut self) -> Option<ContentBlock>;
    fn go_to(&mut self, index: usize) -> Option<ContentBlock>;
    fn get_current(&mut self) -> Option<ContentBlock>;
    fn get_resources(&mut self, id: &str) -> Vec<T>;
    fn get_spine(&mut self) -> HashMap<String, (String, String)>;
    fn meta(&mut self) -> Meta;
}

#[derive(Debug)]
pub struct Document {
    pub inner: Box<dyn DocumentT>,
}

#[derive(Debug)]
pub struct Epub {
    doc: EpubDoc<BufReader<File>>,
}

impl Epub {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let doc = EpubDoc::new(path)?;
        Ok(Self { doc })
    }
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
        self.doc
            .get_current_str()
            .map(|s| ContentBlock::new(s.0, ContentType::Html))
    }

    fn go_prev(&mut self) -> Option<ContentBlock> {
        self.doc.go_prev();
        self.doc
            .get_current_str()
            .map(|s| ContentBlock::new(s.0, ContentType::Html))
    }

    fn go_to(&mut self, index: usize) -> Option<ContentBlock> {
        self.doc.set_current_page(index);

        self.doc
            .get_current_str()
            .map(|s| ContentBlock::new(s.0, ContentType::Html))
    }

    fn get_current(&mut self) -> Option<ContentBlock> {
        self.doc
            .get_current_str()
            .map(|s| ContentBlock::new(s.0, ContentType::Html))
    }

    // TODO PLEASE HELP ME
    // THIS IS Bad CODE USE getresourcebypath instaed in fturue PLEASE TORD PLEASE
    fn get_resources(&mut self, id: &str) -> Vec<T> {
        let x: Vec<_> = self
            .doc
            .resources
            .iter()
            .map(|(k, v)| v.0.clone())
            .collect();

        x.iter()
            .map(|v| T {
                path: v.to_str().unwrap().to_string(),
                content: self.doc.get_resource_by_path(&v).unwrap(),
            })
            .collect()

        // T {
        //                 path: v.0.to_str().unwrap().to_string(),
        //                 content: self.doc.get_resource_by_path(&v.0).unwrap(),
        //             }
    }

    // fn get_resource(&mut self, id: &str) -> Option<Vec<u8>> {
    //     let x = self
    //         .doc
    //         .resources
    //         .iter()
    //         .find(|(_, v)| v.0.to_str().unwrap() == id)
    //         .unwrap()
    //         .1
    //          .0
    //         .clone();

    //     self.doc.get_resource_by_path(&x)
    // }

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub u32);

#[derive(Debug, Clone)]
pub struct ContentBlock {
    pub content: String,
    pub content_type: ContentType,
}

impl ContentBlock {
    pub fn new(content: String, content_type: ContentType) -> Self {
        Self {
            content,
            content_type,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    Text,
    Html,
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
