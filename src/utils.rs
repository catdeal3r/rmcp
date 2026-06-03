use termimad::{crossterm::style, FmtText};
use std::io::{self, Write};
use colored::*;

pub fn set_colours(skin: &mut termimad::MadSkin) {
    skin.set_fg(style::Color::Rgb { r: 210, g: 210, b: 210 }); // orange-ish              
                                                                             
    skin.bold.set_fg(style::Color::Rgb { r: 255, g: 255, b: 255 });                     
                                                                            
    skin.italic.set_fg(style::Color::Rgb { r: 180, g: 180, b: 180 });                   
                                                                             
    skin.code_block.set_fg(style::Color::Rgb { r: 200, g: 200, b: 200 });
    skin.code_block.set_bg(style::Color::Rgb { r: 30, g: 30, b: 30 }); 
}


pub fn get_input() -> String {
    let mut input = String::new();

    print!(" {}  ", ">".bold().blue());

    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    input
}

pub fn format_markdown_content(content: &str) -> FmtText<'_, '_> {
    let mut skin = termimad::MadSkin::default();
    self::set_colours(&mut skin);
    let area = termimad::Area::full_screen();
    
    termimad::FmtText::from(&skin, content, Some(area.width.into()))
}
