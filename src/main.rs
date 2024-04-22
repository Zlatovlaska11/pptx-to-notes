
pub mod parse_pptx;
pub mod ollama_api_wrapper;

fn main() {
    parse_pptx::parse_pptx::get_text("tst.pptx".to_string());
}

