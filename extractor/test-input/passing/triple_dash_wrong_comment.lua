--- Lua implementation of Rust language's Result enum
--- @class Result

local Result = {}
Result.__index = Result

--- Returns true if value is equal to the `Ok` value
--- @param value any
--- @return boolean
function Result:contains(value): boolean
	return self._value == value
end

--- hi
function Result:containsErr(value: any): boolean
	--
end
