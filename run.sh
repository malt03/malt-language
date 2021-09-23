set -eu

cargo run -- ./example/main.mlt ./out/main.wat
wat2wasm ./out/main.wat -o ./out/main.wasm
wasmer ./out/main.wasm
