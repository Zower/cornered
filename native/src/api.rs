// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    time::Duration,
};

use lazy_static::lazy_static;

lazy_static! {
    // TODO multiple
    static ref EPUB: Mutex<Option<EpubDoc<BufReader<File>>>> = Mutex::new(None);

    static ref ONGOING: Mutex<Option<DeviceCode>> = Mutex::new(None);

    static ref ACCESS_TOKEN : Mutex<Option<String>> = Mutex::new(None);
}

use epub::doc::EpubDoc;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

pub fn open_doc(path: String) {
    // let file = File::open(path.into()).unwrap();
    // let mut doc = EpubDoc::from_reader(BufReader::new(file))?;
    // doc.archive.path = path.to_path_buf();
    let doc = EpubDoc::new(path).unwrap();

    EPUB.lock().replace(doc);
}

pub fn go_next() {
    let mut doc = EPUB.lock();
    let doc = doc.as_mut().unwrap();
    doc.go_next();
}

pub fn go_prev() {
    let mut doc = EPUB.lock();
    let doc = doc.as_mut().unwrap();
    doc.go_prev();
}

pub fn get_content() -> String {
    let mut doc = EPUB.lock();
    let doc = doc.as_mut().unwrap();

    doc.get_current_str().unwrap().0
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

    *ONGOING.lock() = Some(response);

    code
}

pub fn poll() {
    let ongoing = { ONGOING.lock().as_ref().unwrap().clone() };

    let mut response_final: Option<CodeResponse> = None;

    while None == response_final {
        std::thread::sleep(Duration::from_secs(ongoing.interval));
        let response = ureq::post("https://github.com/login/oauth/access_token")
            .query("client_id", "bc2ede3adf378ac47e57")
            .query("device_code", &ongoing.device_code)
            .query("grant_type", "urn:ietf:params:oauth:grant-type:device_code")
            .set("accept", "application/json")
            .call()
            .unwrap()
            .into_json::<CodeResponse>();

        if let Ok(response) = response {
            *ONGOING.lock() = None;
            response_final = Some(response);
        }
    }

    *ACCESS_TOKEN.lock() = Some(response_final.unwrap().access_token);

    let response = ureq::get("https://api.github.com/repos/zower/cornered/contents/README.md")
        .set(
            "Authorization",
            format!("Bearer {}", ACCESS_TOKEN.lock().as_ref().unwrap()).as_str(),
        )
        .set("accept", "application/vnd.github+json")
        .call()
        .unwrap()
        .into_json::<FileResponse>()
        .unwrap();

    println!("a {:?}", response);
}

pub fn sync2(path: String) {
    #[allow(deprecated)]
    let x = base64::encode(std::fs::read(path).unwrap());
    let response = ureq::put("https://api.github.com/repos/zower/cornered/contents/book.epub")
        .set(
            "Authorization",
            format!("Bearer {}", ACCESS_TOKEN.lock().as_ref().unwrap()).as_str(),
        )
        .set("accept", "application/vnd.github+json")
        .send_json(ureq::json!({
            "message": "test",
            "content": x
        }))
        .unwrap()
        .into_string()
        .unwrap();

    println!("b {:?}", response);
}

fn get_file(path: &str) -> anyhow::Result<File> {
    let path = std::path::Path::new(&path);

    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;

    Ok(file)
}

pub fn init_db(path: String) -> anyhow::Result<Database> {
    Ok(Database {
        path: format!("{path}/books.cornered"),
    })
}

pub fn clear_db(path: String) -> anyhow::Result<()> {
    let mut file = get_file(&format!("{path}/books.cornered"))?;

    file.write_all(&bincode::serialize(&Vec::<Book>::new())?)?;

    Ok(())
}

pub struct Database {
    pub path: String,
}

impl Database {
    pub fn add(&self, path: String) -> anyhow::Result<Vec<Book>> {
        let id = uuid::Uuid::new_v4().to_string();

        let mut books = self.get_books()?;

        books.push(Book {
            uuid: id,
            path: path,
        });

        let mut file = get_file(&self.path)?;

        file.write_all(&bincode::serialize(&books)?)?;

        Ok(books)
    }

    pub fn get_books(&self) -> anyhow::Result<Vec<Book>> {
        let file = get_file(&self.path)?;

        let books: Vec<Book> = bincode::deserialize_from(file).unwrap_or_default();

        Ok(books)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub uuid: String,
    pub path: String,
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
