---
sidebar_position: 2
---

# List of Tags

This page lists all of the doc tags that Moonwave supports. Tags can take various arguments or be exclusive with or require other tags, so make sure you check the description of each one to understand what it does and how to use it.

:::info
Some tags take arguments. These arguments are described with the following pattern:

- `<angled brackets>` denote **required** arguments.
- `[square brackets]` denote **optional** arguments.

You should **not include these** brackets when using these tags in your code.
:::

Make sure you check out the [Writing Types](Types) guide for more information on how to write types.

## Doc Comments

Doc comments in Moonwave are denoted in one of two ways: A multi-line comment with a single equals sign between the square brackets, or a set of single-line triple-dash comments. Either one is acceptable, and these docs will interchangeably use both styles.

:::tip
Any line that doesn't begin with `@` or `.` within your doc comment is part of the description. You can use [Markdown](https://docusaurus.io/docs/markdown-features) in the description, including [code blocks](https://docusaurus.io/docs/markdown-features/code-blocks) and [admonitions](https://docusaurus.io/docs/markdown-features/admonitions) (this text is in an admonition right now!).
:::

:::tip Short links
- You can use the `[ClassName]` short link syntax to link to classes from within doc comments.
- You can use `[ClassName:method]` or `[ClassName.member]` to link directly to a member of another class
- You can also link to Roblox classes, like `[CFrame]` or `[Part]`.
:::

Doc Comments are always one of four types: class, function, property, or type. Each of these has its own respective tag that turns the doc comment they appear in to that type of doc comment. You should only have one of these per doc comment.

### @class
:::note Usage
`@class <name>`
:::

Denote a class. Generally, it's a good idea to have one per `.lua` file. 

Conventionally this tag should appear right before the class it's documenting is defined, but it can appear anywhere:
```lua
--- @class MyClass
--- A sample class.
local MyClass = {}
MyClass.__index = MyClass
```

### @within
:::note Usage
`@within <class name>`
:::

Doc comments other than `@class` require a `@within` tag describing what class they belong to. Having type, property, or function without belonging to a class is an error.

### @prop
:::note Usage
`@prop <name> <type>`
:::

Denotes a property with a required type of the property. These can appear anywhere in the file, but should usually go near the class they belong to.

```lua
--- @prop name string 
--- @within MyClass
--- A string referring to the name of this thing
--- (this is an example description)
```

### @type
:::note Usage
`@type <name> <type>`
:::

Denotes a type. This is an abstract concept: types that are associated with classes are really just a way to assign a name to a commonly used type. Imagine if you had a library that referred to tables with `x`, `y`, and `z` fields often. It's easier to write that type once and then refer to it later.

```lua
--- @type ArrayOfStringsOrNil {string} | number | nil
--- @within MyClass
--- An array of strings, a number, or nil.
```

You can then use this type in other positions, like a property or to a parameter of a function.

### @interface
:::note Usage
`@interface <name>`
:::

The `@interface` tag refers to the type of a table. It can be used to document the fields of a table that's commonly used in your project.

Fields of the interface are written in the description with lines beginning with a period (`.`), immediately followed by the field name. If preferred, the `@field` tag may also be used.

```lua
--- @interface Command
--- @within MyClass
--- .Name string -- The name of the command
--- .Groups {string} -- A list of groups the command contains
--- .Recursion Command -- This breaks the universe
--- @field ID number -- The @field tag is equivalent to the dot-syntax.
---
--- An object describing a command.

```

### @function
:::note Usage
`@function <name>`

`@method <name>`
:::

The `@function` tag should only be used to document functions that do not actually appear in your file or are automatically generated.

Alternatively, the `@method` tag may be used to indicate a method (invoked with `:` instead of `.`).

:::tip
**As of Moonwave v0.3.0**...

- Free functions no longer require the use of `@function` - only `@within`.
- Parameters and return types are automatically detected in functions, so documenting them manually is not required if
they already have Luau types.
:::

By default, when you place a doc comment above a function, Moonwave will automatically detect that it is a function doc comment so using `@function` is not required.

```lua
--[=[
	This is a very fancy function that adds a couple numbers.

	@param a number -- The first number you want to add
	@param b number -- The second number you wanna add
	@return number -- Returns the sum of `a` and `b`
]=]
function MyClass:add(a, b)
	return a + b
end
```

Or using the `@method` tag (note that the function definition is missing):

```lua
--[=[
	This is a very fancy function that adds a couple numbers.

	@method add
	@within MyClass
	@param a number -- The first number you want to add
	@param b number -- The second number you wanna add
	@return number -- Returns the sum of `a` and `b`
]=]
```

## The Tag Tag
Yes, you read that right: there's a `@tag` tag.

### @tag
:::note Usage
`@tag <tag name>`
:::

The `@tag` tag lets you add *"tags"* to your class members that are displayed visually alongside the item name.

For example, imagine you had a class that abstracted over Players and NPCs in your game. You might have functions that only work for players and functions that only work for NPCs. In that case, the `@tag` tag is useful for communicating that quickly to your readers.

```lua
--[=[
	Paths the AI to a specific spot
	@tag NPC
]=]
function Character:navigate()
end

--[=[
	Kicks the player from the game.
	@tag Player
]=]
function Character:kick()
end
```

## Function tags

This section describes tags that may only be used in function doc comments.

### @yields
:::note Usage
`@yields`
:::

Indicates that this is a yielding function.

```lua
--[=[
	This function takes a while.
	@yields
]=]
function MyClass:wait()
	task.wait(1)
end
```

### @param
:::note Usage
`@param <name> [type] -- [description]`
:::

Describes a parameter for a function. This tag can appear multiple times in a doc comment, and each parameter should have its own.

:::tip
Parameter names and types are automatically detected when using Luau type annotations, so using the `@param` tag is only required if you want to specify a description.

The `@param` `type` argument is optional if the parameter type is specified inline with Luau type annotations. You can still specify it to override.

As of Moonwave v0.3.0, having an undocumented parameter, or an extra `@param` tag with no corresponding Lua parameter, is an error.
:::

The `@param` tag begins with the parameter name, followed by a space and the type. Optionally, this can be followed by two dashes (`--`) and a description. Markdown is parsed in the description.

```lua
--[=[
	@param a number -- The first number you want to add
	@param b number
]=]
function MyClass:doSomething(a, b)
end

--[=[
	Example of only specifying description, using the auto-detected Luau type annotation.

	@param myParam -- Description of myParam
]=]
function MyClass:typeAnnotationExample(myParam: string)
end
```

### @return
:::note Usage
`@return <type> -- [description]`
:::

Describes a return value for a function. This tag can appear multiple times in a doc comment, and each return value should have its own.

:::tip
Return types are automatically detected when using Luau type annotations, so using the `@return` tag is only
required if you want to specify a description.

If you choose to use the `@return` tag, you must specify all returns with `@return` doc tags. Any return types that were automatically detected are discarded if *any* `@return` tag is manually specified.
:::

The `@return` tag is followed by the type of the return. Optionally, this can be followed by two dashes (`--`) and a description. Markdown is parsed in the description.

```lua
--[=[
	@return number -- Some number
	@return number
]=]
function MyClass:doSomething()
	return 1, 2
end
```

### @error
:::note Usage
`@error <type> -- [description]`
:::

Describes a potential error that this function could raise. This tag can appear multiple times in a doc comment, and each error type should have its own.

The `@error` tag is followed by the type of the error. Optionally, this can be followed by two dashes (`--`) and a description. Markdown is parsed in the description.

```lua
--- @error "Unknown" -- This error happens sometimes. We don't know why.
function MyClass:doSomething()
	error("Unknown")
end
```

## Usage tags

These tags can appear on any type of doc comment.

### @unreleased
:::note Usage
`@unreleased`
:::

Indicates that this item is unreleased and may only be usable in pre-release versions.

### @since
:::note Usage
`@since <version>`
:::

Documents what version of your library this function was added.

```lua
--- @since v1.2.3
function MyClass:recentFunction()
end
```

### @deprecated
:::note Usage
`@deprecated <version> -- [description]`
:::

Marks this item as deprecated. Requires the version the item was deprecated, optionally followed by two dashes and a description of what to use instead. Markdown is parsed in the description.

```lua
--- @deprecated v2 -- Use `goodFunction` instead.
function MyClass:badFunction()
end
```

## Realm tags

These tags indicate that an item is only available for use on the server, client, or within a plugin.

More than one of these can be specified at a time, so you can have a function that is only usable on the server or within a plugin, but not on the client, for example.

```lua
--- @server
--- @plugin
function MyClass:foo()
end
```

### @server
:::note Usage
`@server`
:::

Indicates that this item may only be used on the server.

### @client
:::note Usage
`@client`
:::

Indicates that this item may only be used on the client.

### @plugin
:::note Usage
`@plugin`
:::

Indicates that this item may only be used from within a plugin.

## Visibility tags

### @private
:::note Usage
`@private`
:::

Indicates that this item is private and should only be used by the library author or within the same class.

By default, items marked with `@private` are not shown, and must be enabled by checking the "Show private" box that appears at the top of the page.

### @ignore
:::note Usage
`@ignore`
:::

Indicates that this item's documentation should not appear on the generated website. You might want to still document an item so that other tools such as a language server or autocomplete have access to the information without making it publicly visible.

## Property tags

These tags can only appear on `@prop` doc comments.

### @readonly
:::note Usage
`@readonly`
:::

Indicates that this property is read-only and should not be modified by the user.

## Class tags

These tags can only appear on `@class` doc comments.

### @__index
:::note Usage
`@__index <name>`
:::

By default, Moonwave will detect functions denoted in both of these styles:

```lua
--- A function
function MyClass:method()
end

--- A function
function MyClass.__index:method()
end
```

Sometimes though, your `__index` table is not actually named `__index`. In that case, you can specify what it is called and Moonwave will detect the functions properly.

```lua
--- @class MyClass
--- @__index prototype
local MyClass = {}
MyClass.prototype = {}
MyClass.__index = MyClass.prototype

--- A function
function MyClass.prototype:method()
end
```