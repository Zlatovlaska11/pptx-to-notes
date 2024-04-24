pub mod lamapi {

    use std::fs::{File, OpenOptions};

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
    }

    pub async fn send_simple_question(question: String) -> Option<String> {
        let prmot = "udelej jednoduchy a strucny zapisk ve formatu markdown z cestiny ktery bude v cestine z tohoto textu prosim: ";
        let prmpt = prmot.to_string() + &question;

        let mut file = File::open("config.json").expect("Failed to open file");

        let mut json_str = String::new();
        file.read_to_string(&mut json_str)
            .expect("Failed to read file");
        let conf: Config =
            serde_json::from_str(&json_str).expect("JSON was not well-formatted");

        let prompt: Value = json!({


        "model": conf.model,
        "prompt": prmpt,
        "stream": false

                    });

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

            write_to_file(data.response.clone());

            Some(data.response)
        } else {
            None
        }
    }

    use std::io::prelude::*;

    fn write_to_file(text: String) {
        let mut fl = OpenOptions::new()
            .write(true)
            .append(true)
            .open("./zapis.md")
            .unwrap();

        if let Err(e) = writeln!(fl, "{} \n ", text) {
            eprintln!("Couldn't write to file: {}", e);
        }
        if let Err(e) = writeln!(fl, "{} \n ", "___") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
