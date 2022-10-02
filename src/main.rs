
use std::fmt::Write;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::mem::replace;


#[derive(Debug)]
pub struct HexBytes {
    data_bytes :Vec<String>
}

impl HexBytes {
    fn get_byte(mut self, index :usize) -> String{
        let ret = &mut self.data_bytes[index];
        return ret.to_string()
    }
    fn display_hex_bytes(&mut self) -> String{
        let mut ret = String::new();
        for s in &self.data_bytes{
            write!(ret, "{}", s).unwrap()
        }
        return ret;
    }
}

#[derive(Debug)]
struct HexLine {
    address :String,
    hex_bytes :HexBytes,
    data_char :String,
    column :usize
}

#[derive(Debug)]
struct HexDocument {
    first_line :String,
    hex_line :HexLine
}

fn main() {
    const PATH_FILE: &str = "C:\\Users\\Philippe\\rustProjects\\hexa_view\\hello.pdf";
    OpenOptions::new().write(true)
        .create(true)
        .open(PATH_FILE).expect("fail to open or create file");
    let buffer = read_file(PATH_FILE);
    let mut line_bytes = HexBytes {data_bytes :vec![String::new(); 16]};
    let mut line_chars = "   ".to_string();
    let mut hex_line = HexLine {address :String::new(), hex_bytes :line_bytes, data_char :line_chars, column :3};
    let column  = hex_line.hex_bytes.data_bytes.len();
    println!("{}", column);
    let mut header = String::new();
    //---- imprime la ligne des repères hexa ----
    writeln!(header, " {}", " ".repeat(9));
    for m in 0..16 {
        write!(header, " {:02X}",  m).unwrap()
    }
    writeln!(header);
    // initialise le buffer pour les chars
    let mut line_buffer = Vec::new();
    // initialise la string pour les chars
    let mut line_chars=  hex_line.data_char;
    let mut line_bytes =  hex_line.hex_bytes;
    let mut n = 0;
    for b in &buffer {
        //---- imprime l'adresse ----
        if n % 16 == 0{
            print!(" 0x{:06X} ", n);
        }
        //---------------------------
        let mut a_hex_byte = String::new();
        write!(a_hex_byte, " {:02X}", b);
        replace(&mut line_bytes.data_bytes[n%16], a_hex_byte);
        //---- remplace les chars non imprimables -----
        if b > &32 && b < &126{
            line_buffer.push(*b);
        } else {
            line_buffer.push(46);
        }
        //----------------------------------------------
        // met la ligne de chars dans une string
        line_chars =  String::from_utf8_lossy(&line_buffer).to_string();
        n = n + 1;
        if n % 16 == 0 {
            let mut line =  line_bytes.display_hex_bytes();
            line.push_str(line_chars.as_str());
            println!("{}", line);
            //print!("{}", data.display_hex_bytes());
            //println!("{}", b_to_chars);
            line_buffer = Vec::new();
        }
        }
        //---------------------------------------------------------------------------------
    }


// lit les données d'un fichier dans une string
fn read_file(path_file: &str) -> Vec<u8> {
    let mut contents :Vec<u8> = Vec::new();
    let file = File::open(path_file).unwrap();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut contents).expect("fail to read file");
    return contents;
}

