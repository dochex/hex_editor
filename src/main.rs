mod hexview;

use std::fs::{File};
use std::io::{BufReader, Read};
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};


fn main() {
    let path_file: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Entrer le chemin du fichier à ouvrir")
        .default("C:/Users/Philippe/rustProjects/hexa_view/hello.pdf".to_string())
        .interact_text()
        .unwrap();

    /*OpenOptions::new().write(true)
        .create(true)
        .open(PATH_FILE).expect("fail to open or create file");*/
    let mut buffer = read_file(&path_file);
    let view = hexview::HexView::new(buffer.clone());
    println!("{}", format!("{view}"));

    let items = vec!["read a byte", "change a byte"];
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();

    for i in selection {
        if items[i] == "read a byte" {
            read_byte(&buffer);
        } else if items[i] == "change a byte" {
            change_byte(&mut buffer);
        }
    }

}

fn change_byte(buffer: &mut Vec<u8>) {
    let binding = buffer.clone();
    let result = read_byte(&binding);
    let new_byte: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the byte for change")
        .interact_text()
        .unwrap();
    let new_byte_int = i64::from_str_radix(&new_byte, 16).expect("bad hex string in input");
    buffer[result.0] = new_byte_int as u8;
    let view = hexview::HexView::new(buffer.clone());
    println!("{}", format!("{view}"));
}

fn read_byte(buffer: &Vec<u8>) -> (usize, &u8){
    let byte_to_read: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the address of the byte to write")
        .interact_text()
        .unwrap();
    let byte_to_int = i64::from_str_radix(&byte_to_read, 16).expect("bad hex string address");
    let byte_int = buffer.get(byte_to_int as usize);
    println!("The byte at address '{}' is '{:?}'", byte_to_read, byte_int.unwrap());
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

