use std::fs;
use crate::utils;
use crate::search;
use std::io::ErrorKind;

pub fn write_to_file(filename: String, content: String) -> String {
    loop {
        let raw_input = utils::get_input(format!("Allow write to filesystem: \"{}\" (y)es/(n)o/(b)ackup first: ", filename));
        let input = raw_input.to_lowercase().trim().to_string();

        if input != "y" && input != "n" && input != "b" {
            print!("Incorrect input.\n");
            continue
        }

        if input == "n" {
            return "User denied file write.".to_string()
        }

        if input == "b" {
            let backup_filename = format!("{}.backup", filename);
            
            match fs::copy(&filename, &backup_filename) {
                Ok(_) => println!("File backed up to {backup_filename}."),
                Err(e) if e.kind() == ErrorKind::NotFound => {}
                Err(_) => panic!("ERROR: failed to copy file. Err06"), 
            }
        }

        break
    }

    let return_string: String;

    println!("");
    
    let mut file_writing_spinner = utils::create_spinner("Writing to file ...");
    
    match fs::write(&filename, &content) {
        Ok(_) => return_string = format!("Successfully wrote to file: {}.", filename),
        Err(_) => return_string = format!("Failed to write to file: {}.", filename),
    }

    file_writing_spinner.success("Finished writing.");

    println!("\n{}", return_string);

    return_string
}

pub fn web_search(query: &str) -> String {
    
    let mut web_search_spinner = utils::create_spinner(&format!("Searching the web for \"{}\"...", query));
    let web_search_response = search::tavily_keyless_search(query, "5");
    
    web_search_spinner.success("Finished web search.");
    web_search_response
}


pub fn read_from_file(filename: String) -> String {
    loop {
        let raw_input = utils::get_input(format!("Allow read from filesystem: \"{}\" (y)es/(n)o: ", filename));
        let input = raw_input.to_lowercase().trim().to_string();

        if input != "y" && input != "n" {
            print!("Incorrect input.\n");
            continue
        }

        if input == "n" {
            return "User denied file read.".to_string()
        }

        break
    }

    let return_string: String;

    println!("");
    
    let mut file_reading_spinner = utils::create_spinner("Reading from file ...");
    let mut failed = false;
    
    match fs::read_to_string(&filename) {
        Ok(s) => return_string = format!("File contents: {}.", s),
        Err(_) => {
            return_string = format!("Failed to read from file: {}.", filename);
            failed = true
        },
    }

    file_reading_spinner.success("Finished reading.");

    if failed {
        println!("\n{}", return_string);
    }

    return_string
}
