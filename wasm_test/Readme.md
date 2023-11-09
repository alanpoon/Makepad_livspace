cargo build --target wasm32-wasi --release
cp target/wasm32-wasi/release/wasm_test.wasm ../wasm_test_server
simple-http-server wasm_test_server