pub mod args_handler;
pub mod ollama_api_wrapper;
pub mod parse_pptx;

#[tokio::main]
async fn main() {
    args_handler::args_handler::caller().await.unwrap();
}
