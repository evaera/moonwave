local RunService = game:GetService("RunService")

local DEFAULT_NAMESPACE = "game"

local Promise = require(script.Parent.Parent.Promise)

local UnitCollection = require(script.UnitCollection)
local Reactor = require(script.Reactor)
local Serializer = require(script.Serializer)
local HotReloader = require(script.HotReloader)
local Symbol = require(script.Parent.Shared.Symbol)

--[=[
	@class Fabric

	-.reducers Reducers -- This is a property that is set to the reducers.
]=]

--[=[
	@type stringOrNil string | nil
	@within Fabric

	A string or nil.
]=]

--[=[
	@interface Instance
	@within Fabric

	A Roblox Instance.

	.Name string -- A name for this Instance.
	.ClassName string -- The class name for this
	.CollisionGroup number
	@field Blargh boolean -- This uses @field explicitly
]=]

--[=[
	@prop reducers Reducers
	@within Fabric
	@readonly

	This is a property that is set to the reducers.
]=]
local Fabric = {
	reducers = require(script.Operators.Reducers);
	comparators = require(script.Operators.Comparators);
	t = require(script.Parent.Parent.t);
	DEBUG = true;
	Heartbeat = RunService.Heartbeat;
	None = Symbol.named("None");
	Unit = setmetatable({}, {
		__index = function(_, key)
			error(("Unit %q is not registered!"):format(key))
		end
	});
}
Fabric.__index = Fabric

--[=[
	Creates a new instance of Fabric.

	@param namespace string -- A unique namespace to distinguish from other instances of Fabric for network calls.
	@return Fabric
]=]
function Fabric.new(namespace)
	local self = setmetatable({
		namespace = namespace or DEFAULT_NAMESPACE;
		_listeners = {};
	}, Fabric)

	self.serializer = Serializer.new(self)
	self._collection = UnitCollection.new(self)
	self._reactor = Reactor.new(self)

	if RunService:IsStudio() then
		self._hotReloader = HotReloader.new(self)
	end

	return self
end

--[=[
	Registers a unit. This function should be called before attempting to get or create the unit.

	@param unitDefinition UnitDefinition -- The definition of the unit
	@return UnitDefinition -- The passed unit definition
]=]
function Fabric:registerUnit(unitDefinition)
	assert(unitDefinition ~= nil, "unitDefinition is nil")
	self._collection:register(unitDefinition)

	self:fire("unitRegistered", unitDefinition)

	return unitDefinition
end

--[=[
	Registers all units that are immmediate children of a container.
	Skips any test scripts (i.e. name of form `*.spec`) in the container.

	@param container Instance -- The container
	@return nil
]=]
function Fabric:registerUnitsIn(container)
	for _, object in ipairs(container:GetChildren()) do
		if object:IsA("ModuleScript") then
			if not object.Name:match("%.spec$") then
				local unitDefinition = require(object)

				if unitDefinition.name == nil then
					unitDefinition.name = object.Name
				end

				self:registerUnit(unitDefinition)

				if self._hotReloader then
					self._hotReloader:giveModule(object, unitDefinition)
				end
			end
		else
			self:registerUnitsIn(object)
		end
	end
end

--[=[
	Returns the unit associated with a unit resolvable that is attached to a ref,
	or nil if it doesn't exist.

	@param unitResolvable UnitResolvable -- The unit to retrieve
	@param ref Ref -- The ref to retrieve the unit from
	@return Unit? -- The attached unit
]=]
function Fabric:getUnitByRef(unitResolvable, ref)
	return self._collection:getUnitByRef(unitResolvable, ref)
end

--[=[
	Returns the unit associated with a unit resolvable that is attached to ref.
	If it does not exist, then creates and attaches the unit to ref and returns it.

	@param unitResolvable UnitResolvable -- The unit to retrieve
	@param ref Ref -- The ref to retrieve the attached unit from
	@return Unit -- The attached unit
]=]
function Fabric:getOrCreateUnitByRef(unitResolvable, ref)
	return self._collection:getOrCreateUnitByRef(unitResolvable, ref)
end

function Fabric:getLoadedUnitByRef(unitResolvable, ref)
	local unit = self._collection:getUnitByRef(unitResolvable, ref)

	if unit == nil then
		error(("Attempt to get loaded unit %q on %s, but it does not exist."):format(
			tostring(unitResolvable),
			tostring(ref)
		))
	end

	if not (unit._loaded or unit._loading) then
		error(("Attempt to call getLoadedUnitByRef on %q on %s, but it will never be loaded."):format(
			tostring(unitResolvable),
			tostring(ref)
		))
	end

	return Promise.new(function(resolve, reject)
		if unit._loaded then
			return resolve(unit)
		else
			unit:on("loaded", function()
				resolve(unit)
			end)

			-- This must be fired by the user. It's not fired anywhere inside the Fabric library.
			unit:on("loadingFailed", function(...)
				reject(...)
			end)
		end
	end)
end

--[=[
	Removes all units attached to the passed ref.

	@param ref Ref -- The ref to remove all units from
	@return nil
]=]
function Fabric:removeAllUnitsWithRef(ref)
	self._collection:removeAllUnitsWithRef(ref)
end

--[=[
	Fires a fabric event.

	@param eventName string -- The event name to fire
	@param ... any -- The arguments to fire the event with.
	@return nil
]=]
function Fabric:fire(eventName, ...)
	if not self._listeners[eventName] then
		return -- Do nothing if no listeners registered
	end

	for _, callback in ipairs(self._listeners[eventName]) do
		local success, errorValue = coroutine.resume(coroutine.create(callback), ...)

		if not success then
			warn(("Event listener for %s encountered an error: %s"):format(
				tostring(eventName),
				tostring(errorValue)
			))
		end
	end
end

--[=[
	Listens to a fabric event.

	@param eventName string -- The event name to listen to
	@param callback function -- The callback fired
	@return nil
]=]
function Fabric:on(eventName, callback)
	self._listeners[eventName] = self._listeners[eventName] or {}
	table.insert(self._listeners[eventName], callback)

	return function()
		for i, listCallback in ipairs(self._listeners[eventName]) do
			if listCallback == callback then
				table.remove(self._listeners[eventName], i)
				break
			end
		end
	end
end

--[=[
	Logs a debug message. Set fabric.DEBUG = true to enable.

	@param ... any -- The debug information to log
	@return nil
]=]
function Fabric:debug(...)
	if self.DEBUG then
		warn("[Fabric]", ...)
	end
end

return Fabric