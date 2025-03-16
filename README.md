<div align="center">
    <img src="brand/moonwave.svg" alt="Moonwave" height="139" />
</div>

# Moonwave

Moonwave is a command line tool for generating documentation from comments in Lua source code.

This repository contains three projects:
- The Moonwave Extractor, a Rust program that parses Lua files and extracts the docs as JSON
- A plugin for [Docusaurus](https://docusaurus.io/), written in React, which displays the JSON docs visually
- The Moonwave CLI, which allows you to use both the extractor and the Docusaurus plugin without needing to know anything about how they work.

## Key features

- Easy to use: You can generate a website with little-to-no configuration and just a few comments in your Lua code.
- Extensive validation system which can catch mistakes when something doesn't make sense.
- Generates JSON from your doc comments, which can be consumed by many different tools.
- Simple doc comment format that is easy to read directly when editing your code.

## Demo

Check out the [roblox-lua-promise](https://eryn.io/roblox-lua-promise/api/Promise) docs. This website is completely generated by Moonwave!

## Name

Lua is the moon. The moon is your code. The moon influences the waves in the ocean, just like your code influences its documentation. So.. moonwave!

## Docs

[Check out the Documentation website](https://eryn.io/moonwave/)

## Building in Development and Contributing

Requirements:
- [Node.js 18+](https://nodejs.org/en/)
- [Rust 1.75.0 and Cargo](https://rustup.rs/)

It is best to install all dependencies immediately.
1. Clone this repository by running `git clone https://github.com/evaera/moonwave`
2. In `/cli`, run `npm i`, `npm run tsc`, and `npm link`
3. In `/docusaurus-plugin-moonwave`, run `npm i`
4. In `/extractor`, run `cargo install --path . --locked`
5. In `/website`, run `npm `i

**Extractor**

The extractor is written in Rust and outputs JSON code which describes all the Luau classes, functions, properties, et cetera. Every time it is edited, run `cargo install --path . --locked` to update your local `moonwave-extractor` executable. If your contribution involves changing the extractor's output, add a test to ensure it functions as expected. It is recommended to install [Cargo Insta](https://insta.rs/docs/cli/) for testing. Run tests with `cargo test`, and if there are changes, review them with `cargo insta review`.

**Command line interface**

Moonwave comes with a CLI written in TypeScript found in `/cli`. It has two commands: [`moonwave dev`](https://eryn.io/moonwave/docs/intro#use-moonwave-with-your-project) and [`moonwave build`](https://eryn.io/moonwave/docs/Publishing). It is recommended to use `moonwave dev` with the environment variable `MOONWAVE_DEV=1`, which requires manually installing the extractor.

In `/cli`, you can run `npm run dev` to automatically recompile the CLI every time a file is changed. This means that the next time you run a command, it will use the latest changes. Keep in mind that although `moonwave dev` supports hot reload for Luau files, it does not reload for the CLI, so it is necessary to rerun the command every time the command line interface is updated.

**Docusaurus plugin**

Moonwave integrates with Docusaurus via a JavaScript plugin. Using `moonwave dev` in development mode will hot reload for this package.

**Official Moonwave website**

Moonwave has its own [website](https://eryn.io/moonwave/) that documents usage of the tool. The code is found in `/website`. Use `npm run start` to locally view the website. The website pages are stored in `/website/docs`. Hot reload is supported.

**Exclamation mark**

Make sure there are no exclamation marks (!) in the path to your local Moonwave repository. This will cause an error where Webpack says that it can not find the part of the path before the exclamation mark.

Further reading: https://github.com/webpack/webpack/issues/5320

## License

Moonwave is available under the terms of the Mozilla Public License Version 2.0. Terms and conditions are available in [LICENSE.txt](LICENSE.txt) or at <https://www.mozilla.org/en-US/MPL/2.0/>.
