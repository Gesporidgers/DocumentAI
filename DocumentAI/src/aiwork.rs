use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

pub async fn process(file1: &str, file2: &str) -> String {
    let ollama = Ollama::default();
    let model = "gpt-oss:20b".to_string();
    let prompt = "Привет, друг. Ответь пожалуйста кодом разметки pango";
    //let prompt = format!("Сравни два документа:\nЭталон: {}\nДокумент: {}", file1, file2);
    
    match ollama.generate(GenerationRequest::new(model, prompt)).await {
        Ok(res) => res.response,
        Err(_) => "Ошибка при загрузке модели".to_string(),
    }
}