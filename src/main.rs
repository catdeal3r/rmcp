
pub mod consts;
pub mod ai;
pub mod utils;

fn main() {
    //println!("{}", consts::generate_full_prompt("What are the animal cousins of the rabbit?", vec!["User: \"what are the animal cousins of the dingo?\"", "AI: \"The New Guinea Singing Dog and East Asian dogs are close cousins to the dingo\""]));

    let mut previous_messages: Vec<&str> = Vec::new();

    loop {
        let input = utils::get_input();

        let mut thinking_spinner = utils::create_spinner("Thinking ...");
        let full_prompt = consts::generate_full_prompt(&input, previous_messages.clone());
        
        let response = ai::get_ai_response(&full_prompt, &"gemma4:e2b".to_string());

        let user_message = &format!("User: \"{}\"", &input);
        let response_message = &format!("AI: \"{}\"", &response);
        
        previous_messages.push(user_message);
        previous_messages.push(response_message);
        thinking_spinner.success("Finished.");

        utils::format_and_print_markdown_content(&response);
    }
}
