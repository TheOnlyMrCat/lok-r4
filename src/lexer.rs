use std::{ffi::CStr, os::raw::{c_char, c_int}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
	Eof,

	Let,
	Const,
	Mut,
	Fn,
	Use,
	Extern,
	Return,
	Yield,
	If,
	Else,
	Static,
	Entry,
	Dyn,

	OpenPar,
	ClosePar,
	OpenBrace,
	CloseBrace,
	OpenBracket,
	CloseBracket,
	Colon,
	DblColon,
	Semicolon,
	Dot,
	TplDot,
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

	Identifier(String),
	Integer(String),
	Float(String),

	LokStaticString(Vec<u8>),
	LokHeapString(Vec<u8>),
	CStaticString(Vec<u8>),
	CHeapString(Vec<u8>),
	ByteStaticString(Vec<u8>),
	ByteHeapString(Vec<u8>),
}

#[allow(dead_code)] // The items are constructed by the next_token function, which rust cannot see
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum TokenDiscriminant {
	Eof,

	Let,
	Const,
	Mut,
	Fn,
	Use,
	Extern,
	Return,
	Yield,
	If,
	Else,
	Static,
	Entry,
	Dyn,

	OpenPar,
	ClosePar,
	OpenBrace,
	CloseBrace,
	OpenBracket,
	CloseBracket,
	Colon,
	DblColon,
	Semicolon,
	Dot,
	TplDot,
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

	LokStaticString,
	LokHeapString,
	CStaticString,
	CHeapString,
	ByteStaticString,
	ByteHeapString,
}

#[derive(Debug)]
#[repr(C)]
pub struct TokenMeta {
	pub token: TokenDiscriminant,
	pub length: u32,
	pub skipped: u32,
}

pub fn lex() -> (Token, u32, u32) {
	let TokenMeta { token, length, skipped } = unsafe { next_token() };
	(
		match token {
			TokenDiscriminant::Identifier => Token::Identifier(unsafe { get_yytext() }),
		    TokenDiscriminant::Integer => Token::Integer(unsafe { get_yytext() }),
		    TokenDiscriminant::Float => Token::Float(unsafe { get_yytext() }),
		    TokenDiscriminant::LokStaticString => Token::LokStaticString(unsafe { get_string_value() }),
		    TokenDiscriminant::LokHeapString => Token::LokHeapString(unsafe { get_string_value() }),
		    TokenDiscriminant::CStaticString => Token::CStaticString(unsafe { get_string_value() }),
		    TokenDiscriminant::CHeapString => Token::CHeapString(unsafe { get_string_value() }),
		    TokenDiscriminant::ByteStaticString => Token::ByteStaticString(unsafe { get_string_value() }),
		    TokenDiscriminant::ByteHeapString => Token::ByteHeapString(unsafe { get_string_value() }),
			
		    TokenDiscriminant::Eof => Token::Eof,
		    TokenDiscriminant::Let => Token::Let,
		    TokenDiscriminant::Const => Token::Const,
		    TokenDiscriminant::Mut => Token::Mut,
		    TokenDiscriminant::Fn => Token::Fn,
		    TokenDiscriminant::Use => Token::Use,
		    TokenDiscriminant::Extern => Token::Extern,
		    TokenDiscriminant::Return => Token::Return,
		    TokenDiscriminant::Yield => Token::Yield,
		    TokenDiscriminant::If => Token::If,
		    TokenDiscriminant::Else => Token::Else,
		    TokenDiscriminant::Static => Token::Static,
		    TokenDiscriminant::Entry => Token::Entry,
		    TokenDiscriminant::Dyn => Token::Dyn,
		    TokenDiscriminant::OpenPar => Token::OpenPar,
		    TokenDiscriminant::ClosePar => Token::ClosePar,
		    TokenDiscriminant::OpenBrace => Token::OpenBrace,
		    TokenDiscriminant::CloseBrace => Token::CloseBrace,
		    TokenDiscriminant::OpenBracket => Token::OpenBracket,
		    TokenDiscriminant::CloseBracket => Token::CloseBracket,
		    TokenDiscriminant::Colon => Token::Colon,
		    TokenDiscriminant::DblColon => Token::DblColon,
		    TokenDiscriminant::Semicolon => Token::Semicolon,
		    TokenDiscriminant::Dot => Token::Dot,
		    TokenDiscriminant::TplDot => Token::TplDot,
		    TokenDiscriminant::Comma => Token::Comma,
		    TokenDiscriminant::Plus => Token::Plus,
		    TokenDiscriminant::Hyphen => Token::Hyphen,
		    TokenDiscriminant::Star => Token::Star,
		    TokenDiscriminant::Slash => Token::Slash,
		    TokenDiscriminant::Percent => Token::Percent,
		    TokenDiscriminant::Equals => Token::Equals,
		    TokenDiscriminant::Greater => Token::Greater,
		    TokenDiscriminant::Less => Token::Less,
		    TokenDiscriminant::SingleArrow => Token::SingleArrow,
		    TokenDiscriminant::DoubleArrow => Token::DoubleArrow,
		},
		length,
		skipped,
	)
}

unsafe fn get_yytext() -> String {
	CStr::from_ptr(yytext).to_str().unwrap().to_owned()
}

unsafe fn get_string_value() -> Vec<u8> {
	stringValue[0..stringLength as usize].to_owned()
}

#[link(name="lexer", link="static")]
extern "C" {
	static yytext: *mut c_char;
	static stringLength: u16;
	static stringValue: [u8; 2048];

	fn next_token() -> TokenMeta;
	pub fn set_input(filename: *const c_char) -> c_int;
}