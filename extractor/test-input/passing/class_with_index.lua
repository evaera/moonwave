--[=[
	@class abc
]=]

--[=[
	A method within abc's __index
]=]
function abc.__index:methodName()

end

--[=[
	@class xyz
	@__index prototype
]=]

--[=[
	A method within xyz's __index
]=]
function xyz.prototype:methodName()

end