use wasm_workers_rs::{
    http::{Request, Response},
    StdWorkflow,
};

struct HelloWorld;

#[derive(StdWorkflow)]
impl HelloWorld {
    async fn handle(&self, req: Request) -> Result<Response> {
        Ok(Response::ok("Hello wasm!").header("x-generated-by", "wasm-workers-server"))
    }
}