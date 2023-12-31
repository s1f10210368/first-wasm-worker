use wasm_workers_rs::{
    bootstrap_worker,
    http::{Request, Response}, 
    StdWorkflow,
};

struct HelloWorld;

#[derive(StdWorkflow)]
impl HelloWorld {
    async fn handle(&self, req: Request) -> Response {
        Response::ok("Hello wasm!")
    }
}

#[bootstrap_worker]
fn main(mut worker: HelloWorld) {
    worker.init();
}