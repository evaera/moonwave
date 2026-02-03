---
sidebar_position: 4
---

# Configuration

## moonwave.toml

You can customize how Moonwave behaves by creating a `moonwave.toml` file in the root of your project.

Here's a sample `moonwave.toml` with default properties. Every property listed here is optional.

:::tip
Options annotated with the comment `# From Git` mean that property is automatically filled in from your Git repository.
:::

```toml
title = "MyProjectName"  # From Git
gitRepoUrl = "https://github.com/evaera/moonwave" # From Git

gitSourceBranch = "master"
changelog = true
classOrder = []

[docusaurus]
onBrokenLinks = "throw"
onBrokenMarkdownLinks = "warn"
favicon = ""

# From git:
organizationName = "AuthorName"
projectName = "MyProjectName"
url = "https://AuthorName.github.io"
baseUrl = "/MyProjectName"
tagline = "Your project's tagline"

[footer]
style = "dark"
copyright = "Copyright © 2021 AuthorName. Built with Moonwave and Docusaurus"

[[footer.links]]
title = "examples"

[[footer.links.items]]
label = "example"
href = "https://example.com/"

```

See [the publishing guide](/docs/Publishing) for details on the `url` and `baseUrl` options.

## Docusaurus options

The options in the `[docusaurus]` section are directly passed to the Docusaurus options. For more information, please see the [Docusaurus docs](https://docusaurus.io/docs/docusaurus.config.js)

## Custom navbar options

You can add custom navbar options like so:

```toml
[[navbar.items]]
href = "https://discord.gg/abcdefghijk"
label = "Discord"
position = "right"

[[navbar.items]]
href = "https://???"
label = "Something Else"
```

## API Class Order

You can customize the order that classes appear in the API section with the `classOrder` option. In `moonwave.toml`, specify:

```toml
classOrder = [
	"MyClass",
	"Sample"
]
```

Any classes not listed here will be alphabetized and added to the end of the list. Listing a class that doesn't exist is an error.

### Sections
You can categorize your API pages with sections. Instead of the above style, you can do this:
```toml
[[classOrder]]
section = "Section name"
classes = ["Class1", "Class2"]

[[classOrder]]
section = "Another section name"
classes = ["Class3", "Class4"]

[[classOrder]]
section = "Tag Section"
# You can add tagged classes with '@tag <tagName>' to a section like this
tag = "TagForClasses"

[[classOrder]]
# No section name will link classes at the root level of the sidebar
classes = ["Class5", "Class6"]

[[classOrder]]
section = "Yet another section name"
collapsed = false # Determines with the section grouping is collapsed or expanded on page load. Defaults to true.
classes = ["Class7", "ClassAte", "Class9"]
```

### Nested Sections
You can create hierarchical organization with nested sections:

```toml
[[classOrder]]
section = "Parent Section"

# Child section with classes
[[classOrder.items]]
section = "Child Section 1"
classes = ["Class1", "Class2"]

# Child section with tagged classes
[[classOrder.items]]
section = "Child Section 2"
tag = "childTag"

# Classes directly under Parent Section (no subsection)
[[classOrder.items]]
classes = ["Class3"]

# Deeper nesting with grandchild section
[[classOrder.items]]
section = "Child Section 3"

[[classOrder.items.items]]
section = "Grandchild Section"
classes = ["Class4", "Class5"]

# The original format still works alongside nested sections
[[classOrder]]
section = "Regular Section"
classes = ["Class6", "Class7"]
```

You can control the collapsed state at any nesting level:

```toml
[[classOrder]]
section = "Always Expanded"
collapsed = false

[[classOrder.items]]
section = "Always Collapsed Child"
collapsed = true
classes = ["Class1"]
```

Nested sections support all the same features as top-level sections, including class lists, tags, and collapse control, at any level of nesting.

#### Automatic Sections from Folders

You can use the `autoSectionPath` option to automatically categorize classes into sections based on the folder they are inside in your project.

```toml
autoSectionPath = "packages"
```

With this option set, folders inside `YOUR_REPO/packages` will be automatically used as section names. Folders may be `kebab-case`, `PascalCase`, `camelCase`, or `sentence case`: they are automatically converted to `Title Case` in the section name.

For example, a class defined in `YOUR_REPO/packages/thing-doer/init.lua` will automatically be placed in a section called `Thing Doer`.

### Table of Contents (TOC)

You can customize categories in the Table of Contents of each API page. To create a category, first tag all items that should be included in the category with the `@tag` tag.

```lua
--[=[
	This is a very fancy function that adds a couple numbers.

	@param a number -- The first number you want to add
	@param b number -- The second number you wanna add
	@return number -- Returns the sum of `a` and `b`
	@tag utility
]=]
function MyFirstClass:taggedFunction(a, b)
	return a + b
end
```

Then, specify those tags under the `apiCategories` option in your `moonwave.toml` file.

```toml
apiCategories = [
    "constructor",
    "utility",
    "random"
]
```

## Custom home page

By default your project's README is used as the homepage. To use a custom homepage, simply set `enabled` to `true` in the `[home]` section:

```toml
[home]
enabled = true
includeReadme = true # Optional
bannerImage = "https://url" # Optional

[[home.features]]
title = "Feature 1"
description = "This is a feature"
image = "https://url"

[[home.features]]
title = "Feature 2"
description = "This is a second feature"
image = "https://url"
```

Optionally, you can include `includeReadme = true`, which will append your project's README to the end of the home page.

If your project's README contains content that you don't want included in your home page, you can place HTML comments in your project's README to remove any content before/after the comment.

```html
All content behind this comment will be hidden.
<!--moonwave-hide-before-this-line-->

While everything in between both comments will be visible!

<!--moonwave-hide-after-this-line-->
And everything ahead of this comment will also be hidden.
```

You can also combine both HTML tags to hide things in the middle of your README.

```html
The beginning of my content will be visible.
<!--moonwave-hide-after-this-line-->

This content will be hidden.

<!--moonwave-hide-before-this-line-->
While the rest of my content will also be visible.
```
