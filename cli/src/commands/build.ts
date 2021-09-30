import { spawn } from "child_process"
import path from "path"
import { Args } from "../argv.js"
import { getBinaryPath } from "../binary.js"
import { prepareProject } from "../prepareProject.js"

export default async function buildCommand(args: Args) {
  try {
    const { tempDir, projectDir } = prepareProject(process.cwd(), {
      codePaths: args.code,
      fresh: true,
      install: args.install,
      binaryPath: await getBinaryPath(),
    })

    const exitCode = await new Promise((resolve) => {
      spawn(
        "npm" + (process.platform === "win32" ? ".cmd" : ""),
        ["run", "build", "--", "--out-dir", path.join(projectDir, "build")],
        {
          cwd: tempDir,
          stdio: "inherit",
        }
      )
        .on("exit", resolve)
        .on("error", console.error)
    })

    if (exitCode !== 0) {
      throw new Error("Non-zero exit code")
    }

    console.log(
      "Moonwave: Website built into the `build` directory. Do not commit this folder: you should add it to your .gitignore file."
    )
  } catch (e) {
    console.error(typeof e === "object" && e !== null ? e.toString() : e)
    console.error(
      "Moonwave: It looks like something went wrong. Check the error output above."
    )
    process.exit(1)
  }
}
