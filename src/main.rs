use colored::*;

pub mod consts;
pub mod ai;
pub mod utils;
pub mod tools;
pub mod search;

fn main() {    
    println!("\nRunning preflight checks ...");
    let model = "gemma4:e2b".to_string();

    if !utils::preflight_ollama_server() || !utils::preflight_ollama_models()
        || !utils::preflight_ollama_model(&model) {
        return;
    }
    
    println!("\n{}\n", utils::get_welcome_line(&model, &"ollama".to_string(), &"all".to_string()));
            
    let mut previous_messages: Vec<String> = Vec::new();

    let mut task_pending = false;

    loop {
        let input: String;
        
        if task_pending {
            input = "No input from the user, continue working on the current task.".to_string();
        } else {
            input = utils::get_input(format!(" {}  ", ">".bold().blue()));
        }
        
        println!("");

        let mut thinking_spinner = utils::create_spinner("Thinking ...");
        let full_prompt = consts::generate_full_prompt(&input, previous_messages.clone());
        
        let (raw_response, thinking_reason) = ai::get_ai_response(&full_prompt, &model);

        if let Some(thinking) = thinking_reason {
            println!("Reasoning: \"{}\"", thinking)
        }

        if !task_pending {
            let user_message = format!("User: \"{}\"", &input);
            previous_messages.push(user_message);
        }
        
        let response_message = format!("AI: \"{}\"", &raw_response);
        previous_messages.push(response_message);
        
        thinking_spinner.success("Finished thinking.");

        println!("");

        let (response_type, tool_identifier_content) = ai::process_raw_response(raw_response.clone());

        if let ai::ResponseType::Complete(ref output) = response_type {
            utils::format_and_print_markdown_content(&output);

            task_pending = false;
        }

        if let ai::ResponseType::Pending(ai::ResponseTool::FileWrite) = response_type {
            let write_to_file_response = tools::write_to_file(tool_identifier_content.identifier.clone(), tool_identifier_content.content.clone());

            previous_messages.push(write_to_file_response);
            task_pending = true;
        }

        if let ai::ResponseType::Pending(ai::ResponseTool::Output) = response_type {
            utils::format_and_print_markdown_content(&tool_identifier_content.content.clone());

            task_pending = true;
        }

        if let ai::ResponseType::Pending(ai::ResponseTool::WebSearch) = response_type {
            let web_search_response = tools::web_search(&tool_identifier_content.content.clone());

            previous_messages.push(web_search_response);
            task_pending = true;
        }
    }
}
