use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_open_doc(port_: i64, path: *mut wire_uint_8_list, initial_chapter: usize) {
    wire_open_doc_impl(port_, path, initial_chapter)
}

#[no_mangle]
pub extern "C" fn wire_go_next(port_: i64, id: *mut wire_DocumentId) {
    wire_go_next_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_go_prev(port_: i64, id: *mut wire_DocumentId) {
    wire_go_prev_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_get_content(port_: i64, id: *mut wire_DocumentId) {
    wire_get_content_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_get_resources(port_: i64, id: *mut wire_DocumentId) {
    wire_get_resources_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_auth(port_: i64) {
    wire_auth_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_poll(port_: i64) {
    wire_poll_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_sync2(port_: i64, path: *mut wire_uint_8_list) {
    wire_sync2_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_init_db(port_: i64, path: *mut wire_uint_8_list) {
    wire_init_db_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_get_meta(port_: i64, id: *mut wire_uint_8_list) {
    wire_get_meta_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_clear_db(port_: i64) {
    wire_clear_db_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_definition(port_: i64, word: *mut wire_uint_8_list) {
    wire_get_definition_impl(port_, word)
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

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_database_0() -> *mut wire_Database {
    support::new_leak_box_ptr(wire_Database::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_document_id_0() -> *mut wire_DocumentId {
    support::new_leak_box_ptr(wire_DocumentId::new_with_null_ptr())
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
impl Wire2Api<Database> for *mut wire_Database {
    fn wire2api(self) -> Database {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Database>::wire2api(*wrap).into()
    }
}
impl Wire2Api<DocumentId> for *mut wire_DocumentId {
    fn wire2api(self) -> DocumentId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<DocumentId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Database> for wire_Database {
    fn wire2api(self) -> Database {
        Database {}
    }
}
impl Wire2Api<DocumentId> for wire_DocumentId {
    fn wire2api(self) -> DocumentId {
        DocumentId(self.field0.wire2api())
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
pub struct wire_Database {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DocumentId {
    field0: u32,
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

impl NewWithNullPtr for wire_DocumentId {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
