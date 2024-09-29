mod hexview;


use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::Path;
use std::process::exit;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};


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

fn handle_user_action(selection: usize, items: &[&str], buffer: &mut Vec<u8>, path_file: &str) {
    if items[selection] == "read a byte" {
        read_byte(buffer);
    } else if items[selection] == "change some bytes" {
        change_bytes(buffer, path_file);
    } else if items[selection] == "exit" {
        exit(0);
    }
}

fn main() {
    let path_file = get_file_path();
    let mut buffer = read_file(&path_file);
    let view = hexview::HexView::new(&buffer);
    println!("{}", format!("{view}"));

    let items = vec!["read a byte", "change some bytes", "exit"];
    loop {
        let selection = display_menu(&items);
        handle_user_action(selection, &items, &mut buffer, &path_file);
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
                    Ok(_) => Ok(()),
                    Err(_) => Err("This is not an hexadecimal string")
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
    println!("{}", format!("{view}"));
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to save the change?")
        .default(true)
        .show_default(false)
        .interact()
        .unwrap()
    {
        fs::write(Path::new(&path_file.replacen(".pdf", "-new.pdf", 1)), buffer).expect("Unable to write file");
        println!("the new file is created");
    } else {
        println!("nevermind then :(");
    }
}
fn read_byte(buffer: &Vec<u8>) -> (usize, &u8){
    let byte_to_read: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the address of the byte to read")
        .interact_text()
        .unwrap();
    let byte_to_int = i64::from_str_radix(&byte_to_read, 16).expect("bad hex string address");
    let byte_int = buffer.get(byte_to_int as usize);
    println!("The byte at address '{}' is '{:?}'", byte_to_read, byte_int.unwrap());
    println!();
    (byte_to_int as usize, byte_int.unwrap())
}

// lit les données d'un fichier dans une string
fn read_file(path_file: &str) -> Vec<u8> {
    let mut contents: Vec<u8> = Vec::new();
    let file = File::open(path_file).unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut contents).expect("fail to read file");
    contents
}
