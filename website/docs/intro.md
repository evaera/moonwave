---
sidebar_position: 1
---

# Getting Started with Moonwave

Getting started with Moonwave is easy. You can generate a website with little-to-no configuration and just a few comments in your Lua code.

Moonwave is a modular project: while Moonwave is designed to work out of the box, you could also use it to extract your docs as JSON and use it for whatever you want. It's up to you!

The rest of this guide will get you started using the Moonwave command line, which lets you generate and customize your Lua docs website without any prior knowledge.

## Installation

Ensure you have [Node.js](https://nodejs.org/en/) v14+ installed on your computer.

1. Open a terminal, like Command Prompt, Powershell, or [Windows Terminal](https://www.microsoft.com/en-us/p/windows-terminal/9n0dx20hk701) (recommended)
2. Run the command `npm i -g moonwave`.

:::tip
This will install the current version of Moonwave on your system. You should regularly update your Moonwave installation to receive new features and bug fixes. You can do this by running `npm i -g moonwave@latest`.
:::

If you always want the most up to date version, you can skip installing Moonwave and use `npx moonwave` instead of `moonwave` in the following steps. This will always download the latest version of Moonwave before running the command.

## Use Moonwave with your project

Next, you need a Lua project to use Moonwave with.

:::tip
Moonwave automatically pulls some information about your project like Title and Author from your Git config. For this reason, it's recommended to use Git with the project you want to generate docs for. If you don't want to use Git for some reason, these options can be manually configured in the [moonwave.toml](Configuration) file.
:::

1. In your terminal, navigate to the folder containing your project. If you're using your editor's integrated terminal, you might already be in the right folder.
2. Run the command `moonwave dev`. This should open your new website in your browser!


:::info
By default, moonwave looks for your code in a folder called `src` or `lib`. If your source code is not in one of these folders, fear not: just specify the `--code` flag: `moonwave dev --code MyFolderHere`
:::

## Let's write some doc comments

In Moonwave, anything that can be documented (types, properties, or functions) must belong to a "class". A class in Moonwave means nothing more than a table that contains fields or functions: it has no specific meaning in the "Object-Oriented Programming" sense of the term.

A Moonwave class can be a "service", an OOP-style "class", or a even a plain table that contains methods or properties. It's up to you. The only thing to remember is that each Moonwave class will have its own API page.

### Your first class

Doc comments in Moonwave are denoted in one of two ways:

A multi-line comment with a single equals sign between the square brackets:

```lua
--[=[
	@class MyFirstClass

	This is my first class.
]=]
local MyFirstClass = {}
MyFirstClass.__index = MyFirstClass
```

**Or**, a set of single-line triple-dash comments:

```lua
--- @class MyFirstClass
---
--- This is my first class.
local MyFirstClass = {}
MyFirstClass.__index = MyFirstClass
```

These two examples are equivalent. Pick one, and put it in one of your Lua files. The `@class` doc comment can appear anywhere in your file, but it's conventional to put it right above the line where you declare your class name.

Now, your website should automatically update with your class in the "API" section on the navbar!

:::danger Not Working?
Are your changes not showing up? Try restarting Moonwave. Press <kbd>Ctrl</kbd> + <kbd>C</kbd> to stop the server, press the up arrow on your keyboard, and then press enter.
:::

### Documenting Functions

Next let's document a function within your class. This doc comment must be placed directly above your function definition in Lua.

```lua
--[=[
	This is a very fancy function that adds a couple numbers.

	@param a number -- The first number you want to add
	@param b number -- The second number you wanna add
	@return number -- Returns the sum of `a` and `b`
]=]
function MyFirstClass:add(a, b)
	return a + b
end
```

Save your file, and you should see your function as part of your class documentation. If not, check the console - there might be errors.

## Next steps

Now you know the basics of Moonwave. Keep reading the docs:

- [List of all tags](TagList)
- [How to write types](Types)
- [Publishing your website](Publishing)
- Check out an example of a project using Moonwave: [roblox-lua-promise](https://github.com/evaera/roblox-lua-promise/blob/master/lib/init.lua)