import { execSync } from "child_process"
import { readFileSync, writeFileSync } from "fs"

const version = process.argv[2]
if (!version || version.length < 3) {
  console.error("Specify version to release")
  process.exit(1)
}

console.log(`Publishing version ${version}...`)

function updatePackageJsonVersion(filePath) {
  const json = JSON.parse(readFileSync(filePath, { encoding: "utf-8" }))

  json.version = version

  writeFileSync(filePath, JSON.stringify(json, null, 2))
}

function updatePackageDependencyVersion(filePath, dependencyName, version) {
  const json = JSON.parse(readFileSync(filePath, { encoding: "utf-8" }))

  json.dependencies[dependencyName] = "^" + version

  writeFileSync(filePath, JSON.stringify(json, null, 2))
}

function replaceInFile(filePath, pattern, replacement) {
  writeFileSync(
    filePath,
    readFileSync(filePath, { encoding: "utf-8" })
      .replace(pattern, replacement)
  )
}

async function pollPluginPublished() {
  const response = execSync("npm view docusaurus-plugin-moonwave version");
  if (response.toString().trim() !== version.trim()) {
    await new Promise(resolve => setTimeout(resolve, 5000));
    await pollPluginPublished();
  }
}

const run = (cwd, command) =>
  execSync(command, {
    cwd,
    stdio: "inherit",
  })

if (execSync("git status --short --porcelain").toString().length > 0) {
  console.error("Please commit all changes before running this command")
  process.exit(1)
}

updatePackageJsonVersion("cli/package.json")
run("cli", "npm i --package-lock-only")

updatePackageDependencyVersion(
  "cli/template/root/package.json",
  "docusaurus-plugin-moonwave",
  version
)

updatePackageJsonVersion("docusaurus-plugin-moonwave/package.json")
run("docusaurus-plugin-moonwave", "npm i --package-lock-only")

replaceInFile(
  "extractor/Cargo.toml",
  /^(version = "\d+\.\d+\.\d+")$/m,
  `version = "${version}"`
)

run("docusaurus-plugin-moonwave", "npm publish")
await pollPluginPublished();

run("cli/template/root", "npm i --package-lock-only")
run("extractor", "cargo check")

const tag = `v${version}`
run(process.cwd(), "git add .")
run(process.cwd(), `git commit -m "Release version ${version}"`)
run(process.cwd(), `git tag ${tag}`)

run("cli", "npm publish")

run("extractor", "cargo publish")

run(process.cwd(), "git push")
run(process.cwd(), `git push origin ${tag}`)
