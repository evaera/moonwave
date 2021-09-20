import { execSync } from "child_process"
import fs from "fs-extra"
import parseGitConfig from "parse-git-config"
import path from "path"
import toml from "toml"
import getDocusaurusConfig, {
  GenerateConfigParams,
} from "./getDocusaurusConfig"

const TEMPLATE_PATH = path.join(__dirname, "../template")
const ROOT_PATH = path.join(TEMPLATE_PATH, "root")

const INDEX_EXTS = ["html", "js", "mdx", "md"]
const COPY_FOLDERS = ["blog", "docs", "pages"] as const

const NO_README_TEXT = (title: string) => `# ${title}
This project doesn't have a README.
If it had a README.md in its root directory, you would be reading that right now.`

const NO_GIT_REPO_TEXT = `# This project has no configured title
The site title is usually pulled from your Git repo, but no git repo could be detected.
Either set this project up as a Git repo, or configure the website title in moonwave.toml`

export type FoldersEnabled = { [index in typeof COPY_FOLDERS[number]]: boolean }

export type Config = Partial<{
  // Moonwave
  gitRepoUrl: string
  gitSourceBranch: string
  title: string
  changelog: boolean

  // Docusaurus
  docusaurus: Partial<{
    title: string
    tagline: string
    url: string
    baseUrl: string
    onBrokenLinks: string
    onBrokenMarkdownLinks: string
    favicon: string
    organizationName: string
    projectName: string
  }>

  navbar: Partial<{
    hideableSidebar: boolean
    title: string
    logo: { alt: string; src: string }
    items: { to: string; label: string; position: "left" | "right" }[]
  }>

  home: Partial<{
    enabled: boolean

    features: {
      title: string
      description: string
      image: string
    }[]
  }>

  footer: Partial<{
    style: string
    links: {
      title: string
      copyright: string
      items: { label: string; to: string }[]
    }[]
  }>
}>

function getGitRepoUrl(): string | undefined {
  const gitConfig = parseGitConfig.sync()

  if (gitConfig) {
    return gitConfig['remote "origin"']?.url?.replace(/\.git$/, "")
  }
}

function readConfig(projectDir: string): Config {
  const configPath = path.join(projectDir, "moonwave")

  if (fs.existsSync(configPath + ".toml")) {
    return toml.parse(
      fs.readFileSync(configPath + ".toml", { encoding: "utf-8" })
    )
  } else if (fs.existsSync(configPath + ".json")) {
    return fs.readJSONSync(configPath + ".json")
  }

  return {}
}

function getConfig(projectDir: string): Config {
  const gitRepoUrl = getGitRepoUrl()

  const [, repoAuthor, repoName] =
    gitRepoUrl?.match(/^https?:\/\/.+\/(.+)\/(.+)$/) || []

  const config = readConfig(projectDir)

  // Note: Only copying values from other places in the config should go here.
  // Default values for docusaurus.config.js belong in getDocusaurusConfig
  return {
    title: repoName,
    gitRepoUrl,
    changelog: true,
    ...config,

    docusaurus: {
      projectName: repoName ?? undefined,
      organizationName: repoAuthor ?? undefined,
      title: config.title ?? repoName ?? "You need to configure your title",
      baseUrl: repoName ? `/${repoName}/` : "/",
      ...config.docusaurus,
    },

    navbar: {
      title: config.title ?? config.docusaurus?.title ?? repoName ?? "No Title",
      ...config.navbar,
    },
  }
}

function makeHomePage(projectDir: string, tempDir: string, config: Config) {
  if (
    INDEX_EXTS.filter((ext) =>
      fs.existsSync(path.join(projectDir, "pages", "index." + ext))
    ).length === 0
  ) {
    fs.ensureDirSync(path.join(tempDir, "pages"))

    INDEX_EXTS.forEach((ext) =>
      fs.removeSync(path.join(tempDir, "pages", "index." + ext))
    )

    if (config.home?.enabled) {
      const features = config.home?.features?.map((feature) => {
        if (feature.image && feature.image.startsWith("/")) {
          feature.image = config.docusaurus?.baseUrl + feature.image

          return feature
        }
      })

      const indexSource = fs
        .readFileSync(path.join(TEMPLATE_PATH, "home", "index.js"), {
          encoding: "utf-8",
        })
        .replace("/***features***/", JSON.stringify(features ?? null))

      fs.writeFileSync(path.join(tempDir, "pages", "index.js"), indexSource)

      fs.copyFileSync(
        path.join(TEMPLATE_PATH, "home", "index.module.css"),
        path.join(tempDir, "pages", "index.module.css")
      )
    } else {
      const indexPath = path.join(tempDir, "pages", "index.md")
      const readmePath = path.join(projectDir, "README.md")
      if (fs.existsSync(readmePath)) {
        fs.copyFileSync(readmePath, indexPath)
      } else {
        const placeholderHomeText = config.title
          ? NO_README_TEXT(config.title)
          : NO_GIT_REPO_TEXT

        fs.writeFileSync(indexPath, placeholderHomeText)
      }
    }
  }
}

function copyChangelog(
  projectDir: string,
  tempDir: string,
  config: Config
): boolean {
  const changelogPath = path.join(projectDir, "CHANGELOG.md")
  const targetPath = path.join(tempDir, "pages", "CHANGELOG.md")

  if (config.changelog && fs.existsSync(changelogPath)) {
    fs.ensureDirSync(path.join(tempDir, "pages"))

    fs.copyFileSync(changelogPath, targetPath)

    return true
  } else if (fs.existsSync(targetPath)) {
    fs.removeSync(targetPath)
  }

  return false
}

function copyMoonwaveFolder(
  projectDir: string,
  tempDir: string
): { customCssExists: boolean } {
  const staticDir = path.join(projectDir, ".moonwave", "static")
  if (fs.existsSync(staticDir)) {
    fs.copySync(staticDir, path.join(tempDir, "static"))
  }

  const customCssPath = path.join(projectDir, ".moonwave", "custom.css")
  if (fs.existsSync(customCssPath)) {
    fs.copySync(customCssPath, path.join(tempDir, "src", "css", "custom.css"))

    return { customCssExists: true }
  }

  return { customCssExists: false }
}

function writeDocusaurusConfig(tempDir: string, params: GenerateConfigParams) {
  const docusaurusConfigPath = path.join(tempDir, "./docusaurus.config.js")
  const newDocusaurusConfig =
    "module.exports = " + JSON.stringify(getDocusaurusConfig(params), null, 2)

  if (
    fs.existsSync(docusaurusConfigPath) &&
    fs.readFileSync(docusaurusConfigPath, { encoding: "utf-8" }) ===
      newDocusaurusConfig
  ) {
    return false
  } else {
    fs.writeFileSync(docusaurusConfigPath, newDocusaurusConfig)
    return true
  }
}

function copyContentFolders(
  projectDir: string,
  tempDir: string
): FoldersEnabled {
  return Object.fromEntries(
    COPY_FOLDERS.map((folder) => {
      const folderPath = path.join(projectDir, folder)
      const targetPath = path.join(tempDir, folder)

      if (fs.existsSync(folderPath)) {
        fs.copySync(folderPath, targetPath)
        return true
      } else {
        return false
      }
    }).map((wasFound, index) => [COPY_FOLDERS[index], wasFound])
  ) as FoldersEnabled
}

export interface PreparedProject {
  tempDir: string
  projectDir: string

  watchPaths: string[]

  docusaurusConfigModified: boolean
}

export interface PrepareProjectOptions {
  codePaths: string[]
  skipRootCopy?: boolean
  fresh?: boolean
}

export function prepareProject(
  projectDir: string,
  options: PrepareProjectOptions
): PreparedProject {
  const config = getConfig(projectDir)

  const tempDir = path.join(projectDir, "./.moonwave-temp")

  if (options.fresh && fs.existsSync(tempDir)) {
    for (const file of fs
      .readdirSync(tempDir)
      .filter((name) => name !== "node_modules")) {
      fs.removeSync(path.join(tempDir, file))
    }
  }

  if (!options.skipRootCopy) {
    fs.copySync(ROOT_PATH, tempDir)
  }

  // Create home page or copy readme
  makeHomePage(projectDir, tempDir, config)
  // Copy CHANGELOG.md if it exists
  const changelogExists = copyChangelog(projectDir, tempDir, config)

  const foundFolders = copyContentFolders(projectDir, tempDir)

  const { customCssExists } = copyMoonwaveFolder(projectDir, tempDir)

  const docusaurusConfigModified = writeDocusaurusConfig(tempDir, {
    config,
    enablePlugins: foundFolders,
    customCssExists,
    codePaths: options.codePaths,
    changelogExists,
  })

  // TODO: Hash package.json / lockfile and additionally reinstall when changed
  if (!fs.existsSync(path.join(tempDir, "./node_modules"))) {
    console.log("Installing dependencies (this might take awhile)...")

    execSync("npm i", {
      cwd: tempDir,
      stdio: "inherit",
    })
  }

  return {
    docusaurusConfigModified,
    tempDir,
    projectDir,
    watchPaths: [
      path.join(projectDir, "moonwave.toml"),
      path.join(projectDir, "moonwave.json"),
      path.join(projectDir, "CHANGELOG.md"),
      path.join(projectDir, ".moonwave/"),
      ...Object.entries(foundFolders)
        // .filter(([_folder, wasFound]) => wasFound)
        .map(([folder]) => folder)
        .map((folder) => path.join(projectDir, folder)),
    ],
  }
}
