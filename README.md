# watmachine

Project purpose: Write a network-based node that executes web-assembly sent between nodes.

## Milestone 0 (Completed)

This prototype demonstrates:

- A basic client/server setup using `std::net` (no `tokio`, no async).
- Sending a `.wasm` binary from the client to the server over a TCP connection.
- Reading, validating, and executing the WebAssembly module on the server using `wasmtime`.


## Protocol

- The client sends:
  1. A 4-byte length prefix (big-endian `u32`)
  2. Followed by the raw `.wasm` module bytes

- The server:
  - Reads the length prefix and wasm module
  - Executes the first exported function that:
    - Takes no parameters
    - Returns an `i32`
  - Responds with the result or an error message

  ## Example usage

  - Run the server with:
  cargo run serve

  - In another terminal, run the client with:
  cargo run connect wasm_hello.wasm
