---
sidebar_position: 6
---

# Publishing your website

## It's easy


If you are using git and Github pages, you can publish your doc website with one command:

- `moonwave build --publish`

Otherwise, you can run `moonwave build`, and your built website will be in a folder called `build` in your project directory.

:::caution
Remember to add the `build` folder to your `.gitignore`!
:::

## If your master branch is not `master`
Moonwave needs to know your master branch to make the "Edit this page" links work. We don't detect the correct master branch right now, so by default we assume it's `master`. If this is wrong, you should configure it in [`moonwave.toml`](Configuration):

```toml
gitSourceBranch = "main"
```

## Using a custom domain / Not Using Github Pages

In the [Configuration](Configuration), you'll note two options:

```toml
[docusaurus]
url = "https://organizationName.github.io"
baseUrl = "/projectName"
```

These are the defaults. If you're hosting your website on a custom domain, or you aren't using Github Pages, these defaults will be incorrect.

In that case, you **must** create a `moonwave.toml` file and configure these options correctly.

Even if you are using Github Pages, and:
- you're using a custom domain, or
- you're hosting the website on the root path (`organizationName.github.io`, instead of `organizationName.github.io/projectName`)
  
...you need to configure both of these options, **and set `baseUrl = "/"`**

## Github Pages Custom Domain CNAME file

Github Pages requires you to create a CNAME file in your website to host on a custom domain. You should create this file at `yourProjectRoot/.moonwave/static/CNAME` (create the folders if they don't exist) and put your domain in the file. 