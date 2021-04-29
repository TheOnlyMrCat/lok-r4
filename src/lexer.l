%{
#include <errno.h>
#include <stdio.h>
#include <stdint.h>

enum Token {
	Token_Eof,

	Token_Let,
	Token_Const,
	Token_Mut,
	Token_Fn,
	Token_Use,
	Token_Extern,
	Token_Return,
	Token_Yield,
	Token_If,
	Token_Else,
	Token_Static,
	Token_Entry,
	Token_Dyn,

	Token_OpenPar,
	Token_ClosePar,
	Token_OpenBrace,
	Token_CloseBrace,
	Token_OpenBracket,
	Token_CloseBracket,
	Token_Colon,
	Token_DblColon,
	Token_Semicolon,
	Token_Dot,
	Token_TplDot,
	Token_Comma,
	Token_Plus,
	Token_Hyphen,
	Token_Star,
	Token_Slash,
	Token_Percent,
	Token_Equals,
	Token_Greater,
	Token_Less,

	Token_SingleArrow,
	Token_DoubleArrow,

	Token_Identifier,
	Token_Integer,
	Token_Float,
	Token_LokStaticString,
	Token_LokHeapString,
	Token_CStaticString,
	Token_CHeapString,
	Token_ByteStaticString,
	Token_ByteHeapString,
};

char readHex();
void extendString();

enum Token stringType;
uint16_t stringLength; //TODO: Error checking around length of string
char stringValue[2048];
%}
%option noyywrap
%x STRING
%%

"//".*\n   /* End of line comment */
"/*" { // Modified from http://dinosaur.compilertools.net/flex/flex_9.html
	register int c;

	for (;;) {
		while ((c = input()) != '*' && c != EOF );    /* eat up text of comment */

		if ( c == '*' ) {
			while ((c = input()) == '*');
			if (c == '/')
				break;    /* found the end */
		}
		if ( c == EOF ) {
			return Token_Eof; //TODO: use special token
		}
	}
}

let return Token_Let;
const return Token_Const;
mut return Token_Mut;
dyn return Token_Dyn;
fn return Token_Fn;
use return Token_Use;
extern return Token_Extern;
return return Token_Return;
yield return Token_Yield;
if return Token_If;
else return Token_Else;
static return Token_Static;
entry return Token_Entry;

[[:alpha:]_][[:alnum:]_]* return Token_Identifier;

"-"?[[:digit:]_]+ return Token_Integer;
[[:digit:]_]+"."[[:digit:]_]+ return Token_Float;

\"  BEGIN STRING; stringLength = 0; stringType = Token_LokStaticString;
c\" BEGIN STRING; stringLength = 0; stringType = Token_CStaticString;
b\" BEGIN STRING; stringLength = 0; stringType = Token_ByteStaticString;

<STRING>\" BEGIN INITIAL; return stringType;
<STRING>[^\"\\]+ extendString();
<STRING>[^\"\\]+\" extendString(); /* Remove trailing quote */ stringLength--; BEGIN INITIAL; return stringType;
<STRING>\\ {
	register char c = input();
	register char i = 0;
	switch (c) {
	case '0':
		c = 0x0;
		break;
	case 'a':
		c = 0x7;
		break;
	case 'b':
		c = 0x8;
		break;
	case 'e':
		c = 0x1b;
		break;
	case 'f':
		c = 0xc;
		break;
	case 'n':
		c = 0xa;
		break;
	case 'r':
		c = 0xd;
		break;
	case 't':
		c = 0x9;
		break;
	case 'v':
		c = 0xb;
		break;
	case 'x':
		i = readHex();
		if (i == 0x10) {
			return Token_Eof; //TODO: special "invalid" token
		}
		c = i << 4;
		i = readHex();
		if (i == 0x10) {
			return Token_Eof; //TODO: special "invalid" token
		}
		c |= i;
		break;
	//TODO: Larger unicode (UTF-8?) characters
	}
	stringValue[stringLength] = c;
	stringLength++;
}

"(" return Token_OpenPar;
")" return Token_ClosePar;
"{" return Token_OpenBrace;
"}" return Token_CloseBrace;
"[" return Token_OpenBracket;
"]" return Token_CloseBracket;
":" return Token_Colon;
"::" return Token_DblColon;
";" return Token_Semicolon;
"." return Token_Dot;
"..." return Token_TplDot;
"," return Token_Comma;
"+" return Token_Plus;
"-" return Token_Hyphen;
"*" return Token_Star;
"/" return Token_Slash;
"%" return Token_Percent;
"=" return Token_Equals;
">" return Token_Greater;
"<" return Token_Less;

"->" return Token_SingleArrow;
"=>" return Token_DoubleArrow;

<<EOF>> return Token_Eof;
[\0-\ ] /* Skip */
%%
//TODO: Catch-all error token

char readHex() {
	register char i = input();
	if (i >= '0' && i <= '9') {
		return i - '0';
	} else if (i >= 'A' && i <= 'F') {
		return i - 'A';
	} else if (i >= 'a' && i <= 'f') {
		return i - 'a';
	}
	return 0x10;
}

void extendString() {
	memcpy(stringValue + stringLength, yytext, yyleng);
	stringLength += yyleng;
}

// -------- Rust FFI Interface --------
struct TokenMeta {
	enum Token type;
	uint32_t leng;
	uint32_t skipped;
};


int set_input(const char *filename) {
	FILE *file = fopen(filename, "r");
	if (file) {
		yyin = file;
		return 0;
	} else {
		return errno;
	}
}

struct TokenMeta next_token() {
	enum Token tok = yylex();
	struct TokenMeta meta = {
		.type = tok,
		.leng = yyleng,
		.skipped = 0, //TODO
	};
	return meta;
}