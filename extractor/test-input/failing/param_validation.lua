local test

--[=[
	missing doc
]=]
function test:missingDoc(parameter) end

--[=[
	extra param

	@param fake number -- this is a fake param
]=]
function test:extraParam(real: string) end
