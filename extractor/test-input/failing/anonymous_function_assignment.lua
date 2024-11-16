--- @class Class
--- Multiple variables and expressions leads to undesired behavior

local class = {}

--- @within Class
--- Multiple variables should fail
class.foo, class.bar = function(x: number): number end, function(y: string): string end

--- @within Class
--- Multiple variables should fail
local freeFunctionA, freeFunctionB = function(x: number): number end, function(y: string): string end

--- @within Class
--- Multiple variables should fail
baz, qux = function(x: number): number end, function(y: string): string end

--- @within Class
--- Multiple variables should fail
local a, b = function(x: number): number end, function(y: string): string end

--- @within Class
--- Multiple expressions should fail
quux = function(x: number): number end, function(y: string): string end

--- @within Class
--- Multiple expressions should fail
local fum = function(x: number): number end, function(y: string): string end
