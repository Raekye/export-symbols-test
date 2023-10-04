Functions from the library crate marked `#[no_mangle]` seem to be removed from executables if no references are made to the library,
even when a linker arg like `-rdynamic` is used.
This behaviour seems to be described in this bug report: https://github.com/rust-lang/rust/issues/47384.
However, that bug report is supposedly fixed by https://github.com/rust-lang/rust/pull/95604.

```
# cat src/bin/without_extern_crate.rs
fn main() {
	println!("Hello, world!");
}
```

```
# RUSTFLAGS='-C link-arg=-rdynamic' cargo build && nm -g target/debug/without_extern_crate.rs | grep foo
# nothing
```

```
# cat src/bin/with_extern_crate.rs
extern crate export_symbols;

fn main() {
	println!("Hello, world!");
}
```

```
# RUSTFLAGS='-C link-arg=-rdynamic' cargo build && nm -g target/debug/with_extern_crate.rs | grep foo
000000000006dfa0 T foo1
000000000006dfb0 T foo2
```
