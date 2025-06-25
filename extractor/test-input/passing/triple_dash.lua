---	This description
---			some indented text
---	Has a line in the middle, followed by trailing lines.
---
---	Double blank here
---
---	```lua
---	function test()
---		print("indentation")
---
---		do
---
---			print("more indented")
---		end
---	end
---	```
---
---
---	@class TripleDash

--- not present in output
--[=[
	@class MixedComments
]=]
--- some more text
--- @type comment string
--- @within MixedComments

--- @within TripleDash
--- @param amount number -- How many more dashes to add.
function extra_dash(amount) end
