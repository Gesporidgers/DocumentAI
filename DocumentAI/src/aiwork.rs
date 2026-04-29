use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

pub async fn process(file1: &str, file2: &str) -> String {
    let ollama = Ollama::default();
    let model = "gpt-oss:20b".to_string();
    let prompt = "Привет, друг. Ответь пожалуйста кодом разметки pango";
    /*let prompt = format!("Ты — эксперт по анализу технических условий.

Сравни два документа. Выяви:
1) отсутствующие требования (покажи как оно выглядело в исходном)
2) разницу по смыслу,
3) некорректные или неполные части.
4) синтаксические и другие ошибки в тексте документа для анализа

Эталонный документ:
{}

Документ для анализа:
{}

Дай подробный и конкретный отчёт кодом разметки pango о всех выявленных проблемах (нужны все ошибки по конкретной проблеме) при сравнении документов и предложи решение каждой ошибки в структурированном виде.", file1, file2);*/
    
    match ollama.generate(GenerationRequest::new(model, prompt)).await {
        Ok(res) => res.response,
        Err(_) => "Ошибка при загрузке модели".to_string(),
    }
}