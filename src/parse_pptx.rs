pub mod parse_pptx {

    use std::fs::{self, File};

    use std::path::{self, Path};
    use ollama_rs::models::create;
    use zip::read::ZipArchive;

    fn unzip_file(zip_file_path: &str, extract_to: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(zip_file_path)?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = Path::new(extract_to).join(file.sanitized_name());

            if (&*file.name()).ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(())
    }

    use std::io::BufReader;
    use xml::reader::{EventReader, XmlEvent};

    async fn get_text_from_xml(
        filename: String,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let file = File::open(filename.clone()).unwrap();
        let file = BufReader::new(file);

        let parser = EventReader::new(file);

        let mut texts: Vec<String> = Vec::new();

        for event in parser {
            match event {
                Ok(XmlEvent::Characters(text)) => {
                    //println!("{}", text);
                    texts.push(text.clone());
                }
                // Handle other XML events if needed
                _ => {}
            }
        }

        Ok(texts)
    }

    fn get_sorted_list(num_files: u32) -> Vec<Result<Result<fs::DirEntry, io::Error>, io::Error>> {
        let mut file_entries = Vec::new();

        for i in 1..=num_files {
            let file_name = format!("slide{}.xml", i);
            let file_path = Path::new("./pptx-extract/ppt/slides/").join(file_name);

            // Attempt to read the file entry
            let file_entry = fs::metadata(&file_path).map(|_| {
                fs::read_dir("./pptx-extract/ppt/slides/")?
                    .find(|entry| {
                        if let Ok(entry) = entry {
                            entry.path() == file_path
                        } else {
                            false
                        }
                    })
                    .unwrap()
            });

            file_entries.push(file_entry);

        }

        file_entries
    }

    fn get_slide_number(path: String) -> u32 {
        let paths = fs::read_dir(path).unwrap();

        paths.count() as u32
    }

    use std::io;

    pub async fn get_text(file_path: String, filename: Option<String>) -> Vec<String> {
        unzip_file(&file_path, "./pptx-extract").unwrap();

        let mut texts: Vec<String> = Vec::new();

        let files_num = get_slide_number("./pptx-extract/ppt/slides".to_string());


        match filename {
            Some(fl) => {
                if !path::Path::exists(Path::new(&fl)) {
                    fs::write(fl, "".to_string()).unwrap();
                }
            }

            None => {
                if !path::Path::exists(Path::new("./zapis.md")) {
                    fs::write("./zapis.md", "".to_string()).unwrap();
                }
            }
        }

        //print!("{}", files_num.clone());

        let paths = get_sorted_list(files_num - 1);

        //println!("{:?}", paths);

        for path in paths {
            match path {
                Ok(path) => {
                    let path = path.unwrap().path();

                    if path.is_file() {
                        if let Some(file_name) = path.to_str() {
                            let text = get_text_from_xml(file_name.to_string()).await.unwrap();
                            texts.push(text.join(" "));
                        }
                    }
                }
                Err(e) => {
                    println!("error {}", e.to_string());
                }
            }
        }

        texts
    }
}
