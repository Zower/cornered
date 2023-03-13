#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_Database {
  struct wire_uint_8_list *path;
} wire_Database;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_open_doc(int64_t port_, struct wire_uint_8_list *path);

void wire_go_next(int64_t port_);

void wire_go_prev(int64_t port_);

void wire_get_content(int64_t port_);

void wire_auth(int64_t port_);

void wire_poll(int64_t port_);

void wire_sync2(int64_t port_, struct wire_uint_8_list *path);

void wire_init_db(int64_t port_, struct wire_uint_8_list *path);

void wire_clear_db(int64_t port_, struct wire_uint_8_list *path);

void wire_add__method__Database(int64_t port_,
                                struct wire_Database *that,
                                struct wire_uint_8_list *path);

void wire_get_books__method__Database(int64_t port_, struct wire_Database *that);

struct wire_Database *new_box_autoadd_database_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_open_doc);
    dummy_var ^= ((int64_t) (void*) wire_go_next);
    dummy_var ^= ((int64_t) (void*) wire_go_prev);
    dummy_var ^= ((int64_t) (void*) wire_get_content);
    dummy_var ^= ((int64_t) (void*) wire_auth);
    dummy_var ^= ((int64_t) (void*) wire_poll);
    dummy_var ^= ((int64_t) (void*) wire_sync2);
    dummy_var ^= ((int64_t) (void*) wire_init_db);
    dummy_var ^= ((int64_t) (void*) wire_clear_db);
    dummy_var ^= ((int64_t) (void*) wire_add__method__Database);
    dummy_var ^= ((int64_t) (void*) wire_get_books__method__Database);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_database_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}