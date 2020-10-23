
--[==[
    @type CommandArgument
    @prop Type string
    @prop Name string
    @prop Description string
    @prop Optional boolean?
    @prop Default any
    @prop callback nil | (name: string, id: number) => part -- The part you need to go to
]==]


--[=[
    Creates a new instance of Module.

    -@deprecated 1.12 -- Use [[Module.somethingElse]] instead
    @param name string -- This is the name for this Module.
    -@return Module -- Returns the new Module!
]=]  
function Module.new(name)

end

--[=[
    Gets a thingy



    -@static
    @param name string - This is the name for this Module
    -@returns Module - Returns the new Module!
]=]
function Module:get(name)

end
