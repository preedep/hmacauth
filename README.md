# HMACAUTH

HMACAUTH is demo project for test HMAC Authentication , I've split the project into three parts:

- HMACAUTH-LIB : A library that contains the HMAC Authentication logic.
- HMACUTH-WASM : A WebAssembly project that uses the HMACAUTH-LIB to authenticate a request.
- HMACAUTH-WEB : A Web project that uses the HMACAUTH-LIB to authenticate a request.
- HMACAUTH-CLI : A CLI project that uses the HMACAUTH-LIB to authenticate a request.

## Getting Started

What is HMAC Authentication? HMAC Authentication is a way to authenticate a request using a shared secret key between
the client and the server. The client sends a request with a hash of the request body and the server verifies the hash
using the shared secret key.

![img1.png](img%2Fimg1.png)
figure 1: HMAC Authentication

## HMACAUTH-LIB

The HMACAUTH-LIB is a library that contains the HMAC Authentication logic. It provides a way to generate a hash of a
request body and verify the hash using a shared secret key.
in utils.rs you can find the implementation of the HMAC Authentication logic.

utils.rs contains the following functions:

```
- compute_content_sha256 (compute the SHA256 hash of the request body)
- generate_string_to_sign (generate the string to sign using the request method, path, query parameters, headers, and content hash)
- compute_signature (compute the HMAC-SHA256 hash between string_to_sign and secret_key)
```

## HMACAUTH-WEB

The HMACAUTH-WEB is a Simple Web project that uses the HMACAUTH-LIB to authenticate a request. It provides a api
endpoint for test HMAC Authentication. and support web hosting for HMACAUTH-WASM (Web assembly).

Example run HMACAUTH-WEB: (Just API endpoint)

```bash
RUST_LOG=debug ./target/debug/hmacauth_web
```

Example run HMACAUTH-WEB: (API endpoint and Static Web/Web Assembly hosting)

you can define the static folder path by setting the STATIC_FOLDER environment variable.

```bash
cd hmacauth_wasm
./build.sh
cd ..
STATIC_FOLDER=/Users/preedee/Project/Rust/hmacauth/hmacauth_web/static \ 
RUST_LOG=debug ./target/debug/hmacauth_web
```

After running HMACAUTH-WEB, you can access simple web at http://localhost:8080/

## HMACAUTH-WASM

The HMACAUTH-WASM is a Simple WebAssembly project that uses the HMACAUTH-LIB to authenticate a request. It provides a
way to authenticate a request using HMAC Authentication in the browser. This project is hosted on HMACAUTH-WEB *
*_but not yet implemented._**

## HMACAUTH-CLI

The HMACAUTH-CLI is a Simple CLI project that uses the HMACAUTH-LIB to authenticate a request. It provides a way to
authenticate a request using HMAC Authentication in the terminal.

Example run HMACAUTH-CLI: (after running HMACAUTH-WEB)

```bash
RUST_LOG=debug ./target/debug/hmacauth_cli --url "http://localhost:8080/apis/v1/payload" --request-id "request_id_123456" --message "Hello World" --access-key "IbNSH3Lc5ffMHo/wnQuiOD4C0mx5FqDmVMQaAMKFgaQ="
```

## Testing

You can run the test by running the following command: (root project)

```bash

./run_web.sh #(please setup the STATIC_FOLDER environment variable before running this command)

./run_cli.sh
```