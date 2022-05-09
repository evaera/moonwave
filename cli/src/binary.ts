import fs from "fs-extra"
import fetch from "node-fetch"
import os from "os"
import path, { dirname } from "path"
import unzipper from "unzipper"
import { fileURLToPath } from "url"

const __dirname = dirname(fileURLToPath(import.meta.url))

const packageConfig = fs.readJSONSync(path.join(__dirname, "../package.json"))
const version = packageConfig.version

const URL = `https://latest-github-release.eryn.io/evaera/moonwave/v${version}`

interface Asset {
  name: string
  content_type: string
  browser_download_url: string
}

interface Release {
  name: string
  created_at: string
  assets: Asset[]
}

export function promisifyStream(
  stream: fs.ReadStream | fs.WriteStream
): Promise<unknown> {
  return new Promise((resolve, reject) => {
    stream.on("close", resolve)
    stream.on("finish", resolve)
    stream.on("end", resolve)
    stream.on("error", reject)
  })
}

function getBinaryExtension(): string {
  if (os.platform() === "win32") {
    return ".exe"
  }

  return ""
}

function getBinaryName(): string {
  return `moonwave-extractor${getBinaryExtension()}`
}

function getBinaryZipPattern(): RegExp | undefined {
  switch (os.platform()) {
    case "win32":
      return /^moonwave-extractor(?:-|-.*-)win64.zip$/
    case "darwin":
      return /^moonwave-extractor(?:-|-.*-)macos.zip$/
    case "linux":
      return /^moonwave-extractor(?:-|-.*-)linux.zip$/
    default:
      return undefined
  }
}

async function downloadBinary(binaryPath: string) {
  const result = await fetch(URL)

  if (!result.ok) {
    console.error("Raw response from API:", await result.text())
    throw new Error(
      `${result.status} ${result.statusText} - Could not fetch Moonwave binary`
    )
  }

  const release = (await result.json()) as Release

  console.log(
    `Downloading moonwave-extractor from release ${release.name} released at ${release.created_at}`
  )

  const assets = release.assets
  const zipPattern = getBinaryZipPattern()

  if (!zipPattern) {
    throw new Error(`Your platform is unsupported: ${os.platform}`)
  }

  const targetAsset = assets.find((asset) => asset.name.match(zipPattern))

  if (!targetAsset) {
    throw new Error("Release does not contain a binary for this platform")
  }

  const download = await fetch(targetAsset.browser_download_url)

  if (!download.body) {
    throw new Error("Asset download body is null")
  }

  const writeStream = fs.createWriteStream(binaryPath)

  const unzip = download.body.pipe(
    unzipper.ParseOne(new RegExp(getBinaryName()), {})
  )

  const file = unzip.pipe(writeStream)

  await promisifyStream(file)

  if (file.bytesWritten === 0) {
    file.close()
    fs.removeSync(binaryPath)

    throw new Error("Write stream closed but zero bytes were written")
  } else {
    if (os.platform() !== "win32") {
      fs.chmod(binaryPath, 0o755)
    }
  }
}

export async function getBinaryPath() {
  if (process.env.MOONWAVE_DEV) {
    const extractorPath =
      process.env.MOONWAVE_EXTRACTOR_PATH || "moonwave-extractor"
    console.log(`Moonwave: Using development extractor path: ${extractorPath}`)
    return extractorPath
  }

  const binFolder = path.join(__dirname, "bin")
  const binaryPath = path.join(
    binFolder,
    `moonwave-extractor-${version}${getBinaryExtension()}`
  )

  if (fs.existsSync(binaryPath)) {
    return binaryPath
  }

  fs.ensureDirSync(binFolder)

  await downloadBinary(binaryPath)

  return binaryPath
}
