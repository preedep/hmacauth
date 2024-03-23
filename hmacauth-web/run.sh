cd ..
cd hmacauth-wasm
./build.sh
cd ..
cd hmacauth-web
RUST_LOG=debug cargo run
