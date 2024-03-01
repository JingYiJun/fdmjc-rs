mod ast;
mod dsa;
mod llvm;
mod parser;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use ast::Program;
use parser::FDMJParser;
use parser::Rule;
use pest::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(format!("Usage: {}", args[0]).into());
    }
    let file_source = &args[1];
    let file_raws = file_source.trim_end_matches(".fdmj");
    let file_ll = format!("{file_raws}.ll");
    let mut program_source = String::new();
    File::open(file_source)?.read_to_string(&mut program_source)?;
    

    let pair = FDMJParser::parse(Rule::Program, &mut program_source)?.next().unwrap();
    let program = Program::parse(pair);
    let mut s = String::new();
    program.generate(&mut s);

    // generate .ll file
    let mut file_ll = File::create(file_ll)?;
    file_ll.write_all(b"define i64 @main() {\n")?;
    file_ll.write_all(s.as_bytes())?;
    file_ll.write_all(
        b"ret i64 0
}

declare ptr @malloc(i64)
declare void @putint(i64)
declare void @putch(i64)
",
    )?;
    Ok(())
}
