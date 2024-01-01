use wasm_workers_rs::{
    bootstrap_worker, 
    http::{Request, Response},
};

#[derive(StdWorkflow)]
struct HelloWorld;

#[bootstrap_worker] 
fn main(worker: HelloWorld) {
    worker.init();
}

#[derive(StdWorkflow)]
impl HelloWorld {
    async fn handle(&self, req: Request<()>) -> Response<String> {
        Response::ok("Hello wasm!".to_owned())
    }
}