use std::fmt::Write;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::mem;
use std::mem::replace;


#[derive(Debug)]
pub struct HexBytes {
    data_bytes :Vec<String>
}

impl Default for HexBytes {
    fn default () -> HexBytes {
        HexBytes{data_bytes :vec![String::new(); 16]}
    }
}

impl HexBytes {
    fn display_hex_bytes(&self) -> String{
        let mut ret = String::new();
        for s in &self.data_bytes{
            writeln!(ret, "{}", s).unwrap()
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

impl HexLine {
    fn display_doc(&self) -> String{
        let mut all = String::new();
        let mut all = self.hex_bytes.display_hex_bytes();
        return all;
    }
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
    let mut data = HexBytes::default();
    let mut hex_line = HexLine {address :String::new(), hex_bytes :HexBytes::default(), data_char :String::new(), column :3};
    let column  = data.data_bytes.len();
    println!("{}", column);
    let mut header = String::new();
    //let mut hex_line = data.data_bytes;
    //println!("STRING TO READ = \n{}", buffer);
    //---- imprime la ligne des repères hexa ----
    writeln!(header, " {}", " ".repeat(9));
    for m in 0..16 {
        write!(header, " {:02X}",  m).unwrap()
    }
    writeln!(header);
    //-------------------------------------------
    let mut n = 0;
    for b in &buffer {
        //---- imprime l'adresse ----
        /*if n % 16 == 0{
            print!(" 0x{:06X} ", n);
        }
        //---------------------------*/
        let mut a_hex_byte = String::new();
        write!(a_hex_byte, " {:02X}", b);
        mem::replace(&mut data.data_bytes[n], a_hex_byte);
        n = n + 1;
        if n == 16 {
            n= 0;
            for u in &data.data_bytes {
                print!("{}", u);
            }
            println!();
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

