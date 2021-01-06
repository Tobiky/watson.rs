#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub enum Instruction {
    Inew,
    Iinc,
    Ishl,
    Iadd,
    Ineg,
    Isht,
    Itof,
    Itou,
    Finf,
    Fnan,
    Fneg,
    Snew,
    Sadd,
    Onew,
    Oadd,
    Anew,
    Aadd,
    Bnew,
    Bneg,
    Nnew,
    Gdup,
    Gpop,
    Gswp,
}

impl Instruction {
    pub fn create_vector() -> Vec<Instruction> {
        vec![
            Instruction::Inew,
            Instruction::Iinc,
            Instruction::Ishl,
            Instruction::Iadd,
            Instruction::Ineg,
            Instruction::Isht,
            Instruction::Itof,
            Instruction::Itou,
            Instruction::Finf,
            Instruction::Fnan,
            Instruction::Fneg,
            Instruction::Snew,
            Instruction::Sadd,
            Instruction::Onew,
            Instruction::Oadd,
            Instruction::Anew,
            Instruction::Aadd,
            Instruction::Bnew,
            Instruction::Bneg,
            Instruction::Nnew,
            Instruction::Gdup,
            Instruction::Gpop,
            Instruction::Gswp,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;

    #[test]
    fn create_vector(){
        let instruction_vector =  crate::instruction::Instruction::create_vector();
        let mut iterator = instruction_vector.iter();

        assert!(matches!(iterator.next(), Some(Instruction::Inew)));
        assert!(matches!(iterator.next(), Some(Instruction::Iinc)));
        assert!(matches!(iterator.next(), Some(Instruction::Ishl)));
        assert!(matches!(iterator.next(), Some(Instruction::Iadd)));
        assert!(matches!(iterator.next(), Some(Instruction::Ineg)));
        assert!(matches!(iterator.next(), Some(Instruction::Isht)));
        assert!(matches!(iterator.next(), Some(Instruction::Itof)));
        assert!(matches!(iterator.next(), Some(Instruction::Itou)));
        assert!(matches!(iterator.next(), Some(Instruction::Finf)));
        assert!(matches!(iterator.next(), Some(Instruction::Fnan)));
        assert!(matches!(iterator.next(), Some(Instruction::Fneg)));
        assert!(matches!(iterator.next(), Some(Instruction::Snew)));
        assert!(matches!(iterator.next(), Some(Instruction::Sadd)));
        assert!(matches!(iterator.next(), Some(Instruction::Onew)));
        assert!(matches!(iterator.next(), Some(Instruction::Oadd)));
        assert!(matches!(iterator.next(), Some(Instruction::Anew)));
        assert!(matches!(iterator.next(), Some(Instruction::Aadd)));
        assert!(matches!(iterator.next(), Some(Instruction::Bnew)));
        assert!(matches!(iterator.next(), Some(Instruction::Bneg)));
        assert!(matches!(iterator.next(), Some(Instruction::Nnew)));
        assert!(matches!(iterator.next(), Some(Instruction::Gdup)));
        assert!(matches!(iterator.next(), Some(Instruction::Gpop)));
        assert!(matches!(iterator.next(), Some(Instruction::Gswp)));
    }
}