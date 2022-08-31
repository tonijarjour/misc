use std::fs::read_to_string;

struct InputFile<'a> {
    words: u32,
    lines: u32,
    characters: u32,
    name: &'a str,
}

impl<'a> InputFile<'a> {
    fn new(name: &'a str) -> Result<Self, std::io::Error> {
        let content = read_to_string(name)?;
        let (mut characters, mut words, mut lines) = (0, 0, 0);
        let mut spaced: bool = false;
        for c in content.chars() {
            if c as u8 != 0 {
                characters += 1;
            }
            if c != ' ' && c != '\n' {
                spaced = false
            }
            if c == '\n' {
                lines += 1;
                if !spaced {
                    words += 1;
                    spaced = true;
                }
            }
            if c == ' ' && !spaced {
                words += 1;
                spaced = true;
            }
        }
        Ok( Self { lines, words, characters, name } )
    }
}

impl std::fmt::Display for InputFile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {} {}",
            self.lines, self.words, self.characters, self.name)
    }
}

fn main() {
    for f in std::env::args().skip(1) {
        match InputFile::new(&f) {
            Ok(parsed) => println!("{}", parsed),
            Err(_) => println!("{}: could not open", f),
        }
    }
}
