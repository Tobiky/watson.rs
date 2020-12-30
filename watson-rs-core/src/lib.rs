pub mod instructions;
pub mod types;
pub mod mode;
pub mod state;

use mode::Mode;

pub type LexemeType = char;

// TODO: Move to configuration file
pub static MODE_A_LEXEMES_CHARACTERS: [LexemeType; 23] = [
    'B', 'u', 'b', 'a', 'A', 'e', 'i', '\'', 'q', 't', 'p', '?', '!', '~', 'M', '@', 's', 'z', 'o',
    '.', 'E', '#', '%',
];

pub static MODE_S_LEXEMES_CHARACTERS: [LexemeType; 23] = [
    'S', 'h', 'a', 'k', 'r', 'A', 'z', 'i', 'm', 'b', 'u', '$', '-', '+', 'g', 'v', '?', '^', '!',
    'y', '/', 'e', ':',
];

#[allow(unreachable_patterns)]
pub fn lexeme_sequences(mode: Mode) -> Vec<LexemeType> {   
    match mode {
        Mode::A => MODE_A_LEXEMES_CHARACTERS.to_vec(),
        Mode::S => MODE_S_LEXEMES_CHARACTERS.to_vec(),
        _ => {
            panic!("attempt of getting unsupported mode's lexeme sequences")
        }
    }
}