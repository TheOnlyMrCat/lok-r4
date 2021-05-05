use either::{Either, Left, Right};

use crate::error::{LIRError, LIRErrorType};

use super::ast;
use super::{NameResolveMap, StackScope};

#[derive(Debug)]
pub struct Module {
	pub name: Ident,
	pub fn_decls: Vec<DeclFn>,
	pub fn_defs: Vec<DefFn>,
	pub consts: Constants,
	pub entry: Option<DefEntry>,
}

#[derive(Debug)]
pub struct Constants {
	pub strings: Vec<(Vec<u8>, bool)>,
}

#[derive(Clone, Debug)]
pub struct DeclFn {
	pub id: Ident,
	pub params: Vec<(String, Type)>,
	pub varadic: bool,
	pub returns: Option<Type>,
}

#[derive(Debug)]
pub struct DefFn {
	pub id: Ident,
	pub body: FnBody,
}

#[derive(Debug)]
pub struct DefEntry {
	pub returns: Option<Type>,
	pub body: FnBody,
}

#[derive(Debug)]
pub struct FnBody {
	pub decls: Vec<Decl>,
	pub block: Block,
}

#[derive(Clone, Debug)]
pub struct Block {
	pub statements: Vec<Statement>,
	pub tail: Option<Expression>,
}

#[derive(Clone, Debug)]
pub enum Statement {
	Decl(String, Expression),
	Eval(Expression),
	Break(Option<Expression>),
	Return(Option<Expression>),
}

#[derive(Clone, Debug)]
pub struct Expression {
	pub ty: Option<Type>,
	pub value: ExpressionValue
}

#[derive(Clone, Debug)]
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
	Lt,
	Le,
	Eq,
	Ge,
	Gt,
}

#[derive(Clone, Debug)]
pub struct If(pub Box<Expression>, pub Box<Block>, pub Option<Box<Block>>);

#[derive(Clone, Debug)]
pub enum ExpressionValue {
	If(If),
	Loop(Box<Block>),
	Block(Box<Block>),

	Assign(Option<Op>, LExpression, Box<Expression>),

	Op(Op, Box<Expression>, Box<Expression>),
	
	CallConcrete(Ident, Vec<Expression>),

	LExpr(LExpression),
	ConstInt(u64),
	ConstStr(usize /* Index into global string pool */),
}

#[derive(Clone, Debug)]
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
	Never,
	Primitive(Primitive),
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
pub enum Primitive {
	Bool,
	I8,
	I16,
	I32,
	I64,
	U8,
	U16,
	U32,
	U64,
	CChar,
	CShort,
	CInt,
	CLong,
	CLLong,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Ident {
	Local(String),
	UnmangledItem(String),
	// Function(Vec<String>),
	// Static(Vec<String>),
	// Type(Vec<String>),
}

impl Module {
	pub fn from_ast(name: Ident, tl_decls: Vec<ast::TopLevelDecl>) -> Result<Module, LIRError> {
		let mut name_resolve = NameResolveMap::new();
		let mut consts = Constants {
			strings: vec![],
		};

		let mut fn_decls = vec![];
		let mut defs = vec![];
		for decl in tl_decls {
			match decl {
				ast::TopLevelDecl::FnExtern(f) => {
					fn_decls.push(DeclFn {
						id: Ident::UnmangledItem(f.name),
						params: f.params.into_iter().map(|(s, t)| Type::from_ast(t, &mut name_resolve).map(|t| (match s { Some(s) => s.to_owned(), None => "".to_owned() }, t))).collect::<Result<Vec<_>, _>>()?,
						varadic: f.varadic,
						returns: f.returns.map(|t| Type::from_ast(t, &mut name_resolve)).transpose()?,
					})
				},
				ast::TopLevelDecl::Decl(_) => {
					
				},
				ast::TopLevelDecl::Def(def) => {
					match &def {
						ast::TopLevelDef::Def(ast::Def::Fn(f)) => {
							fn_decls.push(DeclFn {
								id: Ident::UnmangledItem(f.name.clone()),
								params: f.params.iter().cloned().map(|(s, t)| Type::from_ast(t, &mut name_resolve).map(|t| (s, t))).collect::<Result<Vec<_>, _>>()?,
								varadic: false,
								returns: f.returns.clone().map(|t| Type::from_ast(t, &mut name_resolve)).transpose()?,
							})
						}
						_ => {}
					}
					defs.push(def);
				}
			}
		}

		use std::collections::HashMap;

		name_resolve.local_fns = fn_decls.iter().map(|decl| Ok((decl.id.clone(), decl.clone())))
			.chain(
				defs.iter().filter_map(|def| match def {
					ast::TopLevelDef::Def(ast::Def::Fn(f)) => match f.params.iter().map(|(s, t)| match Type::from_ast(t.clone(), &mut name_resolve) { Ok(t) => Ok((s.clone(), t)), Err(e) => Err(e) }).collect() {
						Ok(params) => match f.returns.clone().map(|t| Type::from_ast(t, &mut name_resolve)).transpose() {
							Ok(returns) => Some(Ok((Ident::UnmangledItem(f.name.clone()), DeclFn {
								id: Ident::UnmangledItem(f.name.clone()),
								params,
								varadic: false,
								returns,
							}))),
							Err(e) => Some(Err(e)),
						},
						Err(e) => Some(Err(e))
					}
					_ => None,
				})
			)
			.collect::<Result<HashMap<_, _>, _>>()?; //TODO

		let mut fn_defs = vec![];
		let mut entry = None;
		for def in defs {
			match def {
				ast::TopLevelDef::Entry(e) => {
					assert!(entry.is_none(), "Multiple entry points declared!"); //TODO: Error type
					entry = Some(DefEntry {
						returns: e.returns.map(|t| Type::from_ast(t, &mut name_resolve)).transpose()?,
						body: FnBody::from_ast(e.body, &mut name_resolve, &mut consts)?,
					})
				},
				ast::TopLevelDef::Def(ast::Def::Fn(f)) => {
					let mut scope = StackScope::default();
					for (param, ty) in f.params {
						scope.vars.insert(param.clone(), Decl {
							name: Ident::Local(param),
							mutable: false,
							ty: Type::from_ast(ty, &mut name_resolve)?,
						});
					}
					name_resolve.scope_stack.push(scope);
					fn_defs.push(DefFn {
						id: Ident::UnmangledItem(f.name), //TODO
						body: FnBody::from_ast(f.body, &mut name_resolve, &mut consts)?
					});
					name_resolve.scope_stack.pop();
				}
			}
		}

		Ok(Module {
			name,
			fn_decls,
			fn_defs,
			consts,
			entry,
		})
	}

	pub fn print_to_file(&self, file_name: impl AsRef<std::path::Path>) -> std::io::Result<()> {
		use std::fs::File;
		use std::io::Write;

		let mut out_file = File::create(file_name)?;
		write!(out_file, "{:#?}", self)?;
		out_file.flush()?;
		Ok(())
	}
}

impl FnBody {
	fn from_ast(block: ast::Block, name_resolve: &mut NameResolveMap, consts: &mut Constants) -> Result<FnBody, LIRError> {
		let mut decls = vec![];

		Ok(FnBody {
			block: Block::from_ast(block, name_resolve, &mut decls, &mut Vec::new(), consts)?,
			decls,
		})
	}
}

impl Block {
	fn from_ast(block: ast::Block, name_resolve: &mut NameResolveMap, decls: &mut Vec<Decl>, loops: &mut Vec<LoopBreak>, consts: &mut Constants) -> Result<Block, LIRError> {
		let mut statements = vec![];
		name_resolve.scope_stack.push(StackScope::default());

		for statement in block.statements {
			match statement {
				ast::Statement::Expression(e) => {
					statements.push(Statement::Eval(Expression::from_ast(e, name_resolve, decls, loops, consts)?))
				},
				ast::Statement::Break(e) => {
					let expr = e.map(|e| Expression::from_ast(e, name_resolve, decls, loops, consts)).transpose()?;
					if let Some(loop_bk) = loops.last_mut() {
						if let Some(ty) = loop_bk.ty.as_ref() {
							if expr.as_ref().and_then(|e| e.ty.as_ref()) != ty.as_ref() {
								Err(LIRError { ty: LIRErrorType::MismatchedTypes })?;
							}
						} else {
							loop_bk.ty = Some(expr.as_ref().and_then(|e| e.ty.clone()));
						}
						statements.push(Statement::Break(expr));
					} else {
						Err(LIRError { ty: LIRErrorType::BreakOutsideLoop })?;
					}
				}
				ast::Statement::Return(e) => {
					statements.push(Statement::Return(e.map(|e| Expression::from_ast(e, name_resolve, decls, loops, consts)).transpose()?))
				}
				ast::Statement::Decl { name, mutable, expected_type, value } => {
					let mut expr = Expression::from_ast(value, name_resolve, decls, loops, consts)?;
					if let Some(expected) = expected_type {
						expr = expr.coerce(&Type::from_ast(expected, name_resolve)?).ok_or(LIRError { ty: LIRErrorType::MismatchedTypes })?;
					}
					let decl = Decl {
						name: Ident::Local(name.clone()),
						mutable,
						ty: expr.ty.clone().ok_or(LIRError { ty: LIRErrorType::VoidValue })?,
					};
					decls.push(decl.clone());
					name_resolve.scope_stack.last_mut().expect("One was pushed on earlier").vars.insert(name.clone(), decl.clone());
					statements.push(Statement::Decl(name, expr));
				}
			}
		}

		let tail = block.tail.map(|expr| Expression::from_ast(expr, name_resolve, decls, loops, consts)).transpose()?;

		name_resolve.scope_stack.pop();

		Ok(Block {
			statements,
			tail,
		})
	}
}

impl LExpression {
	fn from_ast(expression: ast::Expression, name_resolve: &mut NameResolveMap, _decls: &mut Vec<Decl>, _consts: &mut Constants) -> Result<LExpression, LIRError> {
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
	fn from_ast(expression: ast::Expression, name_resolve: &mut NameResolveMap, decls: &mut Vec<Decl>, loops: &mut Vec<LoopBreak>, consts: &mut Constants) -> Result<Expression, LIRError> {
		Ok(match expression {
			ast::Expression::Assign(lhs, op, rhs) => {
				let lvalue = LExpression::from_ast(*lhs, name_resolve, decls, consts)?;
				if !lvalue.mutable {
					Err(LIRError { ty: LIRErrorType::ImmutAssign })?;
				}
				let rvalue = Expression::from_ast(*rhs, name_resolve, decls, loops, consts)?.coerce(&lvalue.ty).ok_or(LIRError { ty: LIRErrorType::MismatchedTypes })?;

				Expression {
					ty: Some(lvalue.ty.clone()),
					value: ExpressionValue::Assign(op, lvalue, Box::new(rvalue))
				}
			},
			ast::Expression::Op(op, lhs, rhs) => {
				Expression {
					ty: Some(Type::Primitive(match op {
						Op::Eq | Op::Gt | Op::Ge | Op::Lt | Op::Le => Primitive::Bool,
						_ => Primitive::I32
					})), //TODO !!
					value: ExpressionValue::Op(op, Box::new(Expression::from_ast(*lhs, name_resolve, decls, loops, consts)?), Box::new(Expression::from_ast(*rhs, name_resolve, decls, loops, consts)?)),
				}
			},
			ast::Expression::Call(f, mut a) => {
				match *f {
					ast::Expression::LVar(n) => {
						let decl = name_resolve.resolve_fn_default(n).ok_or(LIRError { ty: LIRErrorType::UnresolvedIdent })?.clone();
						if if decl.varadic { a.len() < decl.params.len() } else { a.len() != decl.params.len() } {
							Err(LIRError { ty: LIRErrorType::ArgCountMismatch })?;
						}

						let varargs = if a.len() == decl.params.len() {
							vec![]
						} else {
							a.split_off(decl.params.len())
						};

						let args = a.into_iter()
							.zip(decl.params.iter())
							.map(|(e, (_, ty))| Expression::from_ast(e, name_resolve, decls, loops, consts)?
								.coerce(ty).ok_or(LIRError { ty: LIRErrorType::MismatchedTypes })
							)
							.collect::<Vec<_>>()
							.into_iter()
							.chain(varargs.into_iter()
								.map(|e| Expression::from_ast(e, name_resolve, decls, loops, consts))
							)
							.collect::<Result<Vec<_>, _>>()?;

						Expression {
							ty: decl.returns.clone(),
							value: ExpressionValue::CallConcrete(decl.id.clone(), args),
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
			ast::Expression::Block(b) => {
				let ir = Block::from_ast(*b, name_resolve, decls, loops, consts)?;
				Expression {
					ty: ir.tail.as_ref().and_then(|e| e.ty.clone()),
					value: ExpressionValue::Block(Box::new(ir)),
				}
			},
			ast::Expression::If(i) => {
				let ir = If::from_ast(i, name_resolve, decls, loops, consts)?;
				Expression {
					ty: ir.1.tail.as_ref().and_then(|e| e.ty.clone()),
					value: ExpressionValue::If(ir)
				}
			},
			ast::Expression::Loop(b) => {
				loops.push(LoopBreak {
				    name: "".to_owned(),
				    ty: None,
				});
				let block = Block::from_ast(*b, name_resolve, decls, loops, consts)?;
				let brk = loops.pop().unwrap();
				Expression {
					ty: brk.ty.unwrap_or(Some(Type::Never)),
					value: ExpressionValue::Loop(Box::new(block)),
				}
			},
			ast::Expression::CStringRef(s) => {
				consts.strings.push((s, true));
				Expression {
					ty: Some(Type::PtrConst(Box::new(Type::Primitive(Primitive::CChar)))),
					value: ExpressionValue::ConstStr(consts.strings.len() - 1),
				}
			},
			ast::Expression::LVar(_) => {
				let lexpr = LExpression::from_ast(expression, name_resolve, decls, consts)?;
				Expression {
					ty: Some(lexpr.ty.clone()),
					value: ExpressionValue::LExpr(lexpr),
				}
			},
		})
	}

	fn coerce(mut self, target_type: &Type) -> Option<Expression> {
		if self.ty.as_ref().map(|t| t == target_type).unwrap_or(false) {
			return Some(self);
		}
		Some(match (self.ty, target_type) {
			(Some(Type::Primitive(Primitive::I32)), Type::Primitive(Primitive::CInt)) =>  {
				self.ty = Some(Type::Primitive(Primitive::CInt));
				self
			},
			_ => todo!(),
		})
	}
}

impl If {
	fn from_ast(ast: ast::If, name_resolve: &mut NameResolveMap, decls: &mut Vec<Decl>, loops: &mut Vec<LoopBreak>, consts: &mut Constants) -> Result<If, LIRError> {
		let ast::If(cond, true_branch, false_branch) = ast;
		let condition = Expression::from_ast(*cond, name_resolve, decls, loops, consts)?.coerce(&Type::Primitive(Primitive::Bool)).ok_or(LIRError { ty: LIRErrorType::IllegalConditionExpr })?;
		let true_block = Block::from_ast(*true_branch, name_resolve, decls, loops, consts)?;
		let false_item = match false_branch {
			Some(Left(i)) => {
				let c = If::from_ast(*i, name_resolve, decls, loops, consts)?;
				Some(Box::new(Block { statements: vec![], tail: Some(Expression { ty: c.1.tail.as_ref().and_then(|e| e.ty.clone()), value: ExpressionValue::If(c) }) }))
			},
			Some(Right(b)) => Some(Box::new(Block::from_ast(*b, name_resolve, decls, loops, consts)?)),
			None => None
		};
		let lir = If(Box::new(condition), Box::new(true_block), false_item);
		if lir.1.tail.as_ref().and_then(|tail| tail.ty.as_ref()) == lir.2.as_ref().and_then(|tail| tail.tail.as_ref().and_then(|tail| tail.ty.as_ref())) {
			Ok(lir)
		} else {
			Err(LIRError { ty: LIRErrorType::MismatchedTypes })
		}
	}
}

impl Type {
	fn from_ast(ast: ast::Type, name_resolve: &mut NameResolveMap) -> Result<Type, LIRError> {
		Ok(match ast {
			ast::Type::Name(v) => if v.len() == 1 {
				match &*v[0] {
					"bool" => Type::Primitive(Primitive::Bool),
					"i8" => Type::Primitive(Primitive::I8),
					"i16" => Type::Primitive(Primitive::I16),
					"i32" => Type::Primitive(Primitive::I32),
					"i64" => Type::Primitive(Primitive::I64),
					"u8" => Type::Primitive(Primitive::U8),
					"u16" => Type::Primitive(Primitive::U16),
					"u32" => Type::Primitive(Primitive::U32),
					"u64" => Type::Primitive(Primitive::U64),
					"c_char" => Type::Primitive(Primitive::CChar),
					"c_short" => Type::Primitive(Primitive::CShort),
					"c_int" => Type::Primitive(Primitive::CInt),
					"c_long" => Type::Primitive(Primitive::CLong),
					"c_longlong" => Type::Primitive(Primitive::CLLong),
					_ => Type::Name(name_resolve.resolve_typename_default(v).ok_or(LIRError { ty: LIRErrorType::UnresolvedIdent })?)
				}
			} else {
				Type::Name(name_resolve.resolve_typename_default(v).ok_or(LIRError { ty: LIRErrorType::UnresolvedIdent })?)
			},
			ast::Type::PtrDynConst(ty) => Type::PtrDynConst(Box::new(Type::from_ast(*ty, name_resolve)?)),
			ast::Type::PtrDynMut(ty) => Type::PtrDynMut(Box::new(Type::from_ast(*ty, name_resolve)?)),
			ast::Type::PtrConst(ty) => Type::PtrConst(Box::new(Type::from_ast(*ty, name_resolve)?)),
			ast::Type::PtrMut(ty) => Type::PtrMut(Box::new(Type::from_ast(*ty, name_resolve)?)),
			ast::Type::Slice(ty) => Type::Slice(Box::new(Type::from_ast(*ty, name_resolve)?)),
			ast::Type::Arr(ty, n) => Type::Arr(Box::new(Type::from_ast(*ty, name_resolve)?), n),
			ast::Type::Tuple(types) => Type::Tuple(types.into_iter().map(|ty| Type::from_ast(ty, name_resolve)).collect::<Result<_, _>>()?),
		})
	}
}

impl Ident {
	pub fn fn_mangle(&self) -> String {
		match self {
			Ident::UnmangledItem(s) => s.clone(),
			// Ident::Function(parts) => std::iter::once("_LZ".to_owned())
			// 	.chain(
			// 		parts.iter()
			// 			.flat_map(|item|
			// 				std::iter::once(item.len().to_string())
			// 					.chain(std::iter::once(item.clone()))
			// 			)
			// 	)
			// 	.chain(std::iter::once("E".to_owned()))
			// 	.collect(),
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

struct LoopBreak {
	name: String,
	ty: Option<Option<Type>>, // Outer option is assignment, inner option is for void or not
}

fn integer_type_for_value(_value: u64) -> Type {
	Type::Primitive(Primitive::I32) //TODO
}
