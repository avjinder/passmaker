use clap::Parser;

const UPPERCASE_LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS_LETTERS: &'static str = "0123456789";
const SYMBOLS_LETTERS: &'static str = "~!@#$%^&*()[]{}-_/\\><+,;:=|";

#[derive(Debug, Parser)]
pub struct PassConfig {
    /// password length
    #[arg(short, long, default_value_t = 24)]
    pub length: usize,

    /// allow uppercase letters
    #[arg(short, long, default_value_t = false)]
    pub upper: bool,

    /// allow lowercase letters
    #[arg(long, default_value_t = true)]
    pub lower: bool,

    /// allow symbols
    #[arg(short, long, default_value_t = false)]
    pub symbols: bool,

    /// allow numbers
    #[arg(short, long, default_value_t = true)]
    pub numbers: bool,

    /// number of passwords to generate
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,

    #[clap(skip=String::from(UPPERCASE_LETTERS))]
    pub uppercase_letters: String,

    #[clap(skip=String::from(LOWERCASE_LETTERS))]
    pub lowercase_letters: String,

    #[clap(skip=String::from(NUMBERS_LETTERS))]
    pub numbers_letters: String,

    #[clap(skip=String::from(SYMBOLS_LETTERS))]
    pub symbols_letters: String,
}

impl PassConfig {
    pub fn valid_letters(&self) -> Vec<char> {
        let mut valid: Vec<char> = vec![];
        if self.upper {
            valid.extend(self.uppercase_letters.chars());
        }
        if self.lower {
            valid.extend(self.lowercase_letters.chars());
        }
        if self.numbers {
            valid.extend(self.numbers_letters.chars());
        }
        if self.symbols {
            valid.extend(self.symbols_letters.chars());
        }

        valid
    }
}

impl Default for PassConfig {
    fn default() -> Self {
        PassConfig {
            length: 24,
            upper: false,
            lower: true,
            symbols: false,
            numbers: true,
            count: 1,
            uppercase_letters: String::from(UPPERCASE_LETTERS),
            lowercase_letters: String::from(LOWERCASE_LETTERS),
            numbers_letters: String::from(NUMBERS_LETTERS),
            symbols_letters: String::from(SYMBOLS_LETTERS),
        }
    }
}
