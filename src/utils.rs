use termimad::crossterm::style;
use std::io::{self, Write};
use spinoff::{Spinner, spinners, Color};
use colored::*;

use crate::ai;

pub fn set_colours(skin: &mut termimad::MadSkin) {
    skin.set_fg(style::Color::Rgb { r: 210, g: 210, b: 210 }); // orange-ish              
                                                                             
    skin.bold.set_fg(style::Color::Rgb { r: 255, g: 255, b: 255 });                     
                                                                            
    skin.italic.set_fg(style::Color::Rgb { r: 180, g: 180, b: 180 });                   
                                                                             
    skin.code_block.set_fg(style::Color::Rgb { r: 200, g: 200, b: 200 });
    skin.code_block.set_bg(style::Color::Rgb { r: 30, g: 30, b: 30 }); 
}


pub fn get_input(text: String) -> String {
    let mut input = String::new();

    print!("{}", text);

    io::stdout().flush().expect("FAILED: to flush stdout. Err04");

    io::stdin()
        .read_line(&mut input)
        .expect("FAILED: to read line from stdin. Err05");

    input
}

pub fn format_and_print_markdown_content(content: &str) {
    let mut skin = termimad::MadSkin::default();
    self::set_colours(&mut skin);
    let area = termimad::Area::full_screen();
    
    let formatted_content = termimad::FmtText::from(&skin, content, Some(area.width.into()));
    println!("{}", formatted_content);
}


pub fn create_spinner(content: &str) -> Spinner {
    let spinner_content = &content.to_string();
    Spinner::new(spinners::Line, spinner_content.clone(), Color::Blue)
}


pub fn get_welcome_line(model: &String, provider: &String, tools_amount: &String) -> String {
    format!("{} {} {}\n{}: {} {} {}: {} {} {}: {}", "rmcp".bold().blue(), "·".bold().dimmed(),
        env!("CARGO_PKG_VERSION").bold().dimmed(), "model".bold().blue(),
        model, "·".bold().dimmed(), "provider".bold().blue(), provider,
        "·".bold().dimmed(), "tools".bold().blue(), tools_amount)
}


pub fn preflight_ollama_server() -> bool {
    let mut server_spinner = self::create_spinner("Checking that the ollama server is running ...");
    let ollama_server_up = ai::check_whether_ollama_is_running();
    
    if ollama_server_up {
        server_spinner.success("Ollama server is up.");
        return true;
    } else {
        server_spinner.fail("Ollama server isn't running.");
        return false;
    }
}

pub fn preflight_ollama_models() -> bool {
    let mut available_models_spinner = self::create_spinner("Checking for available models  ...");
    let ollama_available_models = ai::check_whether_ollama_has_any_models();
    
    if ollama_available_models > 0 {
        available_models_spinner.success(&format!("Ollama has {} models available.", ollama_available_models));
        return true;
    } else {
        available_models_spinner.fail("Ollama has no available models");
        return false;
    }
}

pub fn preflight_ollama_model(model: &String) -> bool {
    let mut model_spinner = self::create_spinner("Checking that the ollama server has the current model ...");
    let ollama_model_exists = ai::check_whether_ollama_has_a_model(model);
    
    if ollama_model_exists {
        model_spinner.success(&format!("The model \"{}\" is available.", model));
        return true;
    } else {
        model_spinner.fail(&format!("The model \"{}\" isn't available.", model));
        return false;
    }
}
