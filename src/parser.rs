use std::iter::Peekable;

use crate::codegen::{ast, lir};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
	End = 0,
	Let,
	Mut,
	Fn,
	Extern,
	Const,
	Return,
	Yield,
	If,
	Else,
	Static,
	Entry,

	OpenPar,
	ClosePar,
	OpenBrace,
	CloseBrace,
	OpenBracket,
	CloseBracket,
	Colon,
	DblColon,
	Semicolon,
	Comma,
	Plus,
	Hyphen,
	Star,
	Slash,
	Percent,
	Equals,
	Greater,
	Less,

	SingleArrow,
	DoubleArrow,

	Identifier,
	Integer,
	Float,
	Invalid = 0x7F,
}

struct TokenStream {
	current_location: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
	ty: TokenType,
	value: String,
	location: std::ops::Range<usize>
}

#[derive(Debug)]
pub enum ParseError {
	IO(std::io::Error),
	Eof,
	/// Integer too big to fit in any of the integer types
	BigInt,
	Unexpected {
		expected: &'static str,
		found: Token,
	},
}

type ParseResult<T> = Result<T, ParseError>;

pub fn parse(file_path: &str) -> Result<(Vec<ast::TopLevelDecl>, Vec<ast::TopLevelDef>), ParseError> {
	let mut input = TokenStream::new(file_path).map_err(|e| ParseError::IO(e))?.peekable();
	let mut decls = vec![];
	let mut defs = vec![];
	loop {
		match input.next() {
			Some(Token { ty: TokenType::Extern, .. }) => {
				match next_expect(&mut input)? {
					Token { ty: TokenType::Fn, .. } => {
						decls.push(ast::TopLevelDecl::FnExtern(fn_extern(&mut input)?));
					},
					Token { ty: TokenType::Static, .. } => {

					},
					t => return Err(ParseError::Unexpected { expected: "`fn` or `static`", found: t }),
				}
			},
			Some(Token { ty: TokenType::Entry, .. }) => {
				let returns = if let Some(Token { ty: TokenType::SingleArrow, .. }) = input.peek() {
					input.next();
					Some(type_(&mut input)?)
				} else {
					None
				};
				next_expect_type(&mut input, TokenType::OpenBrace)?;
				defs.push(ast::TopLevelDef::Entry(ast::Entry {
					returns,
					code: block(&mut input)?,
				}))
			}
			Some(t) => return Err(ParseError::Unexpected { expected: "`extern`", found: t }),
			None => break,
		}
	}
	Ok((decls, defs))
}

fn fn_extern(input: &mut Peekable<TokenStream>) -> ParseResult<ast::FnExtern> {
	let name = next_expect_type(input, TokenType::Identifier)?.value;
	next_expect_type(input, TokenType::OpenPar)?;
	let mut params = vec![];
	loop {
		match peek_expect(input)? {
			Token { ty: TokenType::Star, .. } | Token { ty: TokenType::OpenBracket, .. } | Token { ty: TokenType::Identifier, .. } => params.push((None, type_(input)?)),
			Token { ty: TokenType::ClosePar, .. } => { input.next(); break; },
			_ => Err(ParseError::Unexpected { expected: "type or `)`", found: input.next().unwrap() /* To own the token */ })?,
		}
	}

	Ok(ast::FnExtern {
		name,
		params,
		returns: match next_expect(input)? {
			Token { ty: TokenType::Semicolon, .. } => None,
			Token { ty: TokenType::SingleArrow, .. } => {
				let ty = type_(input)?;
				next_expect_type(input, TokenType::Semicolon)?;
				Some(ty)
			},
			t => Err(ParseError::Unexpected { expected: "`;`", found: t })?
		}
	})
}

fn block(input: &mut Peekable<TokenStream>) -> ParseResult<ast::Block> {
	let mut statements = vec![];
	loop {
		if let Some(Token { ty: TokenType::CloseBrace, .. }) = input.peek() {
			input.next();
			break;
		}
		statements.push(statement(input)?);
		next_expect_type(input, TokenType::Semicolon)?;
	}
	Ok(ast::Block(statements))
}

fn statement(input: &mut Peekable<TokenStream>) -> ParseResult<ast::Statement> {
	Ok(match peek_expect(input)? {
		Token { ty: TokenType::Return, .. } => {
			input.next();
			ast::Statement::Return(match peek_expect(input)? {
				Token { ty: TokenType::Semicolon, .. } => None,
				_ => Some(expression(input)?)
			})
		},
		_ => ast::Statement::Expression(expression(input)?)
	})
}

fn expression(input: &mut Peekable<TokenStream>) -> ParseResult<ast::Expression> {
	//TODO
	Ok(match next_expect(input)? {
		Token { ty: TokenType::Identifier, value, .. } => {
			match peek_expect(input)? {
				Token { ty: TokenType::OpenPar, .. } => {
					input.next();
					let mut arguments = vec![];
					loop {
						match peek_expect(input)? {
							Token { ty: TokenType::Comma, .. } => { input.next(); continue; },
							Token { ty: TokenType::ClosePar, .. } => { input.next(); break; },
							_ => { arguments.push(expression(input)?) },
						}
					}
					ast::Expression::FnCall(qualified_id(value)?, arguments)
				},
				_ => todo!(),
			}
		},
		Token { ty: TokenType::Integer, value, .. } => {
			let val = if let Some('-') = value.chars().next() {
				value.parse::<i64>().map_err(|e| ParseError::BigInt)? as u64
			} else {
				value.parse::<u64>().map_err(|e| ParseError::BigInt)?
			};
			ast::Expression::Int(val)
		},
		t => { dbg!(t); todo!() },
	})
}

fn type_(input: &mut Peekable<TokenStream>) -> ParseResult<ast::Type> {
	match next_expect(input)? {
		Token { ty: TokenType::OpenBracket, .. } => todo!(), // Array/slice
		Token { ty: TokenType::Star, .. } => match next_expect(input)? { // Pointer
			Token { ty: TokenType::Const, .. } => Ok(lir::Type::PtrConst(Box::new(type_(input)?))),
			Token { ty: TokenType::Mut, .. } => Ok(lir::Type::PtrMut(Box::new(type_(input)?))),
			t => Err(ParseError::Unexpected { expected: "`const` or `mut`", found: t }),
		},
		Token { ty: TokenType::Identifier, value, .. } => Ok(lir::Type::Name(qualified_id(value)?)),
		t => Err(ParseError::Unexpected { expected: "type", found: t }),
	}
}

fn qualified_id(input: String) -> ParseResult<ast::Ident> {
	Ok(lir::Ident(input)) //TODO
}

fn next_expect(input: &mut Peekable<TokenStream>) -> ParseResult<Token> {
	input.next().ok_or(ParseError::Eof)
}

fn peek_expect(input: &mut Peekable<TokenStream>) -> ParseResult<&Token> {
	input.peek().ok_or(ParseError::Eof)
}

fn next_expect_type(input: &mut Peekable<TokenStream>, ty: TokenType) -> ParseResult<Token> {
	let next = next_expect(input)?;
	if next.ty == ty {
		Ok(next)
	} else {
		Err(ParseError::Unexpected {
			found: next,
			expected: match ty {
				TokenType::Let => "`let`",
				TokenType::Mut => "`mut`",
				TokenType::Fn => "`fn`",
				TokenType::Extern => "`extern`",
				TokenType::Const => "`const`",
				TokenType::Return => "`return`",
				TokenType::Yield => "`yield`",
				TokenType::If => "`if`",
				TokenType::Else => "`else`",
				TokenType::Static => "`static`",
				TokenType::Entry => "`entry`",

				TokenType::OpenPar => "`(`",
				TokenType::ClosePar => "`)`",
				TokenType::OpenBrace => "`{`",
				TokenType::CloseBrace => "`}`",
				TokenType::OpenBracket => "`[`",
				TokenType::CloseBracket => "`]`",
				TokenType::Colon => "`:`",
				TokenType::DblColon => "`::`",
				TokenType::Semicolon => "`;`",
				TokenType::Comma => "`,`",
				TokenType::Plus => "`+`",
				TokenType::Hyphen => "`-`",
				TokenType::Star => "`*`",
				TokenType::Slash => "`/`",
				TokenType::Percent => "`%`",
				TokenType::Equals => "`=`",
				TokenType::Greater => "`>`",
				TokenType::Less => "`<`",
				
				TokenType::SingleArrow => "`->`",
				TokenType::DoubleArrow => "`=>`",

				TokenType::Identifier => "identifier",
				TokenType::Integer => "integer",
				TokenType::Float => "float",
				TokenType::End | TokenType::Invalid => panic!("Unexpected expected token value"),
			}
		})
	}
}

#[link(name = "lexer", kind = "static")]
extern {
	fn setInput(path: *const std::os::raw::c_char) -> std::os::raw::c_int;
	fn nextToken() -> CToken;
}

impl TokenStream {
	fn new(path: &str) -> std::io::Result<TokenStream> {
		let errno = unsafe { setInput(std::ffi::CString::new(path).unwrap().as_ptr()) };
		if errno != 0 {
			Err(std::io::Error::from_raw_os_error(errno))
		} else {
			Ok(TokenStream {
				current_location: 0,
			})
		}
	}
}

#[repr(C)]
struct CToken {
	ty: TokenType,
	ptr: *const std::os::raw::c_char,
	len: std::os::raw::c_int,
	skipped: std::os::raw::c_int,
}

impl Iterator for TokenStream {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		let CToken { ty, ptr, len, skipped } = unsafe { nextToken() };

		if ty == TokenType::End {
			return None;
		}

		self.current_location += skipped as usize;

		let tk = Token {
			ty,
			value: String::from_utf8(unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) }.to_vec()).unwrap(),
			location: self.current_location..self.current_location + len as usize
		};

		self.current_location += len as usize;

		Some(tk)
	}
}