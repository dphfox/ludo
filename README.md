<h1>
	ludo · <small>the <u>lu</u>a <u>do</u>er</small>
</h1>

An ultra-small runtime for Luau written in Rust.

## Philosophy
Libraries are provided by extensions, not the core runtime.

If you choose to use Ludo on its own, your programs won't do much above a
regular call to `luau`. This is by design! - this gives you choice as to what
capabilities your program will have.

To add capabilities to your program, you can explicitly point to a native `.dll`
compatible with Ludo's interface. With this, you can access the native code's
features from inside of Luau.

These "native extension" scripts can define and export a Luau API just like any
other Luau module. End user code must explicitly grant permission to access
native features for security.

## License

Licensed the same way as all of my open source projects: BSD 3-Clause + Security Disclaimer.

As with all other projects, you accept responsibility for choosing and using this project.

See [LICENSE](./LICENSE) or [the license summary](https://github.com/dphfox/licence) for details.