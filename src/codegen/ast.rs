use either::Either;

use super::lir;

#[derive(Debug)]
pub enum TopLevelDecl {
	FnExtern(FnExtern),
	Def(TopLevelDef),
	Decl(Decl),
}

#[derive(Debug)]
pub struct Use {
	pub external: bool,
	pub ty: Option<Vec<u8>>,
	pub path: Vec<u8>,
}

#[derive(Debug)]
pub enum Decl {
	Use(Use)
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
	pub body: Block,
}

#[derive(Debug)]
pub struct Entry {
	pub returns: Option<Type>,
	pub body: Block,
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
	Break(Option<Expression>),
	Return(Option<Expression>),
}

pub type Op = lir::Op;

#[derive(Debug)]
pub struct If(pub Box<Expression>, pub Box<Block>, pub Option<Either<Box<If>, Box<Block>>>);

#[derive(Debug)]
pub enum Expression {
	If(If),
	Loop(Box<Block>),
	Block(Box<Block>),

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
