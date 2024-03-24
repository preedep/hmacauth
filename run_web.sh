cd hmacauth_wasm
./build.sh
cd ..
STATIC_FOLDER=/Users/preedee/Project/Rust/hmacauth/hmacauth_web/static RUST_LOG=debug ./target/debug/hmacauth_web

