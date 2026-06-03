use ollama_rs::{
    generation::completion::request::GenerationRequest,
    Ollama,
};
use tokio::runtime::Runtime;

pub fn get_ai_response(prompt: &String, model: &String) -> String {
    // 1. Create a standard synchronous Tokio runtime
    let rt = Runtime::new().unwrap();

    // 2. Define your async block or function
    let future_task = async {
        let ollama = Ollama::default();

        let request = GenerationRequest::new(model.clone(), prompt.clone());

        ollama.generate(request).await
    };

    // 3. Block the main thread until the async task completes
    let res = rt.block_on(future_task).unwrap();

    res.response
}
