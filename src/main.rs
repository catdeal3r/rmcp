use spinoff::{Spinner, spinners, Color};

pub mod consts;
pub mod ai;
pub mod utils;

fn main() {
    //println!("{}", consts::generate_full_prompt("What are the animal cousins of the rabbit?", vec!["User: \"what are the animal cousins of the dingo?\"", "AI: \"The New Guinea Singing Dog and East Asian dogs are close cousins to the dingo\""]));

    loop {
        let input = utils::get_input();

        let mut thinking_loading = Spinner::new(spinners::Line, "Thinking ...", Color::Blue);
        
        let response = ai::get_ai_response(&input, &"gemma4:e2b".to_string());
        thinking_loading.success("Finished.");

        let formatted_content = utils::format_markdown_content(&response);

        println!("\n{}", formatted_content);
    }
}
