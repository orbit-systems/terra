/// Kinds of errors
#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
	/// general error from `pest`
	SyntaxError,
	/// for `include` and `embed` directives, when failing to load the file
	CouldNotLoad,
	NameConflict,
	/// cyclic definition from `define` directive
	CyclicDefinition,
}

#[derive(Debug, Clone, Copy)]
pub enum WarningKind {
	/// for when the literal used is an unlikely type
	LikelyError,
	/// for when the literal goes out of the range of the capacity of the immediate value
	OutOfRange,
}
