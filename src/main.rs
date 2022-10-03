use std::fmt::Write;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};

#[derive(Debug)]
struct HexDoc {
    first_line: String,
    hex_lines: Vec<String>
}

fn create_hexdoc(path :&str) -> HexDoc {
    OpenOptions::new().write(true)
        .create(true)
        .open(path).expect("fail to open or create file");
    let buffer = read_file(path);
    let num_lines = buffer.len() / 16;
    let mut hex_doc = HexDoc { first_line: String::new(), hex_lines: Vec::new() };
    let mut header = String::new();
    //---- imprime la ligne des repères hexa ----
    write!(header, "{}", " ".repeat(10));
    for m in 0..16 {
        write!(header, " {:02X}", m).unwrap()
    }
    hex_doc.first_line.push_str(header.as_str());
    // initialise le buffer pour les chars
    let mut chars_buffer = Vec::new();
    // initialise la string pour les chars
    let mut line_chars=  String::new();
    let mut address = String::new();
    let mut line_bytes = String::new();
    let mut n = 0;
    for b in &buffer {
        //---- imprime l'adresse ----
        if n % 16 == 0 {
            write!(address, " 0x{:06X} ", n);
        }
        //---------------------------
        write!(line_bytes, " {:02X}", b);
        //---- remplace les chars non imprimables -----
        if b > &31 && b < &126 {
            chars_buffer.push(*b);
        } else {
            chars_buffer.push(46);
        }
        //----------------------------------------------
        // met la ligne de chars dans une string
        line_chars = String::from_utf8_lossy(&chars_buffer).to_string();
        n = n + 1;
        if n % 16 == 0 {
            let mut line = String::new();
            write!(line, "{}{}  {}", address, line_bytes, line_chars);
            hex_doc.hex_lines.push(line);
            address = String::new();
            line_bytes = String::new();
            chars_buffer = Vec::new();
        }
    }
    let modu = buffer.len() % 16;
    let mut line = String::new();
    if modu != 0 {
        write!(line, "{}{}  {}{}", address, line_bytes," ".repeat((16-modu) * 3), line_chars);
    }
    hex_doc.hex_lines.push(line);
    hex_doc
}

fn main() {
    const PATH_FILE: &str = "C:\\Users\\Philippe\\rustProjects\\hexa_view\\hello.pdf";
    let hex_doc = create_hexdoc(PATH_FILE);
    println!("{}", hex_doc.first_line);
    for s in &hex_doc.hex_lines {
        println!("{}", s);
    }
    println!("{}", hex_doc.hex_lines.len());
    let a_line = &hex_doc.hex_lines[240];
    println!("{}", get_line_byte(a_line));
}

fn get_line_byte(s :&str) -> String{
    let trim_s = s.replace(" ", "");
    let ret = &trim_s[8..40];
    ret.to_string()
}


// lit les données d'un fichier dans une string
fn read_file(path_file: &str) -> Vec<u8> {
    let mut contents: Vec<u8> = Vec::new();
    let file = File::open(path_file).unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut contents).expect("fail to read file");
    return contents;
}

