use std::io::{self, Write};
use colored::*;
use spinoff::{Spinner, spinners, Color};

pub mod consts;
pub mod ai;
pub mod utils;

fn main() {
    //println!("{}", consts::generate_full_prompt("What are the animal cousins of the rabbit?", vec!["User: \"what are the animal cousins of the dingo?\"", "AI: \"The New Guinea Singing Dog and East Asian dogs are close cousins to the dingo\""]));


    
    let mut skin = termimad::MadSkin::default();
    utils::set_colours(&mut skin);
    let area = termimad::Area::full_screen();

    loop {
        let mut input = String::new();

        print!(" {}  ", ">".bold().blue());

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let mut thinking_loading = Spinner::new(spinners::Dots3, "Thinking ...", Color::Blue);
        
        let response = ai::get_ai_response(&input, &"gemma4:e2b".to_string());

        let formatted_content = termimad::FmtText::from(&skin, &response, Some(area.width.into()));

        thinking_loading.success("Finished.");

        println!("\n{}", formatted_content);
    }
}
