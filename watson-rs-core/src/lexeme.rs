use crate::{instruction::Instruction, mode::Mode};

pub type LexemeType = char;

// TODO: Complete
pub const MODE_A_REGEX: &str = "";

// TODO: Complete
pub const MODE_B_REGEX: &str = "";

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
    Mode::modes()
        .iter()
        .cloned()
        .map(|mode| (mode, bindings(mode)))
        .collect::<Vec<(Mode, Vec<(LexemeType, Instruction)>)>>()
}

#[cfg(test)]
mod unit_tests {
    use crate::mode::Mode;
    use super::*;

    #[test]
    fn test_mode_lexeme_sequences() {
        assert_eq!(MODE_A_LEXEMES_SEQUENCES.iter().collect::<String>(), String::from("BubaAei'qtp?!~M@szo.E#%"));
        assert_eq!(MODE_S_LEXEMES_SEQUENCES.iter().collect::<String>(), String::from("ShakrAzimbu$-+gv?^!y/e:"));
    }

    #[test]
    fn test_lexeme_sequences() {
        assert_eq!(lexeme_sequences(Mode::A), MODE_A_LEXEMES_SEQUENCES.iter().cloned().collect::<Vec<_>>());
        assert_eq!(lexeme_sequences(Mode::S), MODE_S_LEXEMES_SEQUENCES.iter().cloned().collect::<Vec<_>>());
    }

    #[test]
    fn test_mode_a_bindings() {
        let mode_bindings = vec![
            ('B', Instruction::Inew), 
            ('u', Instruction::Iinc), 
            ('b', Instruction::Ishl), 
            ('a', Instruction::Iadd), 
            ('A', Instruction::Ineg), 
            ('e', Instruction::Isht), 
            ('i', Instruction::Itof),
            ('\'', Instruction::Itou),
            ('q', Instruction::Finf), 
            ('t', Instruction::Fnan), 
            ('p', Instruction::Fneg), 
            ('?', Instruction::Snew), 
            ('!', Instruction::Sadd), 
            ('~', Instruction::Onew), 
            ('M', Instruction::Oadd), 
            ('@', Instruction::Anew), 
            ('s', Instruction::Aadd), 
            ('z', Instruction::Bnew), 
            ('o', Instruction::Bneg),
            ('.', Instruction::Nnew), 
            ('E', Instruction::Gdup), 
            ('#', Instruction::Gpop), 
            ('%', Instruction::Gswp),
        ];

        assert_eq!(bindings(Mode::A), mode_bindings);
    }
    #[test]
    fn test_mode_s_bindings() {
        let mode_bindings = vec![
            ('S', Instruction::Inew), 
            ('h', Instruction::Iinc), 
            ('a', Instruction::Ishl), 
            ('k', Instruction::Iadd), 
            ('r', Instruction::Ineg), 
            ('A', Instruction::Isht), 
            ('z', Instruction::Itof),
            ('i', Instruction::Itou),
            ('m', Instruction::Finf), 
            ('b', Instruction::Fnan), 
            ('u', Instruction::Fneg), 
            ('$', Instruction::Snew), 
            ('-', Instruction::Sadd), 
            ('+', Instruction::Onew), 
            ('g', Instruction::Oadd), 
            ('v', Instruction::Anew), 
            ('?', Instruction::Aadd), 
            ('^', Instruction::Bnew), 
            ('!', Instruction::Bneg),
            ('y', Instruction::Nnew), 
            ('/', Instruction::Gdup), 
            ('e', Instruction::Gpop), 
            (':', Instruction::Gswp),
        ];

        assert_eq!(bindings(Mode::S), mode_bindings);
    }

    #[test]
    fn test_all_bindings() {
        let mode_a_bindings = vec![
            ('B', Instruction::Inew), 
            ('u', Instruction::Iinc), 
            ('b', Instruction::Ishl), 
            ('a', Instruction::Iadd), 
            ('A', Instruction::Ineg), 
            ('e', Instruction::Isht), 
            ('i', Instruction::Itof),
            ('\'', Instruction::Itou),
            ('q', Instruction::Finf), 
            ('t', Instruction::Fnan), 
            ('p', Instruction::Fneg), 
            ('?', Instruction::Snew), 
            ('!', Instruction::Sadd), 
            ('~', Instruction::Onew), 
            ('M', Instruction::Oadd), 
            ('@', Instruction::Anew), 
            ('s', Instruction::Aadd), 
            ('z', Instruction::Bnew), 
            ('o', Instruction::Bneg),
            ('.', Instruction::Nnew), 
            ('E', Instruction::Gdup), 
            ('#', Instruction::Gpop), 
            ('%', Instruction::Gswp),
        ];

        let mode_s_bindings = vec![
            ('S', Instruction::Inew), 
            ('h', Instruction::Iinc), 
            ('a', Instruction::Ishl), 
            ('k', Instruction::Iadd), 
            ('r', Instruction::Ineg), 
            ('A', Instruction::Isht), 
            ('z', Instruction::Itof),
            ('i', Instruction::Itou),
            ('m', Instruction::Finf), 
            ('b', Instruction::Fnan), 
            ('u', Instruction::Fneg), 
            ('$', Instruction::Snew), 
            ('-', Instruction::Sadd), 
            ('+', Instruction::Onew), 
            ('g', Instruction::Oadd), 
            ('v', Instruction::Anew), 
            ('?', Instruction::Aadd), 
            ('^', Instruction::Bnew), 
            ('!', Instruction::Bneg),
            ('y', Instruction::Nnew), 
            ('/', Instruction::Gdup), 
            ('e', Instruction::Gpop), 
            (':', Instruction::Gswp),
        ];

        let all_mode_bindings = vec![ 
            (Mode::A, mode_a_bindings), 
            (Mode::S, mode_s_bindings) 
        ];

        assert_eq!(all_mode_bindings, all_bindings());
    }
}