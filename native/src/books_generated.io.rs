use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_init_app(port_: i64, data_dir: *mut wire_uint_8_list) {
    wire_init_app_impl(port_, data_dir)
}

#[no_mangle]
pub extern "C" fn wire_open_doc(
    port_: i64,
    path: *mut wire_uint_8_list,
    initial_chapter: *mut usize,
) {
    wire_open_doc_impl(port_, path, initial_chapter)
}

#[no_mangle]
pub extern "C" fn wire_get_db(port_: i64) {
    wire_get_db_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_clear_db(port_: i64) {
    wire_clear_db_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_meta(port_: i64, id: *mut wire_uint_8_list) {
    wire_get_meta_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_go_next__method__OpenDocument(port_: i64, that: *mut wire_OpenDocument) {
    wire_go_next__method__OpenDocument_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_go_prev__method__OpenDocument(port_: i64, that: *mut wire_OpenDocument) {
    wire_go_prev__method__OpenDocument_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_go_url__method__OpenDocument(
    port_: i64,
    that: *mut wire_OpenDocument,
    url: *mut wire_uint_8_list,
) {
    wire_go_url__method__OpenDocument_impl(port_, that, url)
}

#[no_mangle]
pub extern "C" fn wire_get_content__method__OpenDocument(port_: i64, that: *mut wire_OpenDocument) {
    wire_get_content__method__OpenDocument_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_get_resource__method__OpenDocument(
    port_: i64,
    that: *mut wire_OpenDocument,
    path: *mut wire_uint_8_list,
) {
    wire_get_resource__method__OpenDocument_impl(port_, that, path)
}

#[no_mangle]
pub extern "C" fn wire_get_toc__method__OpenDocument(port_: i64, that: *mut wire_OpenDocument) {
    wire_get_toc__method__OpenDocument_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_add_book__method__Database(
    port_: i64,
    that: *mut wire_Database,
    path: *mut wire_uint_8_list,
) {
    wire_add_book__method__Database_impl(port_, that, path)
}

#[no_mangle]
pub extern "C" fn wire_update_progress__method__Database(
    port_: i64,
    that: *mut wire_Database,
    id: *mut wire_uint_8_list,
    chapter: usize,
    offset: f64,
) {
    wire_update_progress__method__Database_impl(port_, that, id, chapter, offset)
}

#[no_mangle]
pub extern "C" fn wire_get_books__method__Database(port_: i64, that: *mut wire_Database) {
    wire_get_books__method__Database_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_get_book__method__Database(
    port_: i64,
    that: *mut wire_Database,
    uuid: *mut wire_uint_8_list,
) {
    wire_get_book__method__Database_impl(port_, that, uuid)
}

#[no_mangle]
pub extern "C" fn wire_delete_books__method__Database(
    port_: i64,
    that: *mut wire_Database,
    uuids: *mut wire_StringList,
) {
    wire_delete_books__method__Database_impl(port_, that, uuids)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_database_0() -> *mut wire_Database {
    support::new_leak_box_ptr(wire_Database::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_open_document_0() -> *mut wire_OpenDocument {
    support::new_leak_box_ptr(wire_OpenDocument::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_usize_0(value: usize) -> *mut usize {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Database> for *mut wire_Database {
    fn wire2api(self) -> Database {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Database>::wire2api(*wrap).into()
    }
}
impl Wire2Api<OpenDocument> for *mut wire_OpenDocument {
    fn wire2api(self) -> OpenDocument {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OpenDocument>::wire2api(*wrap).into()
    }
}
impl Wire2Api<usize> for *mut usize {
    fn wire2api(self) -> usize {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Database> for wire_Database {
    fn wire2api(self) -> Database {
        Database {}
    }
}

impl Wire2Api<OpenDocument> for wire_OpenDocument {
    fn wire2api(self) -> OpenDocument {
        OpenDocument {
            id: self.id.wire2api(),
        }
    }
}
impl Wire2Api<OpenDocumentId> for wire_OpenDocumentId {
    fn wire2api(self) -> OpenDocumentId {
        OpenDocumentId(self.field0.wire2api())
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Database {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OpenDocument {
    id: wire_OpenDocumentId,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OpenDocumentId {
    field0: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Database {
    fn new_with_null_ptr() -> Self {
        Self {}
    }
}

impl Default for wire_Database {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_OpenDocument {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
        }
    }
}

impl Default for wire_OpenDocument {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_OpenDocumentId {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}

impl Default for wire_OpenDocumentId {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
