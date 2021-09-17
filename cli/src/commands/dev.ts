import { spawn } from "child_process"
import { Args } from "../argv"
import { prepareProject } from "../prepareProject"

export default async function devCommand(args: Args) {
  try {
    const tempDir = prepareProject(process.cwd(), args)

    const exitCode = await new Promise((resolve, reject) => {
      spawn("npm" + (process.platform === "win32" ? ".cmd" : ""), ["start"], {
        cwd: tempDir,
        stdio: "inherit",
      })
        .on("exit", (code) => resolve)
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
