import yargs from "yargs"
import buildCommand from "./commands/build"
import devCommand from "./commands/dev"
const version = require("../package.json").version as string

export interface Args {
  fresh: boolean
  install: boolean
  code: string[]
}

const argv = yargs
  .scriptName("moonwave")
  .usage("Usage: moonwave [options]")

  .alias("v", "version")
  .version(version)
  .describe("version", "show version information")

  .alias("h", "help")
  .help("help")
  .describe("help", "show help")
  .showHelpOnFail(true)

  .command("build", "build the docs website", () => {}, buildCommand)
  .command(
    "dev",
    "run in development live-reload mode",
    (yargs) => {
      yargs
        .boolean("fresh")
        .describe("fresh", "deletes build cache before building")
        .alias("f", "fresh")
    },
    devCommand
  )

  .array("code")
  .describe("code", "the path to your Lua code. e.g. 'src'")
  .default("code", ["lib", "src"])

  .boolean("install")
  .describe("install", "re-install npm dependencies")
  .alias("i", "install")

  .strictCommands()
  .demandCommand()
  .parse()

export default argv
