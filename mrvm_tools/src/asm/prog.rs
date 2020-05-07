use super::{Instr, InstrDecodingError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Program(pub Vec<Instr>);

impl Program {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from(instr: Vec<Instr>) -> Self {
        Self(instr)
    }

    pub fn instr(&self) -> impl Iterator<Item = &Instr> {
        self.0.iter()
    }

    pub fn prepend(&mut self, instr: Instr) -> &mut Self {
        self.0.insert(0, instr);
        self
    }

    pub fn prepend_all(&mut self, instr: impl AsRef<[Instr]>) -> &mut Self {
        self.0.splice(0..0, instr.as_ref().iter().copied());
        self
    }

    pub fn append(&mut self, instr: Instr) -> &mut Self {
        self.0.push(instr);
        self
    }

    pub fn append_all(&mut self, instr: impl AsRef<[Instr]>) -> &mut Self {
        self.0.extend_from_slice(instr.as_ref());
        self
    }

    pub fn decode(prog: impl AsRef<[u8]>) -> Result<Self, (usize, InstrDecodingError)> {
        let prog = prog.as_ref();

        if prog.len() % 4 != 0 {
            return Err((0, InstrDecodingError::SourceNotMultipleOf4Bytes));
        }

        let mut out = vec![];

        for i in 0..prog.len() / 4 {
            match Instr::decode([
                prog[i * 4],
                prog[i * 4 + 1],
                prog[i * 4 + 2],
                prog[i * 4 + 3],
            ]) {
                Ok(instr) => out.push(instr),
                Err(err) => return Err((i, err)),
            }
        }

        Ok(Self::from(out))
    }

    pub fn to_folded_bytes(&self) -> Vec<[u8; 4]> {
        self.0.iter().map(|instr| instr.encode()).collect()
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = vec![];

        for instr in &self.0 {
            out.extend_from_slice(&instr.encode());
        }

        out
    }

    pub fn encode_words(&self) -> Vec<u32> {
        self.0.iter().map(|instr| instr.encode_word()).collect()
    }
}
