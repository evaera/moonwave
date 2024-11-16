--- @class Class

local class = {}

--- @within Class
class.foo = function(x: number): number end

--- @within Class
local freeFunction = function(x: number): number end

--- @within Class
bar = function(x: number): number end
