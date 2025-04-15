# Development Guide

Requirements:
- [Node.js 18+](https://nodejs.org/en/)
- [Rust 1.75.0 and Cargo](https://rustup.rs/)

It is best to install all dependencies immediately.
1. Clone this repository by running `git clone https://github.com/evaera/moonwave`
2. In `/cli`, run `npm i`, `npm run tsc`, and `npm link`
3. In `/docusaurus-plugin-moonwave`, run `npm i`
4. In `/extractor`, run `cargo install --path . --locked`
5. In `/website`, run `npm i`

**Extractor**

The extractor is written in Rust and outputs JSON code which describes all the Luau classes, functions, properties, et cetera. Every time it is edited, run `cargo install --path . --locked` to update your local `moonwave-extractor` executable. If your contribution involves changing the extractor's output, add a test to ensure it functions as expected. It is recommended to install [Cargo Insta](https://insta.rs/docs/cli/) for testing. Run tests with `cargo test`, and if there are changes, review them with `cargo insta review`.

**Command line interface**

Moonwave comes with a CLI written in TypeScript found in `/cli`. It has two commands: [`moonwave dev`](https://eryn.io/moonwave/docs/intro#use-moonwave-with-your-project) and [`moonwave build`](https://eryn.io/moonwave/docs/Publishing). It is recommended to use `moonwave dev` with the environment variable `MOONWAVE_DEV=1`.

In `/cli`, use `npm run dev` to automatically recompile the CLI every time a file is changed. This means that the next time a Moonwave command is executed, it will use the latest changes. Keep in mind that although `moonwave dev` supports hot reload for Luau files, it does not reload for the CLI, so it is necessary to rerun the command every time the command line interface is updated.

**Docusaurus plugin**

Moonwave integrates with Docusaurus via a JavaScript plugin. Using `moonwave dev` in development mode will hot reload for this package.

**Official Moonwave website**

Moonwave has its own [website](https://eryn.io/moonwave/) that documents usage of the tool. The code is found in `/website`. Use `npm run start` to locally view the website. Hot reload is supported.