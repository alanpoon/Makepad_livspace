# makepad-example-with wasmedge

A guide to compile android apk with wasmedge.

- wget https://github.com/WasmEdge/WasmEdge/releases/download/0.13.5/WasmEdge-0.13.5-android_aarch64.tar.gz
- untar WasmEdge-0.13.5-android_aarch64.tar.gz
- git clone https://github.com/alanpoon/makepad
- cargo install --path=./tools/cargo_makepad
- RUST_BACKTRACE=1 WASMEDGE_DIR=/Users/alanpoon/Documents/rust/wasmedge/WasmEdge-0.13.5-Android cargo makepad android run -p makepad-livspace

# why wasmedge sdk?

In the future we can try AI inference model using wasmedge.