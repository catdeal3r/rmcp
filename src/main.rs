
pub mod consts;
pub mod ai;
pub mod utils;

fn main() {
    let mut previous_messages: Vec<String> = Vec::new();

    loop {
        let input = utils::get_input();

        let mut thinking_spinner = utils::create_spinner("Thinking ...");
        let full_prompt = consts::generate_full_prompt(&input, previous_messages.clone());
        
        let raw_response = ai::get_ai_response(&full_prompt, &"gemma4:e2b".to_string());

        let user_message = format!("User: \"{}\"", &input);
        let response_message = format!("AI: \"{}\"", &raw_response);
        
        previous_messages.push(user_message);
        previous_messages.push(response_message);
        thinking_spinner.success("Finished.");

        let (response_type, tool_identifier_content) = ai::process_raw_response(raw_response.clone());

        if let ai::ResponseType::Complete(output) = response_type {
            utils::format_and_print_markdown_content(&output);
        } else {
            print!("{}", raw_response);
        }
    }
}
