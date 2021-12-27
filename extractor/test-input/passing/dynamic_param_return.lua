--- @class test

local test

--[=[
	auto type
]=]
function test:autoType(x: number, y: { complicated: string, ty: { x: number, y: any } }) end

--[=[
	mixed

	@param y -- extra description for y
]=]
function test:mixed(x: number, y: string): boolean end

--[=[
	multiple returns
]=]
function test:multipleReturns(x: number): (number, string, boolean) end

--[=[
	variadic
]=]
function test:variadic(x: number, ...: string): ...boolean end

--[=[
	question mark

	@param ...? -- example yo
]=]
function test:questionMark(x: number, ...: string) end

--[=[
	free function

	@within test
	@param takes -- this is what it takes, and you ain't got it
]=]
local function free(rakes: number, takes: string): (number, boolean) end

--[=[
	weird formatting

	@within test
	@param secondLine -- A description
]=]
local function weirdFormatting(
	secondLine: number
) end