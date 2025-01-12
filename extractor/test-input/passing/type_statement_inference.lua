--- @class TypeStatementInference
--- Infer types from type statements.

--- @within TypeStatementInference
--- Table gets converted to interface.
type baseData = {
	id: number
}

--- @within TypeStatementInference
--- Everything else uses a basic string representation.
type pet = "cat" | "dog"

--- @within TypeStatementInference
--- Unions and intersections are not marked as interfaces.
type petData = baseData & {
	pet: pet
}

--- @within TypeStatementInference
--- Exported types work the same.
export type response = petData?

--- @within TypeStatementInference
--- @type petFetcher (id: number) -> response
--- \@type can override.
type randomFetcher = any

--- @within TypeStatementInference
--- @interface petLibrary
--- @field toPet (baseData) -> petData
--- \@interface can override.
type randomLibrary = any