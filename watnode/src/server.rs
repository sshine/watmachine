use anyhow::Result;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use wasmtime::{Engine, ExternType, Linker, Module, Store, ValType};

pub fn server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on {}", addr);

    // Accept connections and process them sequentially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Client handler error: {:#}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let result: Result<String> = (|| {
        let wasm_len = read_length_prefix(&mut stream)
            .map_err(|e| anyhow::anyhow!("Could not read length prefix: {e}"))?;

        println!("Expecting wasm module of length: {} bytes", wasm_len);

        let wasm_buf = read_exact_bytes(&mut stream, wasm_len)
            .map_err(|e| anyhow::anyhow!("Problem reading wasm module length: {e}"))?;

        println!(
            "Received wasm module successfully ({} bytes)",
            wasm_buf.len()
        );

        execute_module(&wasm_buf)
            .map_err(|e| anyhow::anyhow!("Failed to execute wasm module: {e}"))?;

        Ok("Wasm executed successfully.".into())
    })();

    // Send response to client
    let response = match result {
        Ok(msg) => msg,
        Err(err) => {
            eprintln!("Error while handling client: {:#}", err);
            format!("Error: {err}")
        }
    };
    stream.write_all(format!("{response}\n").as_bytes())?;
    println!("Response sent to client: {}", response);

    Ok(())
}

fn read_length_prefix(stream: &mut TcpStream) -> Result<usize> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    Ok(u32::from_be_bytes(len_buf) as usize)
}

fn read_exact_bytes(stream: &mut TcpStream, len: usize) -> Result<Vec<u8>> {
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    Ok(buf)
}

fn execute_module(wasm_buf: &[u8]) -> Result<()> {
    let engine = Engine::default();
    let module = Module::new(&engine, wasm_buf)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    // Instantiate the module
    let instance = linker.instantiate(&mut store, &module)?;

    // Call the first function that has no arguments and returns an i32
    for export in module.exports() {
        if let ExternType::Func(func_ty) = export.ty() {
            let params: Vec<_> = func_ty.params().collect();
            let results: Vec<_> = func_ty.results().collect();

            if params.is_empty() && results.len() == 1 && matches!(results[0], ValType::I32) {
                let func = instance
                    .get_func(&mut store, export.name())
                    .ok_or_else(|| anyhow::anyhow!("Missing function '{}'", export.name()))?;

                let typed: wasmtime::TypedFunc<(), i32> = func.typed(&store)?;
                let result = typed.call(&mut store, ())?;
                println!("Called '{}', returned: {}", export.name(), result);
                return Ok(());
            }
        }
    }
    anyhow::bail!("No compatible (no-arg, i32-return) function found")
}
