use clap::Parser;


#[derive(Parser, Debug)]
#[command(version = "0.9", about = "simple powerpoint to notes converter", long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,


    #[arg(short, long)]
    ollama_model: String,
}

pub mod parse_pptx;
pub mod ollama_api_wrapper;


fn main() {
    let args = Args::parse();

   parse_pptx::parse_pptx::get_text(args.file_name); 

    //println!("{}, {}!", args.file_name, args.ollama_model);
}
