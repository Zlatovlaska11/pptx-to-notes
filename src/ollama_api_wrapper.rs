pub mod lamapi {


    use http::{Request, Response};

    fn send_simple_question(question: String) {

        let mut request = Request::builder()
            .uri("http://localhost:11434/api/generate")
            .header("User-Agent", "my-awesome-agent/1.0")
            .body(question);

        // finish accepting the response and returning it        
        
    }
}
