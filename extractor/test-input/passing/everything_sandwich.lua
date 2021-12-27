--[==[
	@type CommandArgument

	This is the description

	@prop Type string
	@prop Name string
	@prop Description string
	@prop Optional boolean?
	@prop Default any
	@prop callback nil | (name: string, id: number) => part -- The part you need to go to

	@param name string = "hi" --

	@function
	@function asdf
]==]

--[==[
	@interface CommandArgument
	@field name string -- The name of the argument
	.name string -- The name of the argument
	.desc string -- The description of the argument
]==]

--[=[
	@class theclass
	@unreleased

	This is the class
]=]


--[=[
	@function TheFunction

	Does the stuff

	@within theclass
	@param thename the type -- the description
	@param thenameredux the type but twice -- the description again
]=]

--[=[
	@class Module

	This is a module lcass with thsf
]=]

--[=[
	Creates a new instance of Module.

	@deprecated 1.12 -- Use [[Module.somethingElse]] instead
	@param name string -- This is the name for this Module.
	@return Module -- Returns the new Module!
	@tag this is a tag
	@tag this is another tag
	@server
	@client
	@error "Bad" -- This can error with the text Bad if there's a bad error.
	@error Error<F> -- THis can error with an ERror object also. Maybe. We don't really know
]=]
function Module.new(name)

end

--[=[
	Gets a thingy


	@param name string -- This is the name for this Module
	@return Module - Returns the new Module!
]=]
function Module:get(name)

end
