# HMACAUTH

HMACAUTH is demo project for test HMAC Authentication , I've split the project into three parts:

- HMACAUTH-LIB : A library that contains the HMAC Authentication logic.
- HMACUTH-WASM : A WebAssembly project that uses the HMACAUTH-LIB to authenticate a request.
- HMACAUTH-WEB : A Web project that uses the HMACAUTH-LIB to authenticate a request.
- HMACAUTH-CLI : A CLI project that uses the HMACAUTH-LIB to authenticate a request.


## Getting Started
What is HMAC Authentication? HMAC Authentication is a way to authenticate a request using a shared secret key between the client and the server. The client sends a request with a hash of the request body and the server verifies the hash using the shared secret key.

![img1.png](img%2Fimg1.png)
figure 1: HMAC Authentication

