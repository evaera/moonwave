import { spawn } from "child_process"
import { Args } from "../argv"
import { prepareProject } from "../prepareProject"

export default async function buildCommand(args: Args) {
  try {
    const { tempDir } = prepareProject(process.cwd(), {
      ...args,
      fresh: true,
    })

    const exitCode = await new Promise((resolve) => {
      spawn(
        "npm" + (process.platform === "win32" ? ".cmd" : ""),
        ["run", "build", "--", "--out-dir", "../build"],
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
  } catch (e) {
    console.error(typeof e === "object" && e !== null ? e.toString() : e)
    console.error(
      "Moonwave: It looks like something went wrong. Check the error output above."
    )
    process.exit(1)
  }
}
