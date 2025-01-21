--- @class MissingFields
--- Fail to add information for fields that do not exist.

--- @within MissingFields
--- @field reason -- Ratelimit or id not found.
type petError = {
	name: string
}