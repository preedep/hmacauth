cd ..
cd hmacauth_wasm
./build.sh
cd ..
cd hmacauth_web
RUST_LOG=debug cargo run
