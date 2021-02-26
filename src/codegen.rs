pub mod ast {
	use super::lir;

	#[derive(Debug)]
	pub enum TopLevelDecl {
		FnExtern(FnExtern),
	}

	#[derive(Debug)]
	pub enum TopLevelDef {
		Entry(Entry),
	}

	#[derive(Debug)]
	pub struct FnExtern {
		pub name: String,
		pub params: Vec<(Option<String>, Type)>,
		pub returns: Option<Type>,
	}

	#[derive(Debug)]
	pub struct Entry {
		pub returns: Option<Type>,
		pub code: Block,
	}

	#[derive(Debug)]
	pub struct Block(pub Vec<Statement>);

	#[derive(Debug)]
	pub enum Statement {
		Expression(Expression),
		Return(Option<Expression>),
	}

	#[derive(Debug)]
	pub enum Expression {
		FnCall(Ident, Vec<Expression>),
		Var(Ident),
		Int(u64),
	}

	pub type Type = lir::Type;
	pub type Ident = lir::Ident;
}

pub mod lir {
	use crate::error::{LIRError, LIRErrorType};

	use super::ast;
	use super::NameResolveMap;

	pub struct Module {
		pub name: String,
		pub decls_fn: Vec<DeclFn>,
		pub defs_fn: Vec<DefFn>,
		pub entry: Option<DefEntry>,
	}

	pub struct DeclFn {
		pub id: Ident,
		pub params: Vec<(String, Type)>,
		pub returns: Type,
	}

	pub struct DefFn {
		pub id: Ident,
		pub body: FnBody,
	}

	pub struct DefEntry {
		pub returns: Option<Type>,
		pub body: FnBody,
	}

	pub struct FnBody {
		pub decls: Vec<Decl>,
		pub block: Block,
	}

	pub struct Block {
		pub statements: Vec<Statement>,
	}

	pub enum Statement {
		Decl(Decl, Expression),
		Eval(Expression),
		Return(Option<Expression>),
	}

	pub struct Expression {
		pub ty: Type,
		pub value: ExpressionValue
	}

	pub enum ExpressionValue {
		Call(Ident, Vec<Expression>),
		Load(Ident),
		ConstInt(u64),
	}

	pub struct Decl {
		pub mutable: bool,
		pub name: String,
		pub ty: Type,
	}

	#[derive(Debug, Clone)]
	pub enum Type { // copied by ast::Type
		Name(Ident),
		PtrConst(Box<Type>),
		PtrMut(Box<Type>),
		PtrDynConst(Box<Type>),
		PtrDynMut(Box<Type>),
		Arr(Box<Type>, Option<usize>),
		Tuple(Vec<Type>),
		Void, //TODO: Decide whether this is necessary
	}

	#[derive(Debug, Clone, Hash, PartialEq, Eq)]
	pub struct Ident(pub String); // copied by ast::Ident

	impl Module {
		pub fn from_ast(name: String, tl_decls: Vec<ast::TopLevelDecl>, tl_defs: Vec<ast::TopLevelDef>) -> Result<Module, LIRError> {
			let mut decls_fn = vec![];
	
			for decl in tl_decls {
				match decl {
					ast::TopLevelDecl::FnExtern(f) => {
						decls_fn.push(DeclFn {
							id: Ident(f.name),
							params: f.params.into_iter().map(|(s, t)| (match s { Some(s) => s, None => "".to_owned() }, t)).collect::<Vec<_>>(),
							returns: f.returns.unwrap_or(Type::Void),
						})
					},
					_ => {}
				}
			}

			use std::collections::HashMap;

			let mut name_resolve = NameResolveMap {
				local_fns: decls_fn.iter().map(|decl| (decl.id.clone(), decl)).collect::<HashMap<_, _>>(),
				scope_stack: vec![],
			};

			let mut defs_fn = vec![];
			let mut entry = None;
			for def in tl_defs {
				match def {
					ast::TopLevelDef::Entry(e) => {
						assert!(entry.is_none(), "Multiple entry points declared!");
						entry = Some(DefEntry {
							returns: e.returns,
							body: FnBody::from_ast(e.code, &mut name_resolve)?,
						})
					},
					_ => {}
				}
			}
	
			Ok(Module {
				name,
				decls_fn,
				defs_fn,
				entry,
			})
		}
	}
	
	impl FnBody {
		fn from_ast(ast::Block(block): ast::Block, name_resolve: &mut NameResolveMap<'_>) -> Result<FnBody, LIRError> {
			let mut decls = vec![];
			let mut statements = vec![];
	
			for statement in block {
				match statement {
					ast::Statement::Expression(e) => {
						statements.push(Statement::Eval(Expression::from_ast(e, name_resolve)?))
					},
					ast::Statement::Return(e) => {
						statements.push(Statement::Return(e.map(|e| Expression::from_ast(e, name_resolve)).transpose()?))
					}
				}
			}
	
			Ok(FnBody {
				decls,
				block: Block {
					statements,
				}
			})
		}
	}
	
	impl Expression {
		fn from_ast(expression: ast::Expression, name_resolve: &mut NameResolveMap<'_>) -> Result<Expression, LIRError> {
			Ok(match expression {
				ast::Expression::FnCall(f, a) => {
					Expression {
						ty: name_resolve.resolve_fn_default(&f).ok_or(LIRError { ty: LIRErrorType::UnresolvedIdent })?,
						value: ExpressionValue::Call(f, a.into_iter().map(|e| Expression::from_ast(e, name_resolve)).collect::<Result<_, _>>()?),
					}
				},
				ast::Expression::Var(i) => {
					Expression {
						ty: name_resolve.resolve_var_default(&i).ok_or(LIRError { ty: LIRErrorType::UnresolvedIdent })?,
						value: ExpressionValue::Load(i),
					}
				},
				ast::Expression::Int(i) => {
					Expression {
						ty: integer_type_for_value(i),
						value: ExpressionValue::ConstInt(i)
					}
				}
			})
		}
	}

	fn integer_type_for_value(value: u64) -> Type {
		Type::Name(Ident("i32".to_owned())) //TODO
	}
}

use std::collections::HashMap;

struct StackScope<'a> {
	vars: HashMap<lir::Ident, &'a lir::Decl>,
}

struct NameResolveMap<'a> {
	local_fns: HashMap<lir::Ident, &'a lir::DeclFn>,
	scope_stack: Vec<StackScope<'a>>,
}

impl NameResolveMap<'_> {
	fn resolve_fn_default(&self, id: &lir::Ident) -> Option<lir::Type> {
		match self.local_fns.get(id) {
			Some(decl) => Some(decl.returns.clone()),
			None => None,
		}
	}

	fn resolve_var_default(&self, id: &lir::Ident) -> Option<lir::Type> {
		None
	}
}

//MARK: Compiler
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::basic_block::BasicBlock;
use inkwell::module::{Module, Linkage};
use inkwell::targets::{Target, TargetMachine, RelocMode, CodeModel, FileType, InitializationConfig};
use inkwell::types::{BasicType, StructType, BasicTypeEnum};
use inkwell::values::{FunctionValue, PointerValue, StructValue, BasicValueEnum};
use inkwell::{AddressSpace, OptimizationLevel};

pub struct Compiler {
	llvm: Context,
	target: Target,
	machine: TargetMachine,
}

impl Compiler {
	pub fn new() -> Compiler {
		Compiler::with_context(Context::create())
	}

	pub fn with_context(context: Context) -> Compiler {
		Target::initialize_all(&InitializationConfig {
			asm_printer: true,
			asm_parser: true,
			base: true,
			disassembler: false,
			info: true,
			machine_code: true,
		});
		let triple = TargetMachine::get_default_triple();
		let target = Target::from_triple(&triple).unwrap();
		println!("triple: {}", triple);
		println!("features: {}", &TargetMachine::get_host_cpu_features().to_string());
		let machine = target.create_target_machine(&triple, "generic", &TargetMachine::get_host_cpu_features().to_string(), OptimizationLevel::None, RelocMode::Default, CodeModel::Default).unwrap();
		Compiler {
			llvm: context,
			target,
			machine,
		}
	}

	pub fn write_module(&self, module: &Module<'_>, file_name: impl AsRef<std::path::Path>) {
		self.machine.write_to_file(module, FileType::Object, file_name.as_ref()).unwrap();
	}
	
	pub fn compile_lir_module(&self, module: lir::Module) -> Module<'_> {
		let llvm_module = self.llvm.create_module(&module.name);
		llvm_module.set_data_layout(&self.machine.get_target_data().get_data_layout());
		llvm_module.set_triple(&self.machine.get_triple());
		
		for decl in module.decls_fn {
			let params = decl.params.iter().map(|(_, ty)| self.get_type(ty)).collect::<Vec<_>>();
			let function = llvm_module.add_function(
				&decl.id.0,
				self.get_type(&decl.returns).fn_type(&params, false),
				Some(Linkage::External),
			);
		}
		
		if let Some(def) = module.entry {
			let function = llvm_module.add_function(
				"main", //TODO
				match def.returns {
					Some(ty) => self.get_type(&ty).fn_type(&[], false),
					None => self.llvm.void_type().fn_type(&[], false),
				},
				Some(Linkage::External),
			);
			self.compile_fn_body(def.body, &llvm_module, function);
		}
		
		llvm_module
	}
}

impl Compiler {
	fn get_type(&self, ty: &lir::Type) -> BasicTypeEnum<'_> {
		match ty {
			lir::Type::PtrConst(t) | lir::Type::PtrMut(t) => self.get_type(&t).ptr_type(AddressSpace::Generic).into(),
			lir::Type::PtrDynConst(t) | lir::Type::PtrDynMut(t) => self.llvm.struct_type(&[
				self.llvm.ptr_sized_int_type(&self.machine.get_target_data(), None).into(),
				self.get_type(&t).ptr_type(AddressSpace::Generic).into()
			], false).into(),
			lir::Type::Arr(..) => todo!(),
			lir::Type::Tuple(..) => todo!(),
			lir::Type::Name(lir::Ident(id)) => match &**id {
				"i8" => self.llvm.i8_type().into(),
				"i16" => self.llvm.i16_type().into(),
				"i32" => self.llvm.i32_type().into(),
				"i64" => self.llvm.i64_type().into(),
				"c_char" => self.llvm.i8_type().into(),
				"c_short" => self.llvm.i16_type().into(), // ILP32, LLP64, LP64
				"c_int" => self.llvm.i32_type().into(), // ILP32, LLP64, LP64
				"c_long" => if self.machine.get_triple().as_str().to_bytes().split(|&b| b == b'-').skip(2).next().unwrap() == b"windows" {
					self.llvm.i32_type().into() // ILP32, LLP64 (Windows APIs)
				} else {
					self.llvm.ptr_sized_int_type(&self.machine.get_target_data(), None).into() // ILP32, LP64
				}
				"c_longlong" => self.llvm.i64_type().into(),
				_ => todo!(),
			},
			lir::Type::Void => todo!(),
		}
	}

	fn compile_fn_body<'ctx>(&'ctx self, body: lir::FnBody, module: &Module<'ctx>, fn_value: FunctionValue<'ctx>) {
		let builder = self.llvm.create_builder();
		let basic_block = self.llvm.append_basic_block(fn_value, "decl");
		builder.position_at_end(basic_block);

		let mut pointers = HashMap::<String, PointerValue<'ctx>>::new();

		for decl in body.decls {
			pointers.insert(decl.name, builder.build_alloca(self.get_type(&decl.ty), "var"));
		}

		builder.build_unconditional_branch(self.compile_block(body.block, "entry", &pointers, module, fn_value));
	}

	fn compile_block<'ctx>(&'ctx self, block: lir::Block, name: &str, pointers: &HashMap<String, PointerValue<'ctx>>, module: &Module<'ctx>, fn_value: FunctionValue<'ctx>) -> BasicBlock<'ctx> {
		let builder = self.llvm.create_builder();
		let root_block = self.llvm.append_basic_block(fn_value, name);
		builder.position_at_end(root_block);

		let mut has_returned = false;
		for statement in block.statements {
			debug_assert!(!has_returned, "Statements after return");
			match statement {
				lir::Statement::Eval(expr) => {
					self.compile_expr(expr.value, pointers, module, fn_value, &builder);
				},
				lir::Statement::Return(expr) => {
					match expr {
						Some(expr) => {
							let value = self.compile_expr(expr.value, pointers, module, fn_value, &builder).unwrap();
							debug_assert_eq!(value.get_type(), fn_value.get_type().get_return_type().unwrap());
							builder.build_return(Some(&value));
							has_returned = true;
						},
						None => {
							debug_assert_eq!(fn_value.get_type().get_return_type(), None);
							builder.build_return(None);
							has_returned = true;
						}
					}
				},
				lir::Statement::Decl(..) => todo!(),
			}
		}

		if !has_returned {
			builder.build_return(None);
		}

		root_block
	}

	fn compile_expr<'ctx>(&'ctx self, expr: lir::ExpressionValue, pointers: &HashMap<String, PointerValue<'ctx>>, module: &Module<'ctx>, fn_value: FunctionValue<'ctx>, builder: &Builder<'ctx>) -> Option<BasicValueEnum<'ctx>> {
		match expr {
			lir::ExpressionValue::Call(id, args) => {
				let callee = module.get_function(&id.0).expect("Undefined reference to function");
				let arguments = args.into_iter().map(|expr| self.compile_expr(expr.value, pointers, module, fn_value, builder)).collect::<Option<Vec<_>>>()?;
				builder.build_call(callee, &arguments, "calltmp").try_as_basic_value().left()
			},
			lir::ExpressionValue::ConstInt(val) => Some(BasicValueEnum::IntValue(self.llvm.i32_type().const_int(val as u64, true))),
			lir::ExpressionValue::Load(_) => todo!(),
		}
	}
}