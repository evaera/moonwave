import { spawn } from "child_process"
import githubPages from "gh-pages"
import path from "path"
import { Args } from "../argv.js"
import { getBinaryPath } from "../binary.js"
import { prepareProject } from "../prepareProject.js"

function publish(buildDir: string): Promise<void> {
  return new Promise((resolve, reject) => {
    githubPages.publish(
      buildDir,
      { dotfiles: true, message: "Built and published by Moonwave" },
      (err) => (err ? reject(err) : resolve())
    )
  })
}

export default async function buildCommand(args: Args) {
  try {
    const { tempDir, projectDir } = prepareProject(process.cwd(), {
      codePaths: args.code,
      fresh: true,
      install: args.install,
      binaryPath: await getBinaryPath(),
    })
    const buildDirName = args["out-dir"] || "build"
    const buildDir = path.join(projectDir, buildDirName)

    const exitCode = await new Promise((resolve) => {
      spawn(
        "npm" + (process.platform === "win32" ? ".cmd" : ""),
        ["run", "build", "--", "--out-dir", buildDir],
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
      `Moonwave: Website built into the \`${buildDirName}\` directory. Do not commit this folder: you should add it to your .gitignore file.`
    )

    if (args.publish) {
      console.log("Moonwave: Publishing build to gh-pages branch...")
      await publish(buildDir)
      console.log("Moonwave: Published! Your website should now be live.")
    }
  } catch (e) {
    console.error(typeof e === "object" && e !== null ? e.toString() : e)
    console.error(
      "Moonwave: It looks like something went wrong. Check the error output above."
    )
    process.exit(1)
  }
}
