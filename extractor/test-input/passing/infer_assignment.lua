--- @class InferAssignment
local InferAssignment = {}

--- TableConstructor
InferAssignment.world = {
	gravity = 9.8,
	time = "relative"
}

--- Number
InferAssignment.year = 2025

--- InterpolatedString
InferAssignment.copyright = `Copyright (c) {InferAssignment.year}`

--- Function
InferAssignment.on_year_change = function(year: number): ()
	InferAssignment.year = year
	InferAssignment.copyright = `Copyright (c) {InferAssignment.year}`
end

--- String
InferAssignment.name = "InferAssignment"

--- Symbol > True
InferAssignment.enabled = true

--- Symbol > False
InferAssignment.DEBUG = false

--- Symbol > Nil
InferAssignment.currently_spectating = nil

--- TypeAssertion > Callback + parameter name
--- @return boolean -- whether it was successful
--- @yields
InferAssignment.announce_message_async = nil :: (message: string) -> boolean

--- TypeAssertion > string
InferAssignment.version = nil :: string

-- Other TypeAssertion variants could be added but they all (hopefully) follow the exact same logic.

--- @within InferAssignment
local timestamp = 1753211834

-- Other local assignment values would be added but they all (hopefully) follow the exact same logic.

--- @class InferAssignment.upcoming
InferAssignment.upcoming = {}

--- This will be a Crazy update.
InferAssignment.upcoming.version = "1.0.0"