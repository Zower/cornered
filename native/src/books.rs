// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use std::{collections::HashMap, path::PathBuf};

pub static CONNECTION: OnceCell<Mutex<Connection>> = OnceCell::new();

static DOCUMENTS: OnceCell<Mutex<HashMap<OpenDocumentId, Document>>> = OnceCell::new();
static DOCUMENT_COUNT: OnceCell<Mutex<u64>> = OnceCell::new();

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use rusqlite::Connection;

use crate::{
    helpers::{open_document, ResponseOkStatus},
    types::{
        Book, ContentBlock, Definitions, Document, GoUrlResult, Meta, OpenDocumentId, Position,
        TocEntry, UploadedFile,
    },
};

pub fn open_doc(path: String, initial_chapter: usize) -> anyhow::Result<OpenDocumentId> {
    let mut doc = open_document(path)?;
    let mut count = DOCUMENT_COUNT.get_or_init(|| Mutex::new(0)).lock();

    *count += 1;

    let id = OpenDocumentId(*count);

    let mut docs = DOCUMENTS.get_or_init(|| Mutex::new(HashMap::new())).lock();

    doc.inner.go_to(initial_chapter);

    docs.insert(id, doc);

    Ok(id)
}

pub fn go_next(id: OpenDocumentId) -> anyhow::Result<ContentBlock> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    doc.inner.go_next().ok_or(anyhow::anyhow!("No next page"))
}

pub fn go_prev(id: OpenDocumentId) -> anyhow::Result<ContentBlock> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    doc.inner
        .go_prev()
        .ok_or(anyhow::anyhow!("No previous page"))
}

pub fn go_url(id: OpenDocumentId, url: String) -> anyhow::Result<GoUrlResult> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    doc.inner
        .go_url(&url)?
        .ok_or(anyhow::anyhow!("No such page"))
}

pub fn get_content(id: OpenDocumentId) -> anyhow::Result<ContentBlock> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    doc.inner
        .get_current()
        .ok_or(anyhow::anyhow!("No current content"))
}

pub fn get_resource(id: OpenDocumentId, path: String) -> anyhow::Result<Vec<u8>> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    doc.inner
        .get_resource(&path)
        .ok_or(anyhow::anyhow!("No such "))
}

pub fn get_toc(id: OpenDocumentId) -> anyhow::Result<Vec<TocEntry>> {
    let mut docs = DOCUMENTS.get().unwrap().lock();

    let doc = docs
        .get_mut(&id)
        .ok_or(anyhow!("No such document: {id:?}"))?;

    Ok(doc.inner.get_toc())
}

pub fn init_db(path: String) -> anyhow::Result<Database> {
    let connection = CONNECTION.get_or_try_init(
        || -> anyhow::Result<parking_lot::lock_api::Mutex<parking_lot::RawMutex, Connection>> {
            Ok(Mutex::new(Connection::open(format!(
                "{path}/cornered.db3"
            ))?))
        },
    )?.lock();

    connection
        .prepare(
            "CREATE TABLE IF NOT EXISTS books (
            uuid TEXT PRIMARY KEY,
            path TEXT NOT NULL,
            chapter INTEGER NOT NULL,
            offset REAL NOT NULL
        )",
        )?
        .execute(())?;

    connection
        .prepare(
            "CREATE TABLE IF NOT EXISTS tokens (
            github_id INTEGER PRIMARY KEY,
            token TEXT NOT NULL
        )",
        )?
        .execute(())?;

    Ok(Database {})
}

pub fn get_meta(id: String) -> anyhow::Result<Meta> {
    let connection = CONNECTION.get().unwrap().lock();

    let mut stmt = connection.prepare("SELECT path FROM books WHERE uuid = ?1")?;

    let mut rows = stmt.query(&[&id])?;

    let row = rows.next()?.ok_or(anyhow!("No such book"))?;

    let path: String = row.get(0)?;

    Ok(open_document(path)?.inner.meta())
}

pub fn clear_db() -> anyhow::Result<()> {
    let stmt = CONNECTION.get().unwrap().lock();

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
            let stmt = CONNECTION
                .get()
                .ok_or(anyhow!("Could not get connection"))?
                .lock();

            stmt.execute(
                "INSERT INTO books (uuid, path, chapter, offset) VALUES (?1, ?2, 0, 0.0)",
                (&id, &path),
            )?;

            Ok::<(), anyhow::Error>(())
        }?;

        Ok(self.get_books()?)
    }

    pub fn update_progress(&self, id: String, chapter: usize, offset: f64) -> anyhow::Result<()> {
        let stmt = CONNECTION
            .get()
            .ok_or(anyhow!("Could not get connection"))?
            .lock();

        stmt.execute(
            "UPDATE books SET chapter = ?1, offset = ?2 WHERE uuid = ?3",
            (&chapter, &offset, &id),
        )?;

        Ok(())
    }

    pub fn get_books(&self) -> anyhow::Result<Vec<Book>> {
        let stmt = CONNECTION
            .get()
            .ok_or(anyhow!("Could not get connection"))?
            .lock();

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

    pub fn get_book(&self, uuid: String) -> anyhow::Result<Book> {
        let stmt = CONNECTION
            .get()
            .ok_or(anyhow!("Could not get connection"))?
            .lock();

        let mut stmt =
            stmt.prepare("SELECT uuid, path, chapter, offset FROM books WHERE uuid = ?1")?;

        let mut books = stmt.query_map([&uuid], |row| {
            Ok(Book {
                uuid: row.get(0)?,
                path: row.get(1)?,
                position: Position {
                    chapter: row.get(2)?,
                    offset: row.get(3)?,
                },
            })
        })?;

        Ok(books.next().ok_or(anyhow!("No such book"))??)
    }

    pub(crate) fn add_synced_book(&self, file: UploadedFile, path: &PathBuf) -> anyhow::Result<()> {
        let stmt = CONNECTION
            .get()
            .ok_or(anyhow!("Could not get connection"))?
            .lock();

        stmt.execute(
            "INSERT INTO books (uuid, path, chapter, offset) VALUES (?1, ?2, ?3, ?4)",
            (
                &file.uuid,
                &path.to_str().expect("valid utf8"),
                &file.position.chapter,
                &file.position.offset,
            ),
        )?;

        Ok(())
    }
}
