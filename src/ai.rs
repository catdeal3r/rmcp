use ollama_rs::{
    generation::completion::request::GenerationRequest,
    Ollama,
};
use tokio::runtime::Runtime;
use serde::Deserialize;
use std::str::FromStr;


pub fn get_ai_response(prompt: &String, model: &String) -> (String, Option<String>) {
    let rt = Runtime::new().expect("FAILED: to create new tokio runtime. Err01");

    let future_task = async {
        let ollama = Ollama::default();

        let request = GenerationRequest::new(model.clone(), prompt.clone());

        ollama.generate(request).await
    };

    let res = rt.block_on(future_task).expect("FAILED: unable to access the ollama server. Err02");

    (res.response, res.thinking)
}

pub fn check_whether_ollama_is_running() -> bool {
    let rt = Runtime::new().expect("FAILED: to create new tokio runtime. Err07");

    let future_task = async {
        let ollama = Ollama::default();

        ollama.list_local_models().await
    };

    let res = rt.block_on(future_task);

    match res {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}


pub fn check_whether_ollama_has_any_models() -> i32 {
    let rt = Runtime::new().expect("FAILED: to create new tokio runtime. Err08");

    let future_task = async {
        let ollama = Ollama::default();

        ollama.list_local_models().await
    };

    let res = rt.block_on(future_task).expect("FAILED: unable to access the ollama server. Err09");

    if res.is_empty() {
        return 0;
    } else {
        return res.len().try_into().unwrap();
    }
}


pub fn check_whether_ollama_has_a_model(target_model: &str) -> bool {
    let rt = Runtime::new().expect("FAILED: to create new tokio runtime. Err10");

    let future_task = async {
        let ollama = Ollama::default();

        ollama.list_local_models().await
    };

    let res = rt.block_on(future_task).expect("FAILED: unable to access the ollama server. Err11");

    if res.is_empty() {
        return false;
    } else {
        let exists = res.iter().any(|m| m.name.contains(target_model));
        return exists;
    }
}


#[derive(Deserialize)]
struct Response {
    state: String,
    tool: String,
    tool_identifier: String,
    tool_content: String,
}

pub struct ToolIdentifierContent {
    pub identifier: String,
    pub content: String,
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
