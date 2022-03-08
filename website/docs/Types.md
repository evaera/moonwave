---
sidebar_position: 2.5
---

# Writing Types

Writing types is an important part of using Moonwave. Types are used everywhere that you document something: a parameter type, a return type, a type alias, etc. 

In general, you should refer to the [Luau Typechecking Guide](https://luau-lang.org/typecheck). Anything that's valid in a type position there should work in Moonwave.

Here are some important notes:

## Arrow functions

The syntax for a function type is a parenthesized list of arguments, followed by an arrow `->`, followed by a parenthesized list of return types.

- `(firstArg: number, secondArg: string) -> (string, number)`

Functions with a single return value can omit the parentheses: 

- `(firstArg: number, secondArg: string) -> string`

Functions that return nothing should be denoted with a set of empty parentheses:

- `(firstArg: number, secondArg: string) -> ()`

So, a function that takes no arguments and returns nothing would look like:

- `() -> ()`

### Optional function arguments

Optional functional arguments can be denoted with either a union of a type and nil, or by putting a question mark after the argument name or type.

These are equivalent:
- `(arg: string | nil) -> ()`
- `(arg?: string) -> ()`
- `(arg: string?) -> ()`

## Arrays

Arrays should be denoted by surrounding any other type with curly braces.

An array of strings would be `{string}`. An array of numbers is `{number}`.

## Union types

When multiple types are valid, you can separate them with the pipe character `|`.

A function that takes a number or string and returns a string or boolean:

- `(arg: number | string) -> string | boolean`

An array of either numbers or nil:

- `{number | nil}`

## Generics

Generic types can be used by denoting type variables between angled brackets.

- `<T>(arg: T) -> (T, T)`
- `() -> Promise<T>`

The types do not necessarily need to be valid Luau types. For example, you may omit declaring `<T>` in a function, like so:

- `(arg: T) -> (T, T)` (note: missing `<T>` at the beginning)

Docs are for reading, not running, so if you think omitting information will make your type more clear, go for it.

### Type variable convention

In general, you can use `T`, `U`, or any other single capital letter in any type position to constrain two types into being the same, without necessarily prescribing what that type is.

You could write a function type like so:

- `(arg: T) -> T`

Which means: A function that takes any type, but whatever type you give it, that's the type that will be returned.

This is different from `(arg: any) -> any`, because in that case, the function doesn't necessarily have to return the same type that is passed in, you could return anything. But by specifying `T` in both places, you're indicating that the two types are the same, whatever they might be. 

For an example of this, check out the [Promise.fold](https://eryn.io/roblox-lua-promise/api/Promise#fold) docs.
