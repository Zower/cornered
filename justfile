set shell := ["nu.exe", "-c"]

default: gen lint

gen:
    flutter pub get
    flutter_rust_bridge_codegen \
        --rust-input "native/src/books.rs" "native/src/util.rs" \
        --dart-output "lib/gen/books_generated.dart" "lib/gen/util_generated.dart" \
        --class-name Books Util \
        --rust-output native/src/books_generated.rs native/src/util_generated.rs
    cp ios/Runner/bridge_generated.h macos/Runner/bridge_generated.h

lint:
    cd native; cargo fmt
    dart format .

clean:
    flutter clean
    cd native && cargo clean
    
serve *args='':
    flutter pub run flutter_rust_bridge:serve {{args}}

# vim:expandtab:sw=4:ts=4
