import { spawn } from "child_process"
import chokidar from "chokidar"
import fs from "fs-extra"
import path from "path"
import { Args } from "../argv.js"
import { getBinaryPath } from "../binary.js"
import { prepareProject } from "../prepareProject.js"

export default async function devCommand(args: Args) {
  try {
    const binaryPath = await getBinaryPath()

    const { tempDir, watchPaths, projectDir } = prepareProject(process.cwd(), {
      codePaths: args.code,
      fresh: args.fresh,
      install: args.install,
      binaryPath,
    })

    console.error(
      `Moonwave: Temporary build directory is located at ${tempDir}`
    )

    const onAll = (event: string, changedPath: string) => {
      if (
        watchPaths.some((watchPath) => {
          if (path.normalize(watchPath) === path.normalize(changedPath)) {
            return true
          }

          const relative = path.relative(watchPath, changedPath)
          return (
            relative &&
            !relative.startsWith("..") &&
            !path.isAbsolute(relative)
          )
        })
      ) {
        if (event === "unlink" || event == "unlinkDir") {
          const relativePath = path.relative(projectDir, changedPath)
          const targetPath = path.join(tempDir, relativePath)

          fs.removeSync(targetPath)
        }

        prepareProject(process.cwd(), {
          codePaths: args.code,
          fresh: false,
          skipRootCopy: true,
          binaryPath,
        })
      }
    }
    
    watchPaths.forEach((item) => {
      chokidar
        .watch(item, {
          ignoreInitial: true
        })
        .on("all", onAll)
    })
    
    chokidar
      .watch(projectDir, {
        ignoreInitial: true,
        depth: 0
      })
      .on("all", onAll)

    const exitCode = await new Promise((resolve) => {
      spawn(
        "npm" + (process.platform === "win32" ? ".cmd" : ""),
        ["run", "start"],
        {
          cwd: tempDir,
          shell: true,
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
