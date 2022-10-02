use std::fmt::Write;

#[derive(Debug)]
pub struct HexBytes {
    data_bytes :Vec<String>
}

impl Default for HexBytes {
    fn default () -> HexBytes {
        HexBytes{data_bytes :vec![String::new(); 16]}
    }
}

#[derive(Debug, Default)]
struct Document {
    first_line :String,
    address :Vec<String>,
    hex_bytes :HexBytes,
    data_char :Vec<String>,
    column :usize
}

impl Document {
    fn default () -> Document {
        Document{first_line :String::new(), address :vec![String::new(); 8], hex_bytes :HexBytes::default(), data_char :vec![String::new(); 18], column :3}
    }
    fn display_doc(&self) -> String{
        let mut all = String::new();
        writeln!(all, "{}",self.first_line);
        return all;
    }
}

fn main() {
    let mut data = HexBytes::default();
    let column  = data.data_bytes.len();
    println!("{}", column);

}

