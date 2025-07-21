--- @class ECAndMaybeS
local E_C_And_Maybe_S = {}

-- [what user specifies] on [what program detects]
-- regular = user does not specify

-- static on static
--- @function log_message
--- @within ECAndMaybeS
--- @param message -- will be prefixed with "[ECAndMaybeS]: "
function E_C_And_Maybe_S.log(message: string): ()

end

-- static on method = error

-- method on static
--- @method entity
--- @within ECAndMaybeS
--- @param size -- size of entity in bytes
--- @return number -- entity id
local function world_entity(skip, size: number): number
	
end

-- method on method
--- @method get_status
--- @within ECAndMaybeS
--- @param entity -- entity id from [entity](ECAndMaybeS#entity)
function E_C_And_Maybe_S:status(entity: number): ()

end

-- regular static
--- @within ECAndMaybeS
--- @param components -- whether components should be counted
--- @return number -- total amount of entities
function E_C_And_Maybe_S.entity_total(components: boolean): number

end

-- regular method
--- @within ECAndMaybeS
--- @param ... -- components
--- @return ...any -- data
function E_C_And_Maybe_S:query(...: number): ...any

end