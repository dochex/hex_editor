use std::fmt::{Display};

struct HexLine{
    address: u32,
    hex_line: Vec<u8>,
    chars_line: Vec<char>
}

pub struct HexView {
    heading: String,
    hex_lines: Vec<HexLine>,
}

impl HexView {
    fn set_heading(&mut self) -> String {
        let mut s = String::new();
        s.push_str(&" ".repeat(10));
        for x in 0..16 {
            s.push_str(&format!("{:02X} ", x));
        }
        s.trim_end().to_string()
    }

    pub fn new(buffer: &Vec<u8>) -> HexView {
        let mut hex_view = HexView {
            heading: String::new(),
            hex_lines: Vec::new(),
        };
        let mut n: u32 = 0;
        for chunk in buffer.chunks(16) {
            let line = HexLine {
                address: n,
                hex_line: chunk.to_vec(),
                chars_line: chunk.iter().map(|&b| b as char).collect(),
            };
            hex_view.hex_lines.push(line);
            n += 1;
        }
        hex_view.heading = hex_view.set_heading();
        hex_view
    }
}

impl Display for HexView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.heading)?;
        for line in &self.hex_lines {
            write!(f, "0x{:06X} ", line.address)?;
            for byte in &line.hex_line {
                write!(f, " {:02X}", byte)?;
            }
            write!(f, "  ")?;
            if line.hex_line.len()< 17 {
                write!(f, "{}", "   ".repeat(17 - line.hex_line.len()))?;
            }
            for c in &line.chars_line {
                if *c > '\u{1F}' && *c < '\u{7E}' {
                    write!(f, "{}", c)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}