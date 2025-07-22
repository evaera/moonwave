--- @class InferAssignment
local InferAssignment = {}

--- BinaryOperator
InferAssignment.book1984 = 2 + 2

--- Parentheses
--- Get it? Inside is an em dash, which can also be used for parenthetical clauses. A parenthetical symbol surrounded by parentheses 😹
InferAssignment.parenthetical = ("—")

--- UnaryOperator
InferAssignment.book4891 = -InferAssignment.book1984

--- FunctionCall
InferAssignment.null = string.char(0)

--- IfExpression
InferAssignment.uh_oh = if InferAssignment.book1984 == 5 then "gg" else "yippee!"

--- Symbol > Ellipsis
--- `...` is populated with program arguments in the demo Luau runtime.
InferAssignment.program_argument = ...

--- TypeAssertion > Callback + missing parameter name
InferAssignment.get_book_name_by_id = nil :: (number) -> string

--- Var
InferAssignment.uh_oh_part_2 = math.sqrt(-1)