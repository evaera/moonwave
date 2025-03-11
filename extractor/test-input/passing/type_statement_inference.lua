--- @class TypeStatementInference
--- Infer types from type statements.

--- @within TypeStatementInference
--- Tables get converted to interfaces.
type baseData = {
	id: number
}

--- @within TypeStatementInference
--- Everything else uses a basic string representation.
type pet = "cat" | "dog"

--- @within TypeStatementInference
--- Anything not strictly a table (union, intersection, et cetera) is not marked as an interface.
type petData = baseData & {
	pet: pet
}

--- @within TypeStatementInference
--- Exported types work the same as non exported.
export type response = petData?

--- @within TypeStatementInference
--- Exported interfaces work the same as non exported.
export type responseFull = {
	method: "GET",
	body: petData?
}

--- @within TypeStatementInference
--- @type petFetcher (id: number) -> response
--- `@type` can override.
type randomFetcher = any

--- @within TypeStatementInference
--- @interface petLibrary
--- @field toPet (baseData) -> petData
--- @field getDatabase () -> petDatabase
--- `@interface` can override.
type randomLibrary = any

--- @within TypeStatementInference
--- Trivia is used to add field descriptions.
type petStorage = {
	entries: {petData}, --- Maximum size of 100.

	--- Current size of `entries`.
	size: number,

	lastAdded: number --- Unix timestamp.
}

--- @within TypeStatementInference
--- Extra test for leading unpunctuated trivia.
type petStorageUnreleased = {
	--- Unix timestamp. Changes for insertions and deletions.
	lastSizeChanged: number
}

--- @within TypeStatementInference
--- @field get (id: number) -> responseFull
--- @field set -- `id` must match `data.id`
--- `@field` can overwrite information on existing fields.
type petDatabase = {
	get: (id: number) -> pet, --- Cached in [petStorage].
	set: (id: number, data: petData) -> () --- This comment will be removed.
}