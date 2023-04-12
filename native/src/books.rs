// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use std::{collections::HashMap, path::PathBuf};

pub static POOL: OnceCell<Pool<SqliteConnectionManager>> = OnceCell::new();
pub static DATA_DIR: OnceCell<PathBuf> = OnceCell::new();

static OPEN_DOCS: OnceCell<Mutex<HashMap<OpenDocumentId, Document>>> = OnceCell::new();
static DOCUMENT_ID: OnceCell<Mutex<u64>> = OnceCell::new();

#[cfg(debug_assertions)]
static IS_INITIALIZED: OnceCell<()> = OnceCell::new();

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::{
    helpers::open_document,
    types::{
        Book, ContentBlock, Document, GoUrlResult, Meta, OpenDocumentId, Position, TocEntry,
        UploadedFile,
    },
};

pub fn init_app(data_dir: String) -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    fn check_init() -> anyhow::Result<()> {
        if let Some(_) = IS_INITIALIZED.get() {
            return Err(anyhow!("App already initialized, ignoring in debug mode"));
        }

        IS_INITIALIZED.set(()).unwrap();

        Ok(())
    }

    #[cfg(debug_assertions)]
    check_init()?;

    let manager =
        SqliteConnectionManager::file(format!("{data_dir}/cornered.db3", data_dir = data_dir));

    let pool = r2d2::Pool::new(manager)?;

    let connection = pool.clone();

    DATA_DIR.set(data_dir.clone().into()).unwrap();

    POOL.set(pool).unwrap();

    OPEN_DOCS.set(Mutex::new(HashMap::new())).unwrap();

    std::thread::spawn(move || -> anyhow::Result<()> {
        let connection = connection.get()?;

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
                    display_name TEXT NOT NULL,
                    token TEXT NOT NULL
                )",
            )?
            .execute(())?;

        connection
            .prepare(
                "CREATE TABLE IF NOT EXISTS current_token (
                    Lock char(1) not null DEFAULT 'X',
                    github_id INTEGER,
                    FOREIGN KEY(github_id) REFERENCES tokens(github_id),
                    constraint PK_T1 PRIMARY KEY (Lock),
                    constraint CK_T1_Locked CHECK (Lock='X')
                )
            ",
            )?
            .execute(())?;

        Ok(())
    });

    Ok(())
}

pub fn open_doc(path: String, initial_chapter: Option<usize>) -> anyhow::Result<OpenDocument> {
    let mut doc = open_document(path)?;
    let mut count = DOCUMENT_ID.get_or_init(|| Mutex::new(0)).lock();

    *count += 1;

    let id = OpenDocumentId(*count);

    let mut docs = OPEN_DOCS.get_or_init(|| Mutex::new(HashMap::new())).lock();

    doc.inner.go_to(initial_chapter.unwrap_or(0));

    docs.insert(id, doc);

    Ok(OpenDocument { id })
}

pub fn get_db() -> Database {
    Database {}
}

pub fn clear_db() -> anyhow::Result<()> {
    let stmt = POOL.get().unwrap().get()?;

    stmt.execute("DELETE FROM books", [])?;

    Ok(())
}

// TODO
pub fn get_meta(id: String) -> anyhow::Result<Meta> {
    let connection = POOL.get().unwrap().get()?;

    let mut stmt = connection.prepare("SELECT path FROM books WHERE uuid = ?1")?;

    let mut rows = stmt.query(&[&id])?;

    let row = rows.next()?.ok_or(anyhow!("No such book"))?;

    let path: String = row.get(0)?;

    Ok(open_document(path)?.inner.meta())
}

pub struct OpenDocument {
    pub id: OpenDocumentId,
}

impl OpenDocument {
    pub fn go_next(&self) -> anyhow::Result<ContentBlock> {
        let mut docs = OPEN_DOCS.get().unwrap().lock();
        let doc = self.get_doc(&mut docs)?;

        doc.inner.go_next().ok_or(anyhow::anyhow!("No next page"))
    }

    pub fn go_prev(&self) -> anyhow::Result<ContentBlock> {
        let mut docs = OPEN_DOCS.get().unwrap().lock();
        let doc = self.get_doc(&mut docs)?;

        doc.inner
            .go_prev()
            .ok_or(anyhow::anyhow!("No previous page"))
    }

    pub fn go_url(&self, url: String) -> anyhow::Result<GoUrlResult> {
        let mut docs = OPEN_DOCS.get().unwrap().lock();
        let doc = self.get_doc(&mut docs)?;

        doc.inner
            .go_url(&url)?
            .ok_or(anyhow::anyhow!("No such page"))
    }

    pub fn get_content(&self) -> anyhow::Result<ContentBlock> {
        let mut docs = OPEN_DOCS.get().unwrap().lock();
        let doc = self.get_doc(&mut docs)?;

        doc.inner
            .get_current()
            .ok_or(anyhow::anyhow!("No current content"))
    }

    pub fn get_resource(&self, path: String) -> anyhow::Result<Vec<u8>> {
        let mut docs = OPEN_DOCS.get().unwrap().lock();
        let doc = self.get_doc(&mut docs)?;

        doc.inner
            .get_resource(&path)
            .ok_or(anyhow::anyhow!("No such resource"))
    }

    pub fn get_toc(&self) -> anyhow::Result<Vec<TocEntry>> {
        let mut docs = OPEN_DOCS.get().unwrap().lock();
        let doc = self.get_doc(&mut docs)?;

        Ok(doc.inner.get_toc())
    }

    fn get_doc<'a>(
        &'a self,
        docs: &'a mut HashMap<OpenDocumentId, Document>,
    ) -> anyhow::Result<&'a mut Document> {
        docs.get_mut(&self.id)
            .ok_or(anyhow!("Document has been deleted"))
    }
}

pub struct Database {}

impl Database {
    pub fn add_book(&self, path: String) -> anyhow::Result<Vec<Book>> {
        let id = uuid::Uuid::new_v4().to_string();

        {
            let stmt = POOL
                .get()
                .ok_or(anyhow!("Could not get connection"))?
                .get()?;

            stmt.execute(
                "INSERT INTO books (uuid, path, chapter, offset) VALUES (?1, ?2, 0, 0.0)",
                (&id, &path),
            )?;

            Ok::<(), anyhow::Error>(())
        }?;

        Ok(self.get_books()?)
    }

    pub fn update_progress(&self, id: String, chapter: usize, offset: f64) -> anyhow::Result<()> {
        let stmt = POOL
            .get()
            .ok_or(anyhow!("Could not get connection"))?
            .get()?;

        stmt.execute(
            "UPDATE books SET chapter = ?1, offset = ?2 WHERE uuid = ?3",
            (&chapter, &offset, &id),
        )?;

        Ok(())
    }

    pub fn get_books(&self) -> anyhow::Result<Vec<Book>> {
        let stmt = POOL
            .get()
            .ok_or(anyhow!("Could not get connection"))?
            .get()?;

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
        let stmt = POOL
            .get()
            .ok_or(anyhow!("Could not get connection"))?
            .get()?;

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

    // Note: Does not delete the file.
    pub fn delete_books(&self, uuids: Vec<String>) -> anyhow::Result<()> {
        let connection = POOL
            .get()
            .ok_or(anyhow!("Could not get connection"))?
            .get()?;

        connection.execute("BEGIN TRANSACTION", [])?;

        let mut prepared = connection.prepare("DELETE FROM books WHERE uuid = ?1")?;

        for uuid in uuids {
            prepared.execute([&uuid])?;
        }

        connection.execute("COMMIT TRANSACTION", [])?;

        Ok(())
    }

    pub(crate) fn add_synced_book(&self, file: UploadedFile, path: &PathBuf) -> anyhow::Result<()> {
        let stmt = POOL
            .get()
            .ok_or(anyhow!("Could not get connection"))?
            .get()?;

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
