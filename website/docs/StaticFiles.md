---
sidebar_position: 5
---

# Static Files

You can further customize Moonwave by creating a `.moonwave` folder in your root directory.

## Static Files (like images)

You can host static files by creating a `static` folder inside your `.moonwave` folder. Anything in this folder will be available on your website from the root path (meaning if you put `image.png` in `static`, it will be on your website at `website.com/image.png`)

This is a great place to put images and other media that you reference in your documentation.

## Custom CSS

You can create a file at `.moonwave/custom.css`. Here's an example of what you can customize:

```css
/**
 * Any CSS included here will be global. The classic template
 * bundles Infima by default. Infima is a CSS framework designed to
 * work well for content-centric websites.
 */

/* You can override the default Infima variables here. */
:root {
  --ifm-color-primary: #25c2a0;
  --ifm-color-primary-dark: rgb(33, 175, 144);
  --ifm-color-primary-darker: rgb(31, 165, 136);
  --ifm-color-primary-darkest: rgb(26, 136, 112);
  --ifm-color-primary-light: rgb(70, 203, 174);
  --ifm-color-primary-lighter: rgb(102, 212, 189);
  --ifm-color-primary-lightest: rgb(146, 224, 208);
  --ifm-code-font-size: 95%;
}
```

## Custom Docs Sidebar

You can create a custom `.moonwave/sidebars.js` file to set up a custom ordering for your `docs` sidebar. This allows you to specify ordering, sections, and if certain files are excluded.

Example `sidebars.js` file:

```js
module.exports = {
  mySidebar: [
    {
      type: "doc",
      id: "getting-started",
      label: "Getting Started",
    },
    {
      type: "category",
      label: "Moonwave",
      items: ["moonwave-basics, moonwave-advances"],
    },
    {
      type: "category",
      label: "Other Resources",
      items: [
        "nested-folder/extra-resources",
        "another-folder/even-more-resources",
      ],
    },
  ],
}
```

Additional information on customizing your `docs` sidebar can be found at [the Docusaurus Documentation](https://docusaurus.io/docs/sidebar).
