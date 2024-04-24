pub mod args_handler {

    use std::error;

    use clap::Parser;

    use crate::{ollama_api_wrapper, parse_pptx};
    #[derive(Parser, Debug)]
    #[command(version = "1.0 Beta", about = "simple powerpoint to notes converter", long_about = None)]
    struct Args {
        #[arg(short, long)]
        file_name: String,

        #[arg(short, long)]
        create_file: bool,

        #[arg(short, long, default_value=Some("zapis.md"))]
        output_file_name: Option<String>,

        #[arg(short, long)]
        subject: Subject,
    }

    #[derive(Debug, Clone, clap::ValueEnum)]
    pub enum Subject {
        Czech,
        Vypoceka,
        Chemie,
        Dejak,
    }

    pub async fn caller() -> Result<(), Box<dyn error::Error>> {
        let args = Args::parse();

        let texts;
        match args.create_file {
            true => {
                texts = parse_pptx::parse_pptx::get_text(
                    args.file_name,
                    Some(args.output_file_name.clone().expect("output file not set")),
                )
                .await;
            }
            false => {
                texts = parse_pptx::parse_pptx::get_text(args.file_name, None).await;
            }
        }

        //println!("{}, {}!", args.file_name, args.ollama_model);
        for txt in texts {
            let resp =
                ollama_api_wrapper::lamapi::send_simple_question(txt, args.subject.clone(), args.output_file_name.clone()).await;

            match resp {
                Some(x) => println!("{x}"),
                None => println!("error with sending req"),
            }
        }

        Ok(())
    }
}
