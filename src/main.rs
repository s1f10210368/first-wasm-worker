use anyhow::Result;
use wasm_workers_rs::{
    handler,
    http::{Request, Response},
};

// そもそもhandlerがライブラリの中から消えているため対策必要

#[handler]
fn reply(req: Request) -> Result<Response> {
    Ok(Response::ok("Hello wasm!")
        .header("x-generated-by", "wasm-workers-server")) 
}
