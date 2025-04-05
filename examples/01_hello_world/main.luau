--[[
	Ludo programs include all of Luau's basic functionality as you'd expect.
]]

-- Luau modules can be required relative to the script.
local module = require("./module")

-- Standard output supports pretty printing.
print("Hello, world:", module)

-- All of Luau's basic libraries are here.
local items = table.create(10, 1)
for i = 3, 10 do
	items[i] = items[i - 2] + items[i - 1]
end

local message = "hello"
for from, to in utf8.graphemes(message) do
	print(message:sub(from, to))
end

-- Error utilities and protected mode calls.
print("Problematic code:", pcall(function()
	error("oops")
end))

xpcall(function(a, b)
	assert(a == b)
end, function(err)
	print("That didn't work...", err)
end, 2, "fish")

-- Ludo disables these deprecated or unsafe functions.
-- Consider using an extension if you need these features.
assert(loadstring == nil)
assert(setfenv == nil)
assert(getfenv == nil)