--[=[
	@class Foo
	@external Promise https://eryn.io/roblox-lua-promise/api/Promise
	@server
	@client
	@plugin
	@private
	@ignore
	@deprecated v1.2 -- Use x instead
	@since v1.0
	@tag bark
	@tag meow
	@tag rawr

	Here's a description for you

	::: info
	with an admonition
	:::
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
	@param promise Promise -- external param
	@param status Status -- external param
	@return a -- return a
	@return b -- return b
	@return Promise -- return external type
	@return Status -- return external type
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
	@external Promise https://eryn.io/roblox-lua-promise/api/Promise
	@external Status https://eryn.io/roblox-lua-promise/api/Promise#Status

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

--[=[
	@interface Command
	@within Foo
	@private
	@ignore
	@tag cmdr
	@tag lua
	@external Promise https://eryn.io/roblox-lua-promise/api/Promise
	@external Status https://eryn.io/roblox-lua-promise/api/Promise#Status

	.Name string -- the name of the command
	.Groups array<string> -- A list of groups that the command contains
	.Recursion Command -- This breaks the universe
	.Promise Promise -- This is a Promise
	.Status Status -- Let's ignore that Promise:getStatus() exists

	An object describing a command
]=]
