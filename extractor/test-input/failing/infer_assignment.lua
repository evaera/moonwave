--- @class InferAssignment
local InferAssignment = {}

-- BinaryOperator
--- @within InferAssignment
InferAssignment.book1984 = 2 + 2

-- Parentheses
--- @within InferAssignment
--- Get it? Inside is an em dash, which can also be used for parenthetical clauses. A parenthetical symbol surrounded by parentheses 😹
InferAssignment.parenthetical = ("—")

-- UnaryOperator
--- @within InferAssignment
InferAssignment.book4891 = -InferAssignment.book1984

-- FunctionCall
--- @within InferAssignment
InferAssignment.null = string.char(0)

-- IfExpression
--- @within InferAssignment
InferAssignment.uh_oh = if InferAssignment.book1984 == 5 then "gg" else "yippee!"

-- Symbol > Ellipsis
-- `...` is populated with program arguments in the demo Luau runtime.
--- @within InferAssignment
InferAssignment.program_argument = ...

-- TypeAssertion > Callback + missing parameter name
--- @within InferAssignment
InferAssignment.get_book_name_by_id = nil :: (number) -> string

-- Var
--- @within InferAssignment
InferAssignment.uh_oh_part_2 = math.sqrt(-1)