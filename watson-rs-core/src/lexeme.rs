use crate::{instruction::Instruction, mode::Mode};

pub type LexemeType = char;

// TODO: Move to configuration file
pub static MODE_A_LEXEMES_SEQUENCES: [LexemeType; 23] = [
    'B', 'u', 'b', 'a', 'A', 'e', 'i', '\'', 'q', 't', 'p', '?', '!', '~', 'M', '@', 's', 'z', 'o',
    '.', 'E', '#', '%',
];

pub static MODE_S_LEXEMES_SEQUENCES: [LexemeType; 23] = [
    'S', 'h', 'a', 'k', 'r', 'A', 'z', 'i', 'm', 'b', 'u', '$', '-', '+', 'g', 'v', '?', '^', '!',
    'y', '/', 'e', ':',
];

// TODO: Create Lexeme struct to ease communication between other functions and structs

#[allow(unreachable_patterns)]
pub fn lexeme_sequences(mode: Mode) -> Vec<LexemeType> {
    match mode {
        Mode::A => MODE_A_LEXEMES_SEQUENCES.to_vec(),
        Mode::S => MODE_S_LEXEMES_SEQUENCES.to_vec(),
        _ => {
            panic!("attempt of getting unsupported mode's lexeme sequences")
        }
    }
}

// TODO: return references instead to avoid copying
// TODO: Make sure this function is in appropriate place
pub fn bindings(mode: Mode) -> Vec<(LexemeType, Instruction)> {
    lexeme_sequences(mode)
        .iter()
        .cloned()
        .zip(Instruction::create_vector())
        .collect()
} 

// TODO: return references instead to avoid copying
pub fn all_bindings() -> Vec<(Mode, Vec<(LexemeType, Instruction)>)> {
    Mode::modes().iter()
                 .cloned()
                 .map(|mode| 
                    (mode, bindings(mode)))
                 .collect::<Vec<(Mode, Vec<(LexemeType, Instruction)>)>>()
}