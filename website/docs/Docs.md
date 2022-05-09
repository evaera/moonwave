---
sidebar_position: 3
---

# Docs, Blog and Custom Pages

Your Moonwave project can have Markdown documentation and a blog, too.

## Markdown Docs
Simply create a folder called `docs` in the root of your project and throw some Markdown files in there.

Your first markdown file should always be called `intro.md` - this is the file we will link to in the nav bar.

For an example of this, check out the [default Docusaurus structure](https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/docs).

### Sidebar order

You can customize the order of your sidebar by adding front matter to your Markdown files.

```md
---
sidebar_position: 1
---

# My title
```

### Nested sections

You can create nested doc sections by simply creating a subfolder inside `docs` and putting markdown files in it.

You can customize the name and position of the category by creating a `_category_.json` file and putting in it:

```json
{
  "label": "Tutorial",
  "position": 3
}
```

Optionally, you can also include the booleans `collapsible` to choose if the section may be collapsed and `collapsed` to choose if it should be by default.

## Blog

Just like before, create a new folder called `blog`. Now you can create Markdown files with the name: `YEAR-MM-DD-title-goes-here.md`.

For an example of this, check out the [default Docusaurus structure](https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/blog).

Also check out [Docusaurus's official blog documentation](https://docusaurus.io/docs/blog) for more information on creating a blog.

## Custom Pages

Create a folder called `pages`. Any markdown, `mdx`, `js`, or `html` pages you place in this folder will be hosted on their respective paths on your website. You can add them to the navbar by editing your `moonwave.toml` navbar items section.

### Overriding the homepage
You can override the homepage by creating an `index.js` file in `pages` and writing your own React code. [Check out the default home page](https://github.com/evaera/moonwave/tree/master/cli/template/home) for a starting point.
