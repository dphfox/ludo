<h1>
	ludo · <small>the <u>lu</u>a <u>do</u>er</small>
</h1>

An ultra-small runtime for Luau written in Rust.

## Philosophy
Libraries are provided by extensions, not the core runtime.

If you choose to use Ludo on its own, your programs won't do much above a
regular call to `luau`. This is by design! - this gives you choice as to what
capabilities your program will have.

To add capabilities to your program, you create extensions by co-locating a
Luau script and a native library in a directory. You can then explicitly point
`ludo` at this directory, which will expose it via `require` at the