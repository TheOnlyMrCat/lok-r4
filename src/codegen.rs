pub mod ast;
pub mod lir;

use std::collections::HashMap;

#[derive(Default, Debug)]
struct StackScope {
	vars: HashMap<String, lir::Decl>,
}

#[derive(Debug)]
struct NameResolveMap {
	local_fns: HashMap<lir::Ident, lir::DeclFn>,
	scope_stack: Vec<StackScope>,
}

impl NameResolveMap {
	fn new() -> NameResolveMap {
		NameResolveMap {
			local_fns: HashMap::new(),
			scope_stack: Vec::new(),
		}
	}

	fn resolve_fn_default(&self, name: Vec<String>) -> Option<&lir::DeclFn> {
		let id = lir::Ident::UnmangledItem(name[0].clone()); //TODO
		self.local_fns.get(&id) //TODO
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

		let mut functions = HashMap::new();
		for decl in module.fn_decls {
			let params = decl.params.iter().map(|(_, ty)| self.get_type(ty)).collect::<Vec<_>>();
			let varadic = decl.varadic;
			let function = llvm_module.add_function(
				&decl.id.fn_mangle(),
				decl.returns.map(|x| self.get_type(&x).fn_type(&params, varadic)).unwrap_or(self.llvm.void_type().fn_type(&params, false)),
				Some(Linkage::External),
			);
			functions.insert(decl.id, function);
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

		for def in module.fn_defs {
			let function = functions.get(&def.id).expect("Was inserted in LIR stage").clone();
			self.compile_fn_body(def.body, &global_pool, &llvm_module, function);
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
			lir::Type::Primitive(p) => match p {
				lir::Primitive::I8 | lir::Primitive::U8 => self.llvm.i8_type().into(),
				lir::Primitive::I16 | lir::Primitive::U16 => self.llvm.i16_type().into(),
				lir::Primitive::I32 | lir::Primitive::U32 => self.llvm.i32_type().into(),
				lir::Primitive::I64 | lir::Primitive::U64 => self.llvm.i64_type().into(),
				lir::Primitive::CChar => self.llvm.i8_type().into(),
				lir::Primitive::CShort => self.llvm.i16_type().into(), // ILP32, LLP64, LP64
				lir::Primitive::CInt => self.llvm.i32_type().into(), // ILP32, LLP64, LP64
				lir::Primitive::CLong => if self.target.get_triple().as_str().to_bytes().split(|&b| b == b'-').skip(2).next().unwrap() == b"windows" {
					self.llvm.i32_type().into() // ILP32, LLP64 (Windows APIs)
				} else {
					self.llvm.ptr_sized_int_type(&self.target.get_target_data(), None).into() // ILP32, LP64 (Unix APIs)
				}
				lir::Primitive::CLLong => self.llvm.i64_type().into(),
			}
			lir::Type::Name(..) => todo!()
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