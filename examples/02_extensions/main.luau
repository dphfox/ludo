--[[
	Ludo is extensible with native code. To do this, add a `native` field to
	your `.ludorc`; see `hello_extension` for an example.

	By default, you may not require a script with `native` access unless you
	explicitly grant permission in your own `.ludorc`. This is done for security
	reasons; you should minimise the number of native extensions used.
]]

local hello_extension = require("@hello_extension")
print(hello_extension.add(2, 2))