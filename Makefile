AR = llvm-ar
CC = clang
LEX = flex

src/gen/liblexer.a: src/gen/lexer.c
	$(CC) -c -o src/gen/lexer.o src/gen/lexer.c
	$(AR) rc src/gen/liblexer.a src/gen/lexer.o
	rm -f src/gen/lexer.o

src/gen/lexer.c: src/lexer.l
	$(LEX) -o src/gen/lexer.c src/lexer.l