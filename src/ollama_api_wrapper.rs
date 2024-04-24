pub mod lamapi {

    use std::fs::{File, OpenOptions};

    use ollama_rs::Ollama;
    use reqwest;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    #[derive(Serialize, Deserialize)]
    struct JsonExtract {
        model: String,
        created_at: String,
        response: String,
        done: bool,
    }

    #[derive(Serialize, Deserialize)]
    struct Config {
        model: String,
        prompt_czech: String,
        prompt_vypocetka: String,
        prompt_dejak: String,
        prompt_chemie: String,
    }

    pub async fn send_simple_question(
        text: String,
        subject: crate::args_handler::args_handler::Subject,
        filename: Option<String>,
    ) -> Option<String> {
        let mut file = File::open("config.json").expect("Failed to open file");

        let mut json_str = String::new();
        file.read_to_string(&mut json_str)
            .expect("Failed to read file");
        let conf: Config = serde_json::from_str(&json_str).expect("JSON was not well-formatted");
        let prmot: String;

        match subject {
            crate::args_handler::args_handler::Subject::Czech => {
                prmot = format!("{} {}", conf.prompt_czech, text)
            }
            crate::args_handler::args_handler::Subject::Vypoceka => {
                prmot = format!("{} {}", conf.prompt_vypocetka, text)
            }
            crate::args_handler::args_handler::Subject::Chemie => {
                prmot = format!("{} {}", conf.prompt_chemie, text)
            }
            crate::args_handler::args_handler::Subject::Dejak => {
                prmot = format!("{} {}", conf.prompt_dejak, text)
            }
        }

        let prompt: Value = json!({


        "model": conf.model,
        "prompt": prmot,
        "stream": false

                    });

        let _ollama = Ollama::default();

        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:11434/api/generate")
            .body(prompt.to_string())
            .send()
            .await
            .unwrap();

        if response.status().is_success() {
            let body_bytes = response.bytes().await.unwrap(); // Read response body as bytes
            let data: JsonExtract = serde_json::from_slice(&body_bytes).unwrap(); // Deserialize bytes into Person struct

            write_to_file(data.response.clone(), filename);

            Some(data.response)
        } else {
            None
        }
    }

    use std::io::prelude::*;

    fn write_to_file(text: String, filename: Option<String>) {
        let mut fname = String::from("zapis.md");
        if filename.is_some() {
            fname = filename.unwrap()
        }

        let mut fl = OpenOptions::new()
            .write(true)
            .append(true)
            .open(fname)
            .unwrap();

        if let Err(e) = writeln!(fl, "{} \n ", text) {
            eprintln!("Couldn't write to file: {}", e);
        }
        if let Err(e) = writeln!(fl, "{} \n ", "___") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
