mod codegen;
mod error;
mod parser;

fn main() {
	let file_path = std::env::args().nth(1).unwrap();
	let (decls, defs) = parser::parse(&file_path).unwrap();
	let module = codegen::lir::Module::from_ast("TODO!".to_owned(), decls, defs);
	let compiler = codegen::Compiler::new();
	compiler.write_module(&compiler.compile_lir_module(module.unwrap()), "todo.o");
}
