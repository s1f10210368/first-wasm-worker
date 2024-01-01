use wasm_workers_rs::{
    bootstrap_worker, 
    http::{Request, Response},
    StdWorkflow
};

#[derive(StdWorkflow)] 
struct HelloWorld;

#[bootstrap_worker]
fn main() {

}

#[derive(StdWorkflow)]
impl HelloWorld {
    async fn handle(&self, req: Request<()>) -> Response<String> {
        Response::ok("Hello wasm!".to_owned())
    }
}