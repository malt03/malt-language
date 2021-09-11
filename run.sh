set -eu

cargo run -- ./example/main.mlt
wat2wasm ./out/main.wat -o ./out/main.wasm
wasmer ./out/main.wasm
