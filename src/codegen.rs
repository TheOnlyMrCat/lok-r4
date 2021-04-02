pub mod ast {
	use super::lir;

	#[derive(Debug)]
	pub enum TopLevelDecl {
		FnExtern(FnExtern),
		Def(TopLevelDef),
	}
	
	#[derive(Debug)]
	pub enum TopLevelDef {
		Entry(Entry),
		Def(Def),
	}

	#[derive(Debug)]
	pub enum Def {
		Fn(FnDef)
	}

	#[derive(Debug)]
	pub struct FnExtern {
		pub name: Ident,
		pub params: Vec<(Option<Ident>, Type)>,
		pub varadic: bool,
		pub returns: Option<Type>,
	}

	#[derive(Debug)]
	pub struct FnDef {
		pub name: Ident,
		pub params: Vec<(Ident, Type)>,
		pub returns: Option<Type>,
	}

	#[derive(Debug)]
	pub struct Entry {
		pub returns: Option<Type>,
		pub code: Block,
	}

	#[derive(Debug)]
	pub struct Block {
		pub statements: Vec<Statement>,
		pub tail: Option<Expression>,
	}

	#[derive(Debug)]
	pub enum Statement {
		Decl {
			name: Ident,
			mutable: bool,
			expected_type: Option<Type>,
			value: Expression,
		},
		Expression(Expression),
		Return(Option<Expression>),
	}

	pub type Op = lir::Op;

	#[derive(Debug)]
	pub enum Expression {
		Assign(Box<Expression>, Option<Op>, Box<Expression>),

		Op(Op, Box<Expression>, Box<Expression>),

		Call(Box<Expression>, Vec<Expression>),

		LVar(NSIdent),
		Int(u64),
		CStringRef(Vec<u8>),
	}

	#[derive(Debug, Clone)]
	pub enum Type {
		Name(NSIdent),
		PtrConst(Box<Type>),
		PtrMut(Box<Type>),
		PtrDynConst(Box<Type>),
		PtrDynMut(Box<Type>),
		Slice(Box<Type>),
		Arr(Box<Type>, u64),
		Tuple(Vec<Type>),
	}

	pub type NSIdent = Vec<Ident>;
	pub type Ident = String;
}

pub mod lir {
	use crate::error::{LIRError, LIRErrorType};

	use super::ast;
	use super::{NameResolveMap, StackScope};

	pub struct Module {
		pub name: Ident,
		pub extern_fns: Vec<ExternFn>,
		pub defs_fn: Vec<DefFn>,
		pub consts: Constants,
		pub entry: Option<DefEntry>,
	}

	pub struct Constants {
		pub strings: Vec<(Vec<u8>, bool)>,
	}

	pub struct ExternFn {
		pub id: Ident,
		pub params: Vec<(String, Type)>,
		pub varadic: bool,
		pub returns: Option<Type>,
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
		pub tail: Option<Expression>,
	}

	pub enum Statement {
		Decl(String, Expression),
		Eval(Expression),
		Return(Option<Expression>),
	}

	pub struct Expression {
		pub ty: Option<Type>,
		pub value: ExpressionValue
	}

	pub struct LExpression {
		pub ty: Type,
		pub mutable: bool,
		pub value: LExpressionValue,
	}

	#[derive(Debug, Clone, Copy)]
	pub enum Op {
		Add,
		Sub,
		Mul,
		Div,
		Rem,

	}

	pub enum ExpressionValue {
		Assign(Option<Op>, LExpression, Box<Expression>),

		Op(Op, Box<Expression>, Box<Expression>),
		
		CallConcrete(Ident, Vec<Expression>),

		LExpr(LExpression),
		ConstInt(u64),
		ConstStr(usize /* Index into global string pool */),
	}

	pub enum LExpressionValue {
		Var(Ident),
	}

	#[derive(Clone, Debug)]
	pub struct Decl {
		pub name: Ident,
		pub mutable: bool,
		pub ty: Type,
	}

	#[derive(Debug, Clone, Hash, PartialEq, Eq)]
	pub enum Type {
		Name(Ident),
		PtrConst(Box<Type>),
		PtrMut(Box<Type>),
		PtrDynConst(Box<Type>),
		PtrDynMut(Box<Type>),
		Slice(Box<Type>),
		Arr(Box<Type>, u64),
		Tuple(Vec<Type>),
	}

	#[derive(Debug, Clone, Hash, PartialEq, Eq)]
	pub enum Ident {
		Local(String),
		UnmangledItem(String),
		Function(Vec<String>),
		Static(Vec<String>),
		Type(Vec<String>),
	}

	impl Module {
		pub fn from_ast(name: Ident, tl_decls: Vec<ast::TopLevelDecl>) -> Result<Module, LIRError> {
			let mut name_resolve = NameResolveMap {
				local_fns: HashMap::new(),
				scope_stack: vec![],
			};
			let mut consts = Constants {
				strings: vec![],
			};

			let mut extern_fns = vec![];
			let mut defs = vec![];
			for decl in tl_decls {
				match decl {
					ast::TopLevelDecl::FnExtern(f) => {
						extern_fns.push(ExternFn {
							id: Ident::UnmangledItem(f.name),
							params: f.params.into_iter().map(|(s, t)| Type::from_ast(t, &mut name_resolve).map(|t| (match s { Some(s) => s.to_owned(), None => "".to_owned() }, t))).collect::<Result<Vec<_>, _>>()?,
							varadic: f.varadic,
							returns: f.returns.map(|t| Type::from_ast(t, &mut name_resolve)).transpose()?,
						})
					},
					ast::TopLevelDecl::Def(def) => {
						defs.push(def);
					}
				}
			}

			use std::collections::HashMap;

			name_resolve.local_fns = extern_fns.iter().map(|decl| (decl.id.clone(), decl)).collect::<HashMap<_, _>>(); //TODO

			let defs_fn = vec![];
			let mut entry = None;
			for def in defs {
				match def {
					ast::TopLevelDef::Entry(e) => {
						assert!(entry.is_none(), "Multiple entry points declared!"); //TODO: Error type
						entry = Some(DefEntry {
							returns: e.returns.map(|t| Type::from_ast(t, &mut name_resolve)).transpose()?,
							body: FnBody::from_ast(e.code, &mut name_resolve, &mut consts)?,
						})
					},
					_ => {}
				}
			}
	
			Ok(Module {
				name,
				extern_fns,
				defs_fn,
				consts,
				entry,
			})
		}
	}
	
	impl FnBody {
		fn from_ast(block: ast::Block, name_resolve: &mut NameResolveMap<'_>, consts: &mut Constants) -> Result<FnBody, LIRError> {
			let mut decls = vec![];
			let mut statements = vec![];

			name_resolve.scope_stack.push(StackScope::default());
	
			for statement in block.statements {
				match statement {
					ast::Statement::Expression(e) => {
						statements.push(Statement::Eval(Expression::from_ast(e, name_resolve, consts)?))
					},
					ast::Statement::Return(e) => {
						statements.push(Statement::Return(e.map(|e| Expression::from_ast(e, name_resolve, consts)).transpose()?))
					}
				    ast::Statement::Decl { name, mutable, expected_type, value } => {
						let expr = Expression::from_ast(value, name_resolve, consts)?;
						let ty = expr.ty.clone().ok_or(LIRError { ty: LIRErrorType::VoidValue })?;
						if let Some(expected) = expected_type {
							if Type::from_ast(expected, name_resolve)? != ty {
								return Err(LIRError { ty: LIRErrorType::MismatchedTypes });
							}
						}
						let decl = Decl {
							name: Ident::Local(name.clone()),
							mutable,
							ty,
						};
						decls.push(decl.clone());
						name_resolve.scope_stack.last_mut().expect("One was pushed on earlier").vars.insert(name.clone(), decl.clone());
						statements.push(Statement::Decl(name, expr));
					}
				}
			}

			let block = Block {
				statements,
				tail: block.tail.map(|expr| Expression::from_ast(expr, name_resolve, consts)).transpose()?,
			};
	
			Ok(FnBody {
				decls,
				block,
			})
		}
	}

	impl LExpression {
		fn from_ast(expression: ast::Expression, name_resolve: &mut NameResolveMap<'_>, _consts: &mut Constants) -> Result<LExpression, LIRError> {
			Ok(match expression {
				ast::Expression::LVar(i) => {
					let Decl { ty, name, mutable, ..} = name_resolve.resolve_var_default(i).ok_or(LIRError { ty: LIRErrorType::UnresolvedIdent })?;
					LExpression {
						ty,
						mutable,
						value: LExpressionValue::Var(name),
					}
				},
				_ => Err(LIRError { ty: LIRErrorType::InvalidLValueExpr })?
			})
		}
	}
	
	impl Expression {
		fn from_ast(expression: ast::Expression, name_resolve: &mut NameResolveMap<'_>, consts: &mut Constants) -> Result<Expression, LIRError> {
			Ok(match expression {
				ast::Expression::Assign(lhs, op, rhs) => {
					let lvalue = LExpression::from_ast(*lhs, name_resolve, consts)?;
					if !lvalue.mutable {
						Err(LIRError { ty: LIRErrorType::ImmutAssign })?;
					}
					let rvalue = Expression::from_ast(*rhs, name_resolve, consts)?;
					Expression {
						ty: Some(lvalue.ty.clone()),
						value: ExpressionValue::Assign(op, lvalue, Box::new(rvalue))
					}
				},
			    ast::Expression::Op(op, lhs, rhs) => {
					Expression {
						ty: Some(Type::Name(Ident::UnmangledItem("i32".to_owned()))), //TODO !!
						value: ExpressionValue::Op(op, Box::new(Expression::from_ast(*lhs, name_resolve, consts)?), Box::new(Expression::from_ast(*rhs, name_resolve, consts)?)),
					}
				},
				ast::Expression::Call(f, a) => {
					match *f {
						ast::Expression::LVar(n) => {
							let (ty, ident) = name_resolve.resolve_fn_default(n).ok_or(LIRError { ty: LIRErrorType::UnresolvedIdent })?;
							Expression {
								ty,
								value: ExpressionValue::CallConcrete(ident, a.into_iter().map(|e| Expression::from_ast(e, name_resolve, consts)).collect::<Result<_, _>>()?),
							}
						},
						_ => todo!(),
					}
				},
				ast::Expression::Int(i) => {
					Expression {
						ty: Some(integer_type_for_value(i)),
						value: ExpressionValue::ConstInt(i)
					}
				},
				ast::Expression::CStringRef(s) => {
					consts.strings.push((s, true));
					Expression {
						ty: Some(Type::PtrConst(Box::new(Type::Name(Ident::UnmangledItem("c_char".to_owned()))))),
						value: ExpressionValue::ConstStr(consts.strings.len() - 1),
					}
				},
				ast::Expression::LVar(_) => {
					let lexpr = LExpression::from_ast(expression, name_resolve, consts)?;
					Expression {
						ty: Some(lexpr.ty.clone()),
						value: ExpressionValue::LExpr(lexpr),
					}
				},
			})
		}
	}

	impl Type {
		fn from_ast(ast: ast::Type, name_resolve: &mut NameResolveMap<'_>) -> Result<Type, LIRError> {
			Ok(match ast {
				ast::Type::Name(v) => Type::Name(name_resolve.resolve_typename_default(v).ok_or(LIRError { ty: LIRErrorType::UnresolvedIdent })?),
				ast::Type::PtrDynConst(ty) => Type::PtrDynConst(Box::new(Type::from_ast(*ty, name_resolve)?)),
				ast::Type::PtrDynMut(ty) => Type::PtrDynMut(Box::new(Type::from_ast(*ty, name_resolve)?)),
				ast::Type::PtrConst(ty) => Type::PtrConst(Box::new(Type::from_ast(*ty, name_resolve)?)),
				ast::Type::PtrMut(ty) => Type::PtrMut(Box::new(Type::from_ast(*ty, name_resolve)?)),
				_ => todo!(),
			})
		}
	}
	impl Ident {
		pub fn fn_mangle(&self) -> String {
			match self {
				Ident::UnmangledItem(s) => s.clone(),
				Ident::Function(parts) => std::iter::once("_LZ".to_owned())
					.chain(
						parts.iter()
							.flat_map(|item|
								std::iter::once(item.len().to_string())
									.chain(std::iter::once(item.clone()))
							)
					)
					.chain(std::iter::once("E".to_owned()))
					.collect(),
				_ => panic!("Attempted to mangle incompatible id as function"),
			}
		}

		pub fn mod_mangle(&self) -> String {
			"TODO".to_owned() //TODO
		}

		pub fn local_mangle(&self) -> String {
			match self {
				Ident::Local(s) => s.clone(),
				_ => panic!("Attempted to mangle incompatible id as local"),
			}
		}
	}

	fn integer_type_for_value(_value: u64) -> Type {
		Type::Name(Ident::UnmangledItem("i32".to_owned())) //TODO
	}
}

use std::collections::HashMap;

#[derive(Default)]
struct StackScope {
	vars: HashMap<String, lir::Decl>,
}

struct NameResolveMap<'a> {
	local_fns: HashMap<lir::Ident, &'a lir::ExternFn>,
	scope_stack: Vec<StackScope>,
}

impl NameResolveMap<'_> {
	fn resolve_fn_default(&self, name: Vec<String>) -> Option<(Option<lir::Type>, lir::Ident)> {
		let id = lir::Ident::UnmangledItem(name[0].clone()); //TODO
		match self.local_fns.get(&id) { //TODO
			Some(decl) => Some((decl.returns.clone(), id)),
			None => None,
		}
	}

	fn resolve_var_default(&self, name: Vec<String>) -> Option<lir::Decl> {
		if name.len() == 1 {
			let mut found = None;
			for scope in self.scope_stack.iter().rev() {
				if let Some(decl) = scope.vars.get(&name[0]) {
					found = Some(decl.clone());
					break;
				}
			}
			found
		} else {
			None
		}
	}

	fn resolve_typename_default(&self, name: Vec<String>) -> Option<lir::Ident> {
		match &*name[0] {
			"i8"|"i16"|"i32"|"i64"|"c_char"|"c_short"|"c_int"|"c_long"|"c_longlong" if name.len() == 1 => Some(lir::Ident::UnmangledItem(name[0].clone())),
			_ => todo!(),
		}
	}
}

//MARK: Compiler
use inkwell::{context::Context, values::GlobalValue};
use inkwell::builder::Builder;
use inkwell::basic_block::BasicBlock;
use inkwell::module::{Module, Linkage};
use inkwell::targets::{Target, TargetMachine, RelocMode, CodeModel, FileType, InitializationConfig};
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{FunctionValue, PointerValue, BasicValueEnum};
use inkwell::{AddressSpace, OptimizationLevel};

pub struct Compiler {
	llvm: Context,
	target: TargetMachine,
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
		let machine = target.create_target_machine(&triple, "generic", &TargetMachine::get_host_cpu_features().to_string(), OptimizationLevel::None, RelocMode::Default, CodeModel::Default).unwrap();
		Compiler {
			llvm: context,
			target: machine,
		}
	}

	pub fn print_ir(&self, module: &Module<'_>, file_name: impl AsRef<std::path::Path>) {
		module.print_to_file(file_name).unwrap();
	}

	pub fn write_module(&self, module: &Module<'_>, file_name: impl AsRef<std::path::Path>) {
		self.target.write_to_file(module, FileType::Object, file_name.as_ref()).unwrap();
	}
	
	pub fn compile_lir_module(&self, module: lir::Module) -> Module<'_> {
		let llvm_module = self.llvm.create_module(&module.name.mod_mangle());
		llvm_module.set_data_layout(&self.target.get_target_data().get_data_layout());
		llvm_module.set_triple(&self.target.get_triple());

		for decl in module.extern_fns {
			let params = decl.params.iter().map(|(_, ty)| self.get_type(ty)).collect::<Vec<_>>();
			let varadic = decl.varadic;
			let _function = llvm_module.add_function(
				&decl.id.fn_mangle(),
				decl.returns.map(|x| self.get_type(&x).fn_type(&params, varadic)).unwrap_or(self.llvm.void_type().fn_type(&params, false)),
				Some(Linkage::External),
			);
		}

		let global_pool = GlobalPool {
			strings: module.consts.strings.into_iter().enumerate().map(|(i, (v, null))| {
				let value = self.llvm.const_string(&v, null);
				let global = llvm_module.add_global(value.get_type(), Some(AddressSpace::Const), &format!("str{}", i));
				global.set_constant(true);
				global.set_initializer(&value);
				global
			}).collect(),
		};
		
		if let Some(def) = module.entry {
			let function = llvm_module.add_function(
				"main", //TODO
				match def.returns {
					Some(ty) => self.get_type(&ty).fn_type(&[], false),
					None => self.llvm.void_type().fn_type(&[], false),
				},
				Some(Linkage::External),
			);
			self.compile_fn_body(def.body, &global_pool, &llvm_module, function);
		}
		
		llvm_module
	}
}

impl Compiler {
	fn get_type(&self, ty: &lir::Type) -> BasicTypeEnum<'_> {
		match ty {
			lir::Type::PtrConst(t) | lir::Type::PtrMut(t) => self.get_type(&t).ptr_type(AddressSpace::Generic).into(),
			lir::Type::PtrDynConst(t) | lir::Type::PtrDynMut(t) => self.llvm.struct_type(&[
				self.llvm.ptr_sized_int_type(&self.target.get_target_data(), None).into(),
				self.get_type(&t).ptr_type(AddressSpace::Generic).into()
			], false).into(),
			lir::Type::Arr(..) => todo!(),
			lir::Type::Slice(..) => todo!(),
			lir::Type::Tuple(..) => todo!(),
			lir::Type::Name(id) => {
				match id {
					lir::Ident::UnmangledItem(id) => {
						// Check for primitive types
						match &**id {
							"i8" => self.llvm.i8_type().into(),
							"i16" => self.llvm.i16_type().into(),
							"i32" => self.llvm.i32_type().into(),
							"i64" => self.llvm.i64_type().into(),
							"c_char" => self.llvm.i8_type().into(),
							"c_short" => self.llvm.i16_type().into(), // ILP32, LLP64, LP64
							"c_int" => self.llvm.i32_type().into(), // ILP32, LLP64, LP64
							"c_long" => if self.target.get_triple().as_str().to_bytes().split(|&b| b == b'-').skip(2).next().unwrap() == b"windows" {
								self.llvm.i32_type().into() // ILP32, LLP64 (Windows APIs)
							} else {
								self.llvm.ptr_sized_int_type(&self.target.get_target_data(), None).into() // ILP32, LP64 (Unix APIs)
							}
							"c_longlong" => self.llvm.i64_type().into(),
							_ => todo!(),
						}
					}
					_ => todo!(),
				}
			}
		}
	}

	fn compile_fn_body<'ctx>(&'ctx self, body: lir::FnBody, global_pool: &GlobalPool<'ctx>, module: &Module<'ctx>, fn_value: FunctionValue<'ctx>) {
		let builder = self.llvm.create_builder();
		let basic_block = self.llvm.append_basic_block(fn_value, "decl");
		builder.position_at_end(basic_block);

		let mut pointers = HashMap::<String, PointerValue<'ctx>>::new();

		for decl in &body.decls {
			let name = decl.name.local_mangle();
			pointers.insert(name.clone(), builder.build_alloca(self.get_type(&decl.ty), &name));
		}

		builder.build_unconditional_branch(self.compile_block(body.block, "entry", &pointers, global_pool, module, fn_value));
	}

	fn compile_block<'ctx>(&'ctx self, block: lir::Block, name: &str, pointers: &HashMap<String, PointerValue<'ctx>>, global_pool: &GlobalPool<'ctx>, module: &Module<'ctx>, fn_value: FunctionValue<'ctx>) -> BasicBlock<'ctx> {
		let builder = self.llvm.create_builder();
		let root_block = self.llvm.append_basic_block(fn_value, name);
		builder.position_at_end(root_block);

		let mut has_returned = false;
		for statement in block.statements {
			debug_assert!(!has_returned, "Statements after return");
			match statement {
				lir::Statement::Eval(expr) => {
					self.compile_expr(expr.value, pointers, global_pool, module, fn_value, &builder);
				},
				lir::Statement::Return(expr) => {
					match expr {
						Some(expr) => {
							let value = self.compile_expr(expr.value, pointers, global_pool, module, fn_value, &builder).unwrap();
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
				lir::Statement::Decl(name, expr) => {
					builder.build_store(
						pointers.get(&name).expect("All decl statements are given pointers").clone(),
						self.compile_expr(expr.value, pointers, global_pool, module, fn_value, &builder).expect("Type was checked by LIR")
					);
				},
			}
		}

		if !has_returned {
			builder.build_return(None);
		}

		root_block
	}

	fn compile_expr<'ctx>(&'ctx self, expr: lir::ExpressionValue, pointers: &HashMap<String, PointerValue<'ctx>>, global_pool: &GlobalPool<'ctx>, module: &Module<'ctx>, fn_value: FunctionValue<'ctx>, builder: &Builder<'ctx>) -> Option<BasicValueEnum<'ctx>> {
		match expr {
		    lir::ExpressionValue::Assign(op, lhs, rhs) => {
				let val = match op {
					Some(_) => todo!(),
					None => self.compile_expr(rhs.value, pointers, global_pool, module, fn_value, builder),
				};
				builder.build_store(self.compile_lexpr(lhs.value, pointers, global_pool, module, fn_value, builder), val.expect("Type was checked by LIR"));
				val
			}
			lir::ExpressionValue::Op(op, lhs, rhs) => {
				match op {
					lir::Op::Add => Some(BasicValueEnum::IntValue(builder.build_int_add(self.compile_expr(lhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), self.compile_expr(rhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), "addtmp"))),
					lir::Op::Sub => Some(BasicValueEnum::IntValue(builder.build_int_sub(self.compile_expr(lhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), self.compile_expr(rhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), "addtmp"))),
					lir::Op::Mul => Some(BasicValueEnum::IntValue(builder.build_int_mul(self.compile_expr(lhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), self.compile_expr(rhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), "addtmp"))),
					lir::Op::Div => Some(BasicValueEnum::IntValue(builder.build_int_signed_div(self.compile_expr(lhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), self.compile_expr(rhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), "addtmp"))),
					lir::Op::Rem => Some(BasicValueEnum::IntValue(builder.build_int_signed_rem(self.compile_expr(lhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), self.compile_expr(rhs.value, pointers, global_pool, module, fn_value, builder)?.into_int_value(), "addtmp"))),
				}
			}
			lir::ExpressionValue::CallConcrete(id, args) => {
				let callee = module.get_function(&id.fn_mangle()).expect("Undefined reference to function");
				let arguments = args.into_iter().map(|expr| self.compile_expr(expr.value, pointers, global_pool, module, fn_value, builder)).collect::<Option<Vec<_>>>()?;
				builder.build_call(callee, &arguments, "calltmp").try_as_basic_value().left()
			},
			lir::ExpressionValue::ConstInt(val) => Some(BasicValueEnum::IntValue(self.llvm.i32_type().const_int(val as u64, true))),
			lir::ExpressionValue::ConstStr(i) => Some(BasicValueEnum::PointerValue(global_pool.strings[i].as_pointer_value())), //TODO: Caching?
			lir::ExpressionValue::LExpr(lexpr) => Some(builder.build_load(self.compile_lexpr(lexpr.value, pointers, global_pool, module, fn_value, builder), "loadtmp")),
		}
	}

	fn compile_lexpr<'ctx>(&'ctx self, expr: lir::LExpressionValue, pointers: &HashMap<String, PointerValue<'ctx>>, _global_pool: &GlobalPool<'ctx>, _module: &Module<'ctx>, _fn_value: FunctionValue<'ctx>, _builder: &Builder<'ctx>) -> PointerValue<'ctx> {
		match expr {
			lir::LExpressionValue::Var(ident) => match ident {
				lir::Ident::Local(_) => pointers.get(&ident.local_mangle()).expect("Local variable should have been declared").clone(),
				_ => todo!(),
			}
		}
	}
}

struct GlobalPool<'ctx> {
	strings: Vec<GlobalValue<'ctx>>,
}