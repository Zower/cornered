set shell := ["nu.exe", "-c"]

default: gen lint

gen:
    flutter pub get
    flutter_rust_bridge_codegen \
        --rust-input "native/src/api.rs" "native/src/test.rs" \
        --dart-output "lib/gen/bridge_generated_1.dart" "lib/gen/bridge_generated_2.dart" \
        --class-name Api Test \
        --rust-output native/src/bridge_generated_1.rs native/src/bridge_generated_2.rs
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
