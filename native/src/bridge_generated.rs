#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.65.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::types::ContentBlock;
use crate::types::ContentType;
use crate::types::Definition;
use crate::types::Definitions;
use crate::types::DocumentId;
use crate::types::Meaning;
use crate::types::Position;

// Section: wire functions

fn wire_open_doc_impl(
    port_: MessagePort,
    path: impl Wire2Api<String> + UnwindSafe,
    initial_chapter: impl Wire2Api<usize> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "open_doc",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            let api_initial_chapter = initial_chapter.wire2api();
            move |task_callback| open_doc(api_path, api_initial_chapter)
        },
    )
}
fn wire_go_next_impl(port_: MessagePort, id: impl Wire2Api<DocumentId> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "go_next",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_id = id.wire2api();
            move |task_callback| go_next(api_id)
        },
    )
}
fn wire_go_prev_impl(port_: MessagePort, id: impl Wire2Api<DocumentId> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "go_prev",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_id = id.wire2api();
            move |task_callback| go_prev(api_id)
        },
    )
}
fn wire_get_content_impl(port_: MessagePort, id: impl Wire2Api<DocumentId> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_content",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_id = id.wire2api();
            move |task_callback| get_content(api_id)
        },
    )
}
fn wire_get_resource_impl(
    port_: MessagePort,
    id: impl Wire2Api<DocumentId> + UnwindSafe,
    path: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_resource",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_id = id.wire2api();
            let api_path = path.wire2api();
            move |task_callback| get_resource(api_id, api_path)
        },
    )
}
fn wire_auth_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "auth",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(auth()),
    )
}
fn wire_poll_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "poll",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(poll()),
    )
}
fn wire_sync2_impl(port_: MessagePort, path: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "sync2",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            move |task_callback| Ok(sync2(api_path))
        },
    )
}
fn wire_init_db_impl(port_: MessagePort, path: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init_db",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            move |task_callback| init_db(api_path)
        },
    )
}
fn wire_get_meta_impl(port_: MessagePort, id: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_meta",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_id = id.wire2api();
            move |task_callback| get_meta(api_id)
        },
    )
}
fn wire_clear_db_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "clear_db",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| clear_db(),
    )
}
fn wire_get_definition_impl(port_: MessagePort, word: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_definition",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_word = word.wire2api();
            move |task_callback| get_definition(api_word)
        },
    )
}
fn wire_add_book__method__Database_impl(
    port_: MessagePort,
    that: impl Wire2Api<Database> + UnwindSafe,
    path: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "add_book__method__Database",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_path = path.wire2api();
            move |task_callback| Database::add_book(&api_that, api_path)
        },
    )
}
fn wire_update_progress__method__Database_impl(
    port_: MessagePort,
    that: impl Wire2Api<Database> + UnwindSafe,
    id: impl Wire2Api<String> + UnwindSafe,
    chapter: impl Wire2Api<usize> + UnwindSafe,
    offset: impl Wire2Api<f64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "update_progress__method__Database",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_id = id.wire2api();
            let api_chapter = chapter.wire2api();
            let api_offset = offset.wire2api();
            move |task_callback| {
                Database::update_progress(&api_that, api_id, api_chapter, api_offset)
            }
        },
    )
}
fn wire_get_books__method__Database_impl(
    port_: MessagePort,
    that: impl Wire2Api<Database> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_books__method__Database",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Database::get_books(&api_that)
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}
impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<usize> for usize {
    fn wire2api(self) -> usize {
        self
    }
}
// Section: impl IntoDart

impl support::IntoDart for Book {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.uuid.into_dart(),
            self.path.into_dart(),
            self.position.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Book {}

impl support::IntoDart for ContentBlock {
    fn into_dart(self) -> support::DartAbi {
        vec![self.content.into_dart(), self.content_type.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ContentBlock {}

impl support::IntoDart for ContentType {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Text => 0,
            Self::Html => 1,
        }
        .into_dart()
    }
}
impl support::IntoDart for Database {
    fn into_dart(self) -> support::DartAbi {
        Vec::<u8>::new().into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Database {}

impl support::IntoDart for Definition {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.definition.into_dart(),
            self.example.into_dart(),
            self.synonyms.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Definition {}

impl support::IntoDart for Definitions {
    fn into_dart(self) -> support::DartAbi {
        vec![self.word.into_dart(), self.meanings.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Definitions {}

impl support::IntoDart for DocumentId {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for DocumentId {}

impl support::IntoDart for Meaning {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.part_of_speech.into_dart(),
            self.definitions.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Meaning {}

impl support::IntoDart for Meta {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.title.into_dart(),
            self.author.into_dart(),
            self.cover.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Meta {}

impl support::IntoDart for Position {
    fn into_dart(self) -> support::DartAbi {
        vec![self.chapter.into_dart(), self.offset.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Position {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
