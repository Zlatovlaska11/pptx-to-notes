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
}

pub mod ollama_api_wrapper;
pub mod parse_pptx;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let texts;
    match args.create_file {
        true => {
            texts =
                parse_pptx::parse_pptx::get_text(args.file_name, Some(args.output_file_name)).await;
        }
        false => {
            texts = parse_pptx::parse_pptx::get_text(args.file_name, None).await;
        }
    }

    //println!("{}, {}!", args.file_name, args.ollama_model);
    for txt in texts {
        let resp = ollama_api_wrapper::lamapi::send_simple_question(txt).await;

        match resp {
            Some(x) => println!("{x}"),
            None => println!("error with sending req"),
        }
    }
}
