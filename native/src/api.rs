// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use std::{fs::File, io::BufReader};

use lazy_static::lazy_static;

lazy_static! {
    // TODO multiple
    static ref EPUB: Mutex<Option<EpubDoc<BufReader<File>>>> = Mutex::new(None);
}

use epub::doc::EpubDoc;
use parking_lot::Mutex;

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

pub fn get_content() -> String {
    let mut doc = EPUB.lock();
    let doc = doc.as_mut().unwrap();

    doc.get_current_str().unwrap().0
}
