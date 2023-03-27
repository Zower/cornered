// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use std::{collections::HashMap, time::Duration};

static SQLX: OnceCell<Mutex<Connection>> = OnceCell::new();
// static ACCESS_TOKEN: OnceCell<Mutex<String>> = OnceCell::new();
// static EPUB: OnceCell<Mutex<EpubDoc<BufReader<File>>>> = OnceCell::new();
// static ONGOING: OnceCell<Mutex<DeviceCode>> = OnceCell::new();
static DOCUMENTS: OnceCell<Mutex<HashMap<DocumentId, Document>>> = OnceCell::new();
static DOCUMENT_COUNT: OnceCell<Mutex<u32>> = OnceCell::new();

// lazy_static! {
//     // TODO multiple
//     static ref EPUB: Mutex<Option<EpubDoc<BufReader<File>>>> = Mutex::new(None);

//     static ref ONGOING: Mutex<Option<DeviceCode>> = Mutex::new(None);

//     static ref ACCESS_TOKEN : Mutex<Option<String>> = Mutex::new(None);
// }

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::{
    types::{ContentBlock, Definitions, Document, DocumentId, Position},
    util::{open_document, ResponseOkStatus},
};

pub fn open_doc(path: String, initial_chapter: usize) -> anyhow::Result<DocumentId> {
    let mut doc = open_document(path)?;
    let mut count = DOCUMENT_COUNT.get_or_init(|| Mutex::new(0)).lock();

    *count += 1;

    let id = DocumentId(*count);

    let mut docs = DOCUMENTS.get_or_init(|| Mutex::new(HashMap::new())).lock();

    doc.inner.go_to(initial_chapter);

    docs.insert(id, doc);

    Ok(id)
}

pub fn go_next(id: DocumentId) -> anyhow::Result<ContentBlock> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    doc.inner.go_next().ok_or(anyhow::anyhow!("No next page"))
}

pub fn go_prev(id: DocumentId) -> anyhow::Result<ContentBlock> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    doc.inner
        .go_prev()
        .ok_or(anyhow::anyhow!("No previous page"))
}

pub fn get_content(id: DocumentId) -> anyhow::Result<ContentBlock> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    doc.inner
        .get_current()
        .ok_or(anyhow::anyhow!("No current content"))
}

// pub fn get_resource(id: DocumentId, resource_id: String) -> anyhow::Result<Vec<u8>> {
//     let mut docs = DOCUMENTS.get().unwrap().lock();

//     let doc = docs
//         .get_mut(&id)
//         .ok_or(anyhow!("No such document: {id:?}"))?;

//     doc.inner
//         .get_resource(&resource_id)
//         .ok_or(anyhow::anyhow!("No such content"))
// }

// pub fn get_spine(id: DocumentId) -> anyhow::Result<Vec<T>> {
//     let mut docs = DOCUMENTS.get().unwrap().lock();

//     let doc = docs
//         .get_mut(&id)
//         .ok_or(anyhow!("No such document: {id:?}"))?;

//     Ok(doc
//         .inner
//         .get_spine()
//         .iter()
//         .map(|x| T {
//             key: x.0.clone(),
//             value: x.1 .0.clone(),
//             mime: x.1 .1.clone(),
//         })
//         .collect())
// }

pub fn get_resources(id: DocumentId) -> anyhow::Result<Vec<T>> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    Ok(doc.inner.get_resources(""))
}

pub struct T {
    pub path: String,
    pub content: Vec<u8>,
}

pub fn auth() -> String {
    let response = ureq::post("https://github.com/login/device/code")
        .query("client_id", "bc2ede3adf378ac47e57")
        .query("scope", "workflow,repo")
        .set("accept", "application/json")
        .call()
        .unwrap()
        .into_json::<DeviceCode>()
        .unwrap();

    let code = response.user_code.clone();

    // ONGOING.set(Mutex::new(response)).unwrap();

    code
}

pub fn poll() {
    // let ongoing = { ONGOING.get().unwrap().lock().clone() };

    // let mut response_final: Option<CodeResponse> = None;

    // while None == response_final {
    //     std::thread::sleep(Duration::from_secs(ongoing.interval));
    //     let response = ureq::post("https://github.com/login/oauth/access_token")
    //         .query("client_id", "bc2ede3adf378ac47e57")
    //         .query("device_code", &ongoing.device_code)
    //         .query("grant_type", "urn:ietf:params:oauth:grant-type:device_code")
    //         .set("accept", "application/json")
    //         .call()
    //         .unwrap()
    //         .into_json::<CodeResponse>();

    //     if let Ok(response) = response {
    //         // *ONGOING.() = None;
    //         response_final = Some(response);
    //     }
    // }

    // ACCESS_TOKEN.set(Mutex::new(response_final.unwrap().access_token));

    // let response = ureq::get("https://api.github.com/repos/zower/cornered/contents/README.md")
    //     .set(
    //         "Authorization",
    //         format!("Bearer {}", ACCESS_TOKEN.lock().as_ref().unwrap()).as_str(),
    //     )
    //     .set("accept", "application/vnd.github+json")
    //     .call()
    //     .unwrap()
    //     .into_json::<FileResponse>()
    //     .unwrap();

    // println!("a {:?}", response);
}

pub fn sync2(path: String) {
    // #[allow(deprecated)]
    // let x = base64::encode(std::fs::read(path).unwrap());
    // let response = ureq::put("https://api.github.com/repos/zower/cornered/contents/book.epub")
    //     .set(
    //         "Authorization",
    //         format!("Bearer {}", ACCESS_TOKEN.lock().as_ref().unwrap()).as_str(),
    //     )
    //     .set("accept", "application/vnd.github+json")
    //     .send_json(ureq::json!({
    //         "message": "test",
    //         "content": x
    //     }))
    //     .unwrap()
    //     .into_string()
    //     .unwrap();

    // println!("b {:?}", response);
}

pub fn init_db(path: String) -> anyhow::Result<Database> {
    let x = SQLX.get_or_try_init(
        || -> anyhow::Result<parking_lot::lock_api::Mutex<parking_lot::RawMutex, Connection>> {
            Ok(Mutex::new(Connection::open(format!(
                "{path}/cornered.db3"
            ))?))
        },
    )?;

    let stmt = x
        .try_lock_for(Duration::from_secs(1))
        .ok_or(anyhow!("Failed to lock DB"))?;

    stmt.prepare(
        "CREATE TABLE IF NOT EXISTS books (
            uuid TEXT PRIMARY KEY,
            path TEXT NOT NULL,
            chapter INTEGER NOT NULL,
            offset REAL NOT NULL
        )",
    )?
    .execute(())?;

    // stmt.prepare(
    //     "CREATE TABLE IF NOT EXISTS progress (
    //         book TEXT REFERENCES books(uuid) ON DELETE CASCADE,
    //     )",
    // )?
    // .execute(())?;

    Ok(Database {})
}

/// Returns the metadata that might be useful in a "bookshelf" view
pub fn get_meta(id: String) -> anyhow::Result<Meta> {
    let stmt = SQLX.get().unwrap().lock();

    let mut stmt = stmt.prepare("SELECT path FROM books WHERE uuid = ?1")?;

    let mut rows = stmt.query(&[&id])?;

    let row = rows.next()?.ok_or(anyhow!("No such book"))?;

    let path: String = row.get(0)?;

    Ok(open_document(path)?.inner.meta())
}

pub fn clear_db() -> anyhow::Result<()> {
    let stmt = SQLX.get().unwrap().lock();

    stmt.execute("DELETE FROM books", [])?;

    Ok(())
}

pub fn get_definition(mut word: String) -> anyhow::Result<Definitions> {
    word.retain(|c| !r#"(),".;:'"#.contains(c));

    Ok(ureq::get(&format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        word.trim().to_lowercase()
    ))
    .call()?
    .ok_status()?
    .into_json::<Vec<Definitions>>()?
    .remove(0))
}

pub struct Database {}

impl Database {
    pub fn add_book(&self, path: String) -> anyhow::Result<Vec<Book>> {
        let id = uuid::Uuid::new_v4().to_string();

        {
            let stmt = SQLX.get().unwrap().lock();

            stmt.execute(
                "INSERT INTO books (uuid, path, chapter, offset) VALUES (?1, ?2, 0, 0.0)",
                (&id, &path),
            )?;
        }

        Ok(self.get_books()?)
    }

    pub fn update_progress(&self, id: String, chapter: usize, offset: f64) -> anyhow::Result<()> {
        let stmt = SQLX.get().unwrap().lock();

        stmt.execute(
            "UPDATE books SET chapter = ?1, offset = ?2 WHERE uuid = ?3",
            (&chapter, &offset, &id),
        )?;

        Ok(())
    }

    pub fn get_books(&self) -> anyhow::Result<Vec<Book>> {
        let stmt = SQLX.get().unwrap().lock();

        let mut stmt = stmt.prepare("SELECT uuid, path, chapter, offset FROM books")?;

        let books = stmt.query_map([], |row| {
            Ok(Book {
                uuid: row.get(0)?,
                path: row.get(1)?,
                position: Position {
                    chapter: row.get(2)?,
                    offset: row.get(3)?,
                },
            })
        })?;

        Ok(books.map(|x| x.unwrap()).collect())
    }
}

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

#[derive(Debug, Deserialize, Clone)]
struct DeviceCode {
    device_code: String,
    user_code: String,
    // verification_uri: String,
    // expires_in: u64,
    interval: u64,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct CodeResponse {
    access_token: String,
    scope: String,
    token_type: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct FileResponse {
    content: String,
}

// #[derive(Serialize, Debug)]
// struct CreateFile {
//     message: String,
//     content: String,
// }
