use std::fs;
use crate::utils;
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

    print!("\n");
    
    let mut file_writing_spinner = utils::create_spinner("Writing to file ...");
    
    match fs::write(&filename, &content) {
        Ok(_) => return_string = format!("Successfully wrote to file: {}.", filename),
        Err(_) => return_string = format!("Failed to write to file: {}.", filename),
    }

    file_writing_spinner.success("Finished writing.");

    print!("\n{}\n", return_string);

    return_string
}

