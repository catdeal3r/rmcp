use ollama_rs::{
    generation::completion::request::GenerationRequest,
    Ollama,
};
use tokio::runtime::Runtime;
use serde::Deserialize;
use std::str::FromStr;


pub fn get_ai_response(prompt: &String, model: &String) -> String {
    let rt = Runtime::new().expect("FAILED: to create new tokio runtime. Err01");

    let future_task = async {
        let ollama = Ollama::default();

        let request = GenerationRequest::new(model.clone(), prompt.clone());

        ollama.generate(request).await
    };

    let res = rt.block_on(future_task).expect("FAILED: unable to block the main thread untill async task completes. Err02");

    res.response
}



#[derive(Deserialize)]
struct Response {
    state: String,
    tool: String,
    tool_identifier: String,
    tool_content: String,
}

pub struct ToolIdentifierContent {
    identifier: String,
    content: String,
}

#[derive(Debug, strum::EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum ResponseTool {
    WebSearch,
    WebSwarmSearch,
    WebFetch,
    FileWrite,
    FileRead,
    Output,
}

pub enum ResponseType {
    Pending(ResponseTool),
    Complete(String),
    Incorrect,
}

fn create_response_struct(raw_response: String) -> Response {
    let response: Response = serde_json::from_str(&raw_response).expect("FAILED: to parse json response from model. Err03");
    response
}


pub fn process_raw_response(raw_response: String) -> (ResponseType, ToolIdentifierContent) {
    let response: Response = self::create_response_struct(raw_response.clone());

    if response.state == "complete" && response.tool == "output" {
        return (ResponseType::Complete(response.tool_content), ToolIdentifierContent{identifier: "".to_string(), content: "".to_string()})
    }

    if let Ok(tool_type) = ResponseTool::from_str(&response.tool) {
        return (ResponseType::Pending(tool_type), ToolIdentifierContent{identifier: response.tool_identifier, content: response.tool_content})
    }

    (ResponseType::Incorrect, ToolIdentifierContent{identifier: "".to_string(), content: "".to_string()})
}
