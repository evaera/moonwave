--[=[
	@class anotherclass

	This is yet another class
]=]

--[=[
	This is a function that uses the external type [Promise]

	@function exchangePromises
	@within anotherclass
	@external Promise https://eryn.io/roblox-lua-promise/api/Promise
	@param promise Promise -- this is your promise
	@return Promise -- and this is my promise
]=]

--[=[
	This is a function that uses two external types

	@function getPromiseStatus
	@within anotherclass
	@param promise Promise -- A promise
	@external Status https://eryn.io/roblox-lua-promise/api/Promise#Status
	@return Status -- The Status of the Promise
]=]
