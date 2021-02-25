%{
#include <stdio.h>

enum TokenType {
	END = 0,
	LET,
	MUT,
	FN,
	EXTERN,
	CONST,
	RETURN,
	YIELD,
	IF,
	ELSE,
	STATIC,
	ENTRY,

	OPENPAR,
	CLOSEPAR,
	OPENBRACE,
	CLOSEBRACE,
	OPENBRACKET,
	CLOSEBRACKET,
	COLON,
	DBLCOLON,
	SEMICOLON,
	COMMA,
	PLUS,
	HYPHEN,
	STAR,
	SLASH,
	PERCENT,
	EQUALS,
	GREATER,
	LESS,

	SINGLEARROW,
	DOUBLEARROW,

	IDENTIFIER,
	INTEGER,
	FLOAT,
	INVALID = 0x7F,
};

int skippedBytes;
%}

%option noyywrap
%option nostdinit
%option nodefault

%x COMMENT
%%
	skippedBytes = 0;

"//".* skippedBytes += yyleng;
"/*" skippedBytes += yyleng; BEGIN(COMMENT);
<COMMENT>[^*\n]*        skippedBytes += yyleng;
<COMMENT>[^*\n]*\n      skippedBytes += yyleng;
<COMMENT>"*"+[^*/\n]*   skippedBytes += yyleng;
<COMMENT>"*"+[^*/\n]*\n skippedBytes += yyleng;
<COMMENT>"*"+"/"        skippedBytes += yyleng; BEGIN(INITIAL);

let return LET;
mut return MUT;
fn return FN;
extern return EXTERN;
const return CONST;
return return RETURN;
yield return YIELD;
if return IF;
else return ELSE;
static return STATIC;
entry return ENTRY;

[[:alpha:]_][[:alnum:]_]* return IDENTIFIER;

"-"?[[:digit:]_]+ return INTEGER;
[[:digit:]_]+"."[[:digit:]_]+ return FLOAT;

"(" return OPENPAR;
")" return CLOSEPAR;
"{" return OPENBRACE;
"}" return CLOSEBRACE;
"[" return OPENBRACKET;
"]" return CLOSEBRACKET;
":" return COLON;
"::" return DBLCOLON;
";" return SEMICOLON;
"," return COMMA;
"+" return PLUS;
"-" return HYPHEN;
"*" return STAR;
"/" return SLASH;
"%" return PERCENT;
"=" return EQUALS;
">" return GREATER;
"<" return LESS;

"->" return SINGLEARROW;
"=>" return DOUBLEARROW;

<<EOF>>  return END;
[ \t\n]+ skippedBytes += yyleng;
.        return INVALID;

%%

typedef struct {
	enum TokenType type;
	const char *textPtr;
	int textLeng;
	int skippedBytes;
} Token;

int setInput(const char *path) {
	FILE *file = fopen(path, "r");
	if (file) {
		yyin = file;
		return 0;
	} else {
		return errno;
	}
}

Token nextToken() {
	enum TokenType type = yylex();
	Token tk = {
		.type = type,
		.textPtr = yytext,
		.textLeng = yyleng,
		.skippedBytes = skippedBytes,
	};
	return tk;
}