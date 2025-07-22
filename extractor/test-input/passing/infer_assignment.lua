--- @class InferAssignment
local InferAssignment = {}

-- TableConstructor
--- @within InferAssignment
InferAssignment.world = {
	gravity = 9.8,
	time = "relative"
}

-- Number
--- @within InferAssignment
InferAssignment.year = 2025

-- InterpolatedString
--- @within InferAssignment
InferAssignment.copyright = `Copyright (c) {InferAssignment.year}`

-- Function
--- @within InferAssignment
InferAssignment.on_year_change = function(year: number): ()
	InferAssignment.year = year
	InferAssignment.copyright = `Copyright (c) {InferAssignment.year}`
end

-- String
--- @within InferAssignment
InferAssignment.name = "InferAssignment"

-- Symbol > True
--- @within InferAssignment
InferAssignment.enabled = true

-- Symbol > False
--- @within InferAssignment
InferAssignment.DEBUG = false

-- Symbol > Nil
--- @within InferAssignment
InferAssignment.currently_spectating = nil

-- TypeAssertion > Callback + parameter name
--- @within InferAssignment
--- @return boolean -- whether it was successful
--- @yields
InferAssignment.announce_message_async = nil :: (message: string) -> boolean

-- TypeAssertion > string
--- @within InferAssignment
InferAssignment.version = nil :: string

-- Other TypeAssertion variants could be added but they all (hopefully) follow the exact same logic.