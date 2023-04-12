use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_get_users(port_: i64) {
    wire_get_users_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_primary_user(port_: i64) {
    wire_get_primary_user_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_auth(port_: i64) {
    wire_auth_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_poll(port_: i64, ongoing: *mut wire_DeviceFlowResponse) {
    wire_poll_impl(port_, ongoing)
}

#[no_mangle]
pub extern "C" fn wire_upload_file(
    port_: i64,
    repo: *mut wire_uint_8_list,
    uuid: *mut wire_uint_8_list,
    user: *mut wire_GithubUserJson,
) {
    wire_upload_file_impl(port_, repo, uuid, user)
}

#[no_mangle]
pub extern "C" fn wire_update_files(
    port_: i64,
    repo: *mut wire_uint_8_list,
    user: *mut wire_GithubUserJson,
) {
    wire_update_files_impl(port_, repo, user)
}

#[no_mangle]
pub extern "C" fn wire_font_search(port_: i64, query: *mut wire_uint_8_list) {
    wire_font_search_impl(port_, query)
}

#[no_mangle]
pub extern "C" fn wire_get_definition(port_: i64, word: *mut wire_uint_8_list) {
    wire_get_definition_impl(port_, word)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_device_flow_response_1() -> *mut wire_DeviceFlowResponse {
    support::new_leak_box_ptr(wire_DeviceFlowResponse::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_github_user_json_1() -> *mut wire_GithubUserJson {
    support::new_leak_box_ptr(wire_GithubUserJson::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_1(len: i32) -> *mut wire_uint_8_list {
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
impl Wire2Api<DeviceFlowResponse> for *mut wire_DeviceFlowResponse {
    fn wire2api(self) -> DeviceFlowResponse {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<DeviceFlowResponse>::wire2api(*wrap).into()
    }
}
impl Wire2Api<GithubUserJson> for *mut wire_GithubUserJson {
    fn wire2api(self) -> GithubUserJson {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<GithubUserJson>::wire2api(*wrap).into()
    }
}
impl Wire2Api<DeviceFlowResponse> for wire_DeviceFlowResponse {
    fn wire2api(self) -> DeviceFlowResponse {
        DeviceFlowResponse {
            device_code: self.device_code.wire2api(),
            user_code: self.user_code.wire2api(),
            verification_uri: self.verification_uri.wire2api(),
            interval: self.interval.wire2api(),
        }
    }
}
impl Wire2Api<GithubUserJson> for wire_GithubUserJson {
    fn wire2api(self) -> GithubUserJson {
        GithubUserJson {
            display_name: self.display_name.wire2api(),
            id: self.id.wire2api(),
        }
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
pub struct wire_DeviceFlowResponse {
    device_code: *mut wire_uint_8_list,
    user_code: *mut wire_uint_8_list,
    verification_uri: *mut wire_uint_8_list,
    interval: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_GithubUserJson {
    display_name: *mut wire_uint_8_list,
    id: u64,
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

impl NewWithNullPtr for wire_DeviceFlowResponse {
    fn new_with_null_ptr() -> Self {
        Self {
            device_code: core::ptr::null_mut(),
            user_code: core::ptr::null_mut(),
            verification_uri: core::ptr::null_mut(),
            interval: Default::default(),
        }
    }
}

impl Default for wire_DeviceFlowResponse {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_GithubUserJson {
    fn new_with_null_ptr() -> Self {
        Self {
            display_name: core::ptr::null_mut(),
            id: Default::default(),
        }
    }
}

impl Default for wire_GithubUserJson {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
