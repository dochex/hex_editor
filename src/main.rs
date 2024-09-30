mod hexview;

use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::vec::Vec;

const MENU_ITEMS: [&str; 3] = ["read a byte", "change some bytes", "exit"];

fn main() {
    let path_file = get_file_path();
    let mut buffer = read_file_buffer(&path_file).unwrap();
    let view = hexview::HexView::new(&buffer);
    println!("{}", format!("{view}"));
    loop {
        let selection = display_menu(&MENU_ITEMS);
        match handle_user_action(selection, &mut buffer, &path_file) {
            Ok(_) => continue,
            Err("User requested exit") => break,
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn get_file_path() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Set the path of the file")
        .default("C:/Users/Philippe/RustroverProjects/hex_editor/hello.pdf".to_string())
        .validate_with(|input: &String| -> Result<(), &str> {
            let path = Path::new(input);
            if path.exists() && path.is_file(){
                Ok(())
            } else {
                Err("This file does not exist or is a directory")
            }
        })
        .interact_text()
        .expect("Failed to get valid file path input")
}
fn display_menu(items: &[&str]) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want?")
        .items(items)
        .interact()
        .expect("Failed to get menu selection")
}

fn handle_user_action(selection: usize, buffer: &mut Vec<u8>, path_file: &str) -> Result<(), &'static str> {
    match selection {
        0 => {
            read_byte(buffer);
            Ok(())
        },
        1 => {
            change_bytes(buffer, path_file);
            Ok(())
        },
        2 => Err("User requested exit"),
        _ => Err("Invalid selection"),
    }
}

fn change_bytes(buffer: &mut Vec<u8>, path_file: &str) {
    let how_many: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the number of byte to change")
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<i8>() {
                Ok(_) => Ok(()),
                Err(_) => Err("This is not a valid number")
            }
        })
        .interact_text()
        .unwrap();
    let how_many: usize = how_many.parse().expect("Invalid number");
    let mut n = 0;
    while n < how_many {
        let the_address: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the address of the byte to change")
            .validate_with(|the_address: &String| -> Result<(), &str> {
                match u64::from_str_radix(the_address, 16) {
                    Ok(the_address) if the_address < buffer.len() as u64 => Ok(()),
                    Ok(_) => Err("Address out of range"),
                    Err(_) => Err("This is not a valid hexadecimal string")
                }
            })
            .interact_text()
            .unwrap();
        let the_byte: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the byte for change")
            .validate_with(|the_byte: &String| -> Result<(), &str> {
                match u64::from_str_radix(the_byte, 16) {
                    Ok(_) => Ok(()),
                    Err(_) => Err("This is not an hexadecimal string")
                }
            })
            .interact_text()
            .unwrap();
        let address_in_int = i64::from_str_radix(&the_address, 16).expect("bad hex string in input");
        let the_byte_in_int = i64::from_str_radix(&the_byte, 16).expect("bad hex string in input");
        buffer[address_in_int as usize] = the_byte_in_int as u8;
        n += 1;
    }
    let view = hexview::HexView::new(&buffer);
    println!("{view}");
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to save the change?")
        .default(true)
        .show_default(false)
        .interact()
        .unwrap()
    {
        if fs::write(Path::new(&path_file.replacen(".pdf", "-new.pdf", 1)), buffer).is_ok() {
            println!("the new file is created");
        }else{
            println!("failed to create the new file");
        }
    } else {
        println!("nevermind then :(");
    }
}
fn read_byte(buffer: &Vec<u8>) -> (usize, &u8) {
    let addr_to_read: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the address of the byte to read")
        .validate_with(|input: &String| -> Result<(), &str> {
            match u64::from_str_radix(input, 16) {
                Ok(addr) if addr < buffer.len() as u64 => Ok(()),
                Ok(_) => Err("Address out of range"),
                Err(_) => Err("This is not a valid hexadecimal string")
            }
        })
        .interact_text()
        .unwrap();
    let addr_in_int = usize::from_str_radix(&addr_to_read, 16).expect("Invalid hexadecimal string");
    let byte = buffer.get(addr_in_int).expect("Invalid hexadecimal string");
    println!("The byte at address '{}' is '{:02X}'", addr_to_read, byte);
    println!();
    (addr_in_int, byte)
}

fn read_file_buffer(path_file: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut contents = Vec::new();
    let file = match File::open(path_file) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return Err(e);
        }
    };
    let mut buf_reader = BufReader::new(file);
    match buf_reader.read_to_end(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => {
            println!("Error reading file: {}", e);
            Err(e)
        }
    }
}

