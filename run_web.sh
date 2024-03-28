cd hmacauth_wasm
./build.sh
cd ..

#cp target/wasm32-unknown-unknown/debug/hmacauth_wasm.wasm hmacauth_web/static/pkg/

STATIC_FOLDER=/Users/preedee/Project/Rust/hmacauth/hmacauth_web/static RUST_LOG=debug ./target/debug/hmacauth_web

