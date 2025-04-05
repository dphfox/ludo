--[[
	When a .ludorc file specifies the `native` field, a corresponding `native`
	global will be exposed to any Luau scripts within that file's jurisdiction.
	The `native` global exposes the API provided by the native library.

	It's best to build an idiomatic, well-typed API around `native` in a script
	just like this, rather than exporting the `native` global. Your LSP will
	behave better, and your extension will be more secure.
]]

local exports = {}

function exports.add(a: number, b: number): number
	return native.do_addition(a, b)
end

return exports