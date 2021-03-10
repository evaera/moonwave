--[=[
	@class Foo
	@server
	@client
	@private
	@ignore
	@deprecated v1.2 -- Use x instead
	@since v1.0
	@tag bark
	@tag meow
	@tag rawr

	Here's a description for you
]=]
--[=[
	@class Bar
	@unreleased
]=]

--[=[
	@function new
	@within Foo
	@param a A A A -- param a
	@param b B B B -- param b
	@return a -- return a
	@return b -- return b
	@server
	@client
	@private
	@ignore
	@yields
	@deprecated v1.5 -- Use something else
	@since v1.23
	@tag uno
	@tag dos
	@error c -- this errors sometimes. shrug

	This function creates a new Foo
]=]
--[=[
	@function comingSoon
	@within Bar
	@unreleased
]=]

--[=[
	@prop ready boolean
	@within Foo
	@server
	@client
	@private
	@ignore
	@readonly
	@deprecated v1.3 -- Use blah
	@since v0.1.1
	@tag salad
	@tag fries
	@tag nuggets
]=]
--[=[
	@prop notReady boolean
	@within Bar
	@unreleased
]=]

--[=[
	@type nilOrNumber nil | number
	@within Foo
	@private
	@ignore
	@tag arrgh
	@tag yarr

	Nil or number matey
]=]