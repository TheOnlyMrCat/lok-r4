use std::ffi::CString;

mod codegen;
mod error;
mod lexer;

use lexer::Token;

#[path="gen/parser.rs"]
mod parser;

fn main() {
	let file_path = std::env::args().nth(1).unwrap();
	let lexer = Lexer::new(&file_path).unwrap();
	let decls = parser::LokFileParser::new().parse(lexer).unwrap();
	let module = codegen::lir::Module::from_ast(codegen::lir::Ident::UnmangledItem("Dunno".to_owned()), decls);
	let compiler = codegen::Compiler::new();
	let compiled_mod = compiler.compile_lir_module(module.unwrap());
	compiler.print_ir(&compiled_mod, "todo.ll");
	compiler.write_module(&compiled_mod, "todo.o");
}

struct Lexer {
	pos: usize,
}

impl Lexer {
	fn new(path: &str) -> std::io::Result<Lexer> {
		let errno = unsafe { lexer::set_input(CString::new(path).unwrap().as_ptr()) };
		if errno > 0 {
			Err(std::io::Error::from_raw_os_error(errno))?
		}
		Ok(Lexer {
			pos: 0,
		})
	}
}

#[derive(Debug)]
pub enum LexError {
	
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'a> Iterator for Lexer {
	type Item = Spanned<Token, usize, LexError>;

	fn next(&mut self) -> Option<Spanned<Token, usize, LexError>> {
		let (token, _len, _skipped) = lexer::lex();
		match token {
			Token::Eof => None,
			token => Some(Ok((self.pos, token, self.pos))) //TODO: Position information
		}
	}
}