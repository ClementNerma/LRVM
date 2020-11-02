//! MRVM uses an assembly language called LASM (Lightweight Assembly).
//! This module allows to assemble LASM source code through the [CustomAsm](https://github.com/hlorenzi/customasm) library.

use crate::asm::{InstrDecodingError, Program};
use crate::bytes::bytes_to_words;
use customasm::asm::Assembler;
use customasm::diagn::RcReport;
use customasm::util::FileServerMock;

static CUSTOMASM_HEADER: &str = include_str!("customasm.def");

/// Assemble a LASM source code to machine code.
/// Returns an error message in case of error.
pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut src = String::from("#include \"header.lasm\"");
    src.push('\n');
    src.push_str(source);

    let mut fileserver = FileServerMock::new();
    fileserver.add("header.lasm", CUSTOMASM_HEADER);
    fileserver.add("src.lasm", src);

    let assemble =
        |report: RcReport, fileserver: &FileServerMock, filename: &str| -> Result<Vec<u8>, ()> {
            let mut asm = Assembler::new();
            asm.register_file(filename);
            let output = asm.assemble(report, fileserver, 10)?;

            Ok(output.binary.format_binary())
        };

    let report = RcReport::new();

    assemble(report.clone(), &fileserver, "src.lasm").map_err(|_| {
        let mut err = vec![];
        report.print_all(&mut err, &fileserver);
        String::from_utf8(err).unwrap()
    })
}

/// Assemble a LASM source code to machine code and split it to words.
/// Returns an error message in case of error.
pub fn assemble_words(source: &str) -> Result<Vec<u32>, String> {
    assemble(source).map(bytes_to_words)
}

/// Convert a LASM source code to a strongly-typed program
/// May fail because Program::decode() may fail if for instance there is raw data in the assembled program (strings for instance)
pub fn assemble_prog(source: &str) -> Result<Result<Program, (usize, InstrDecodingError)>, String> {
    Ok(Program::decode(assemble(source)?))
}
