//! Because things go wrong

#[derive(Debug)]
pub struct LIRError {
	pub ty: LIRErrorType
}

#[derive(Debug)]
pub enum LIRErrorType {
	UnresolvedIdent,
	MismatchedTypes,
	ArgCountMismatch,
	VoidValue,
	InvalidLValueExpr,
	ImmutAssign,
}