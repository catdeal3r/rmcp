use termimad::crossterm::style;
use std::io::{self, Write};
use spinoff::{Spinner, spinners, Color};

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
