--- @class ECAndMaybeS
local E_C_And_Maybe_S = {}

--- @method entity_properties_async
--- @within ECAndMaybeS
--- @yields
function E_C_And_Maybe_S:entity_properties_async(size: number, name: string): (number, boolean)

end

--- @method query
--- @within ECAndMaybeS
function E_C_And_Maybe_S.query(name1: string, name2: string): ()

end

--- @function query
--- @within ECAndMaybeS
function E_C_And_Maybe_S:is_component(id: number): ()

end

--- @method entity_simple
--- @within ECAndMaybeS
local function new_entity(self: E_C_And_Maybe_S): number
	
end

-- Ensures existing behaviour is kept.
--- @within ECAndMaybeS
--- @param size -- The size of the entity.
--- @param name -- The name of the entity.
--- @return number -- How much space the entity actually takes up.
function E_C_And_Maybe_S:entity_precise(size: number, name: string): number

end