wasm-pack build --target web

echo "Copying files to hmacauth-web"

cp -r pkg ../hmacauth-web/static

echo "Done"
