use clap::Parser;
use std::net::{SocketAddr, TcpListener};

mod node;
mod types;

use node::Node;
#[derive(Parser)]
struct Opt {
    /// If provided, bind to the provided address (e.g. 127.0.0.1:1337).
    /// If omitted, the OS will pick a free port on 127.0.0.1.
    #[clap(long)]
    addr: Option<SocketAddr>,

    /// If present, listen for inbound connections
    #[clap(long)]
    gateway: bool,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let bind_addr = opt.addr.unwrap_or_else(|| "127.0.0.1:0".parse().unwrap());
    let actual_addr = if bind_addr.port() == 0 {
        let listener = TcpListener::bind(bind_addr)?;
        let local = listener.local_addr()?;
        std::mem::drop(listener);
        local
    } else {
        bind_addr
    };

    println!(
        "Starting node on {} as {}",
        actual_addr,
        if opt.gateway { "gateway" } else { "edge" }
    );

    let mut node = Node::new(actual_addr, opt.gateway);

    Ok(())
}
