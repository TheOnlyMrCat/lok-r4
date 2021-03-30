AR = llvm-ar
CC = clang
LEX = flex
RFLEX = rflex
LALRPOP = lalrpop

LEXFLAGS =

all: src/gen/liblexer.a src/gen/parser.rs

src/gen/liblexer.a: src/gen/lexer.c
	$(CC) -c -o src/gen/lexer.o src/gen/lexer.c
	$(AR) rc src/gen/liblexer.a src/gen/lexer.o
	rm -f src/gen/lexer.o

src/gen/lexer.c: src/lexer.l
	$(LEX) -o src/gen/lexer.c $(LEXFLAGS) src/lexer.l

src/gen/lexer.rs: src/lexer.rl
	$(RFLEX) src/lexer.rl
	mv src/lexer.rs src/gen/lexer.rs

src/gen/parser.rs: src/parser.lalrpop
	cd src; \
	$(LALRPOP) --out-dir gen --force parser.lalrpop