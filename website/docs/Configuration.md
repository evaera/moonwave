---
sidebar_position: 4
---

# Configuration

## moonwave.toml

You can customize how Moonwave behaves by creating a `moonwave.toml` file in the root of your project.

Here's a sample `moonwave.toml` with default properties. Every property listed here is optional.

:::tip
Lines ending with the comment `# From Git` mean that property is automatically filled in from your Git repository.
:::

```toml
title = "My Project Name"  # From Git
gitRepoUrl = "https://github.com/UpliftGames/moonwave" # From Git
gitSourceBranch = "master"
changelog = true
classOrder = []

[docusaurus]
organizationName = "organizationName" # From Git
projectName = "projectName" # From Git
url = "https://organizationName.github.io" # From Git
tagline = "Your project's tagline"
baseUrl = "/projectName" # From Git
onBrokenLinks = "throw"
onBrokenMarkdownLinks = "warn"
favicon = ""

[[navbar.items]]
href = "https://discord.gg/abcdefghijk"
label = "Discord"
position = "right"

[[navbar.items]]
href = "https://???"
label = "Something Else"

[home]
enabled = false
includeReadme = false

[[home.features]]
title = "Feature 1"
description = "This is a feature"
image = "https://url

[[home.features]]
title = "Feature 2"
description = "This is a second feature"
image = "https://url

[footer]
style = "dark"

[footer.links]
title = ""
copyright = "Copyright © 2021 organizationName. Built with Moonwave and Docusaurus"
```

## Docusaurus options

The options in the `[docusaurus]` section are directly passed to the Docusaurus options. For more information, please see the [Docusaurus docs](https://docusaurus.io/docs/docusaurus.config.js)

## API Class Order

You can customize the order that classes appear in the API section with the `classOrder` option. In `moonwave.toml`, specify:

```toml
classOrder = [
	"MyClass",
	"Sample"
]
```

Any classes not listed here will be alphabetized and added to the end of the list. Listing a class that doesn't exist is an error.

## Custom home page

By default your project's README is used as the homepage. To use a custom homepage, simply set `enabled` to `true` in the `[home]` section:

```toml
[home]
enabled = true
includeReadme = true # Optional

[[home.features]]
title = "Feature 1"
description = "This is a feature"
image = "https://url

[[home.features]]
title = "Feature 2"
description = "This is a second feature"
image = "https://url
```

Optionally, you can include `includeReadme = true`, which will append your project's README to the end of the home page.

If your project's README begins with some content that you don't want included in your home page, you can place an HTML comment in your project's README that will not include any content before it.

```html
Project Logo, Project Name, Etc
<!--moonwave-hide-before-this-line-->
My project is amazing and it does everything you could ever want.
```

Only the content underneath the HTML comment will be included in your Moonwave homepage.