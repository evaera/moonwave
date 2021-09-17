import { spawn } from "child_process"
import chokidar from "chokidar"
import fs from "fs-extra"
import path from "path"
import { Args } from "../argv"
import { prepareProject } from "../prepareProject"

export default async function devCommand(args: Args) {
  try {
    const prepare = () => prepareProject(process.cwd(), args)

    let { tempDir, watchPaths, projectDir } = prepare()

    const watcher = chokidar
      .watch(projectDir, {
        ignoreInitial: true,
      })
      .on("all", (event, changedPath) => {
        if (
          watchPaths.filter((watchPath) => changedPath.includes(watchPath))
            .length > 0
        ) {
          if (event === "unlink" || event == "unlinkDir") {
            const relativePath = path.relative(changedPath, projectDir)
            const targetPath = path.join(tempDir, relativePath)

            fs.removeSync(targetPath)
          }

          let { watchPaths: newWatchPaths } = prepareProject(process.cwd(), {
            ...args,
            fresh: false,
          })

          for (const watchPath of watchPaths) {
            if (!newWatchPaths.includes(watchPath)) {
              watcher.unwatch(watchPath)
            }
          }

          for (const watchPath of newWatchPaths) {
            if (!watchPaths.includes(watchPath)) {
              watcher.add(watchPath)
            }
          }

          watchPaths = newWatchPaths
        }
      })

    const exitCode = await new Promise((resolve) => {
      spawn(
        "npm" + (process.platform === "win32" ? ".cmd" : ""),
        ["run", "start"],
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
