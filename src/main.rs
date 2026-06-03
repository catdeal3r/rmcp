use colored::*;

pub mod consts;
pub mod ai;
pub mod utils;
pub mod tools;

fn main() {
    println!("\nRunning preflight checks ...");
    let model = "gemma4:e2b".to_string();

    // TODO: move this to a function in utils.rs
    
    let mut server_spinner = utils::create_spinner("Checking that the ollama server is running ...");
    let ollama_server_up = ai::check_whether_ollama_is_running();
    
    if ollama_server_up {
        server_spinner.success("Ollama server is up.");
    } else {
        server_spinner.fail("Ollama server isn't running.");
        return;
    }

    let mut available_models_spinner = utils::create_spinner("Checking for available models  ...");
    let ollama_available_models = ai::check_whether_ollama_has_any_models();
    
    if ollama_available_models > 0 {
        available_models_spinner.success(&format!("Ollama has {} models available.", ollama_available_models));
    } else {
        available_models_spinner.fail("Ollama has no available models");
        return;
    }

    
    let mut model_spinner = utils::create_spinner("Checking that the ollama server has the current model ...");
    let ollama_model_exists = ai::check_whether_ollama_has_a_model(&model);
    
    if ollama_model_exists {
        model_spinner.success(&format!("The model \"{}\" is available.", model));
    } else {
        model_spinner.fail(&format!("The model \"{}\" isn't available.", model));
        return;
    }
    
    // ------

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
        
        let raw_response = ai::get_ai_response(&full_prompt, &model);

        if !task_pending {
            let user_message = format!("User: \"{}\"", &input);
            previous_messages.push(user_message);
        }
        
        let response_message = format!("AI: \"{}\"", &raw_response);
        previous_messages.push(response_message);
        
        thinking_spinner.success("Finished thinking.");

        println!("");

        let (response_type, tool_identifier_content) = ai::process_raw_response(raw_response.clone());

        if let ai::ResponseType::Complete(output) = response_type {
            utils::format_and_print_markdown_content(&output);

            task_pending = false;
            continue
        }

        if let ai::ResponseType::Pending(ai::ResponseTool::FileWrite) = response_type {
            let write_to_file_response = tools::write_to_file(tool_identifier_content.identifier, tool_identifier_content.content);

            previous_messages.push(write_to_file_response);
            task_pending = true;
        }
    }
}
