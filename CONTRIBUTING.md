# Development Guide

Requirements:
- [Node.js 18+](https://nodejs.org/en/)
- [Rust 1.75.0 and Cargo](https://rustup.rs/)
- [Cargo Insta](https://insta.rs/docs/cli/)

It is best to install all dependencies immediately.
1. Clone this repository by running `git clone https://github.com/evaera/moonwave`
2. In `/cli`, run `npm i`, `npm run tsc`, and `npm link`
3. In `/docusaurus-plugin-moonwave`, run `npm i`
4. In `/extractor`, run `cargo install --path . --locked`
5. In `/website`, run `npm i`

## Extractor

The Extractor in Moonwave is a Rust program which parses documentation comments in Lua source code and outputs JSON to represent the Moonwave doc entries defined in those comments.

### Using in Development

Every time it is edited, rerun `cargo install --path . --locked` to update your local `moonwave-extractor` executable.

### Testing

It is necessary to run tests to ensure that the changes made to the Extractor are correct. If your contribution involves modifying the Extractor's output, then add new tests. In `/extractor`, run all the tests with `cargo test`, and if there are changes, run `cargo insta review`.

## Command Line Interface

The CLI is the user-facing interface of Moonwave, written in TypeScript, which handles project configuration and interacts with both the Extractor and Docusaurus Plugin to build and deploy Moonwave documentation websites.

### Using in Development

It is recommended to use `moonwave dev` with the environment variable `MOONWAVE_DEV=1`, which will utilise the local version of the Extractor and the Docusaurus Plugin.

### Live Reloading

In `/cli`, use `npm run dev` to automatically recompile the CLI every time a file is changed. This means that the next time a Moonwave command is executed, it will use the latest changes. Keep in mind that although `moonwave dev` supports hot reload for Luau files, it does not reload for the CLI, so it is necessary to rerun the command every time the Command Line Interface is updated.

## Docusaurus Plugin

Moonwave integrates with Docusaurus via a JavaScript Plugin which adds an API section to the website, generating a separate page for each Luau class. It uses both custom React components as well as Docusaurus components and features.

See https://docusaurus.io/

### Live Reloading

Using `moonwave dev` in development mode will set a local path in `package.json`, enabling hot reload for this package.

### Agnostisation

*(the act of making agnostic)*

It is a goal to make this package fully agnostic towards Docusaurus so that the components can be used in any React environment.

See https://github.com/evaera/moonwave/issues/47

## Official Moonwave website

Moonwave has its own [website](https://eryn.io/moonwave/) that documents usage of the tool. In `/website`, use `npm run start` to locally view the website. Hot reload is supported.