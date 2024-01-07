#[handler]
fn reply(req: Request<String>) -> Result<Response<String>> {
    Ok(http::Response::builder()
    .status(200)
    .header("x-generated-by", "wasm-workers-server")
    .body(String::from("Hello Wasm!").into())?)
}

#[proc_macro_attribute]
pub fn handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand::expand_macro(attr, item)
}

const main_fn = quote! {
    use wasm_workers_rs::io::{Input, Output};
    use std::io::stdin;

    fn main() {
        let input = Input::new(stdin());
        let error = Output::new(
            "There was an error running the handler",
            500,
            None,
            None
        ).to_json().unwrap();

        if let Ok(input) = input {
            let mut cache = input.cache_data();

            if let Ok(response) = #func_call {
                match Output::from_response(response, cache).to_json() {
                    Ok(res) => println!("{}", res),
                    Err(_) => println!("{}", error)
                }
            } else {
                println!("{}", error)
            }
        } else {
            println!("{}", error)
        }
    }

    #handler_fn
};