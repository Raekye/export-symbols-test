fn main() {
	// Without referencing foo2, it won't show up in the final executable (neither will foo1).
	// By referencing foo2, foo1 also shows up, even though nothing references it?
	let _ = export_symbols::foo2;
	println!("Hello, world!");
}
