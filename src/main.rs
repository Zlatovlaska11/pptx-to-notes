
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version = "0.9", about = "simple powerpoint to notes converter", long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,

    #[arg(short, long)]
    create_file: bool,

    #[arg(short, long)]
    output_file_name: String,

    #[arg(short, long)]
    subject: Subject,

}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Subject {
    Czech,
    Vypoceka,
    Chemie,
    Dejak
}

pub mod args_handler;
pub mod ollama_api_wrapper;
pub mod parse_pptx;

#[tokio::main]
async fn main() {
    args_handler::args_handler::caller().await.unwrap();
}
