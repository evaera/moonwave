const path = require("path")
const fs = require("fs")
const { promisify } = require("util")
const exec = promisify(require("child_process").exec)

module.exports = (context, options) => ({
  name: "docusaurus-plugin-moonwave",

  getThemePath() {
    return path.resolve(__dirname, "./theme")
  },

  getPathsToWatch() {
    return options.code.map((filePath) => `${filePath}/**/*.lua`)
  },

  async loadContent() {
    const basePath = options.projectDir || path.resolve(process.cwd(), "..")

    const binaryPath = options.binaryPath ?? "moonwave-extractor"

    const api = await Promise.all(
      options.code.map((root) =>
        exec(
          `"${binaryPath}" extract "${root.replace(
            /\\/g,
            "/"
          )}" --base "${basePath}"`
        )
          .then(({ stdout, stderr }) => {
            if (stderr.length > 0) {
              return Promise.reject(stderr)
            }

            return stdout
          })
          .then((raw) => JSON.parse(raw))
      )
    )

    return api.flat()
  },

  async contentLoaded({ content, actions: { addRoute, createData } }) {
    content.sort((a, b) => {
      if (a.name < b.name) {
        return -1
      } else if (a.name > b.name) {
        return 1
      } else {
        return 0
      }
    })

    let allLuaClassNamesOrdered = []

    const nameSet = new Set()
    content.forEach((luaClass) => nameSet.add(luaClass.name))

    const classOrder = options.classOrder

    // Handles simple classOrder array assignment
    if (typeof classOrder[0] === "string") {
      classOrder.forEach((name) => {
        if (!nameSet.has(name)) {
          throw new Error(
            `Moonwave plugin: "${name}" listed in classOrder option does not exist`
          )
        }
      })

      const sideBarFormatClassOrder = classOrder.map((name) => ({
        type: "link",
        href: `/api/${name}`,
        label: name,
      }))

      const sideBarFormatClassOrderUnlistedNames = content
        .map((luaClass) => luaClass.name)
        .filter((name) => !classOrder.includes(name))
        .sort((a, b) => a.localeCompare(b))
        .map((name) => ({
          type: "link",
          href: `/api/${name}`,
          label: name,
        }))

      allLuaClassNamesOrdered = [
        ...sideBarFormatClassOrder,
        ...sideBarFormatClassOrderUnlistedNames,
      ]
    }
    // Handles cases where classOrder is assigned via TOML tables
    else {
      if (classOrder.length >= 1) {
        const listedNames = classOrder.flatMap((section) => section.classes)

        let sideBarFormatClassOrder = []
        classOrder.forEach((element) => {
          if (element.section) {
            const items = element.classes.map((name) => {
              if (!nameSet.has(name)) {
                throw new Error(
                  `Moonwave plugin: "${name}" listed in classOrder option does not exist`
                )
              }

              return {
                type: "link",
                href: `/api/${name}`,
                label: name,
              }
            })

            sideBarFormatClassOrder.push({
              type: "category",
              label: element.section,
              collapsible: true,
              collapsed: true,
              items: items,
            })
          } else {
            element.classes.forEach((name) => {
              if (!nameSet.has(name)) {
                throw new Error(
                  `Moonwave plugin: "${name}" listed in classOrder option does not exist`
                )
              }

              sideBarFormatClassOrder.push({
                type: "link",
                href: `/api/${name}`,
                label: name,
              })
            })
          }
        })

        const unlistedNames = content
          .map((luaClass) => luaClass.name)
          .filter((name) => !listedNames.includes(name))
          .sort((a, b) => a.localeCompare(b))
          .map((name) => ({
            type: "link",
            href: `/api/${name}`,
            label: name,
          }))

        allLuaClassNamesOrdered = [...sideBarFormatClassOrder, ...unlistedNames]
      } else {
        allLuaClassNamesOrdered = [...nameSet].sort().map((name) => ({
          type: "link",
          href: `/api/${name}`,
          label: name,
        }))
      }
    }

    const allLuaClassNames = await createData(
      "sidebar.json",
      JSON.stringify(allLuaClassNamesOrdered)
    )

    const baseUrl = context.baseUrl
    const pluginOptions = await createData(
      "options.json",
      JSON.stringify({
        sourceUrl: options.sourceUrl,
        baseUrl: baseUrl,
        classOrder: classOrder,
      })
    )

    addRoute({
      path: baseUrl + "api/",
      exact: true,
      component: path.resolve(__dirname, "components/Redirect.js"),
      modules: {
        allLuaClassNames,
        pluginOptions,
      },
    })

    for (const luaClass of content) {
      const apiDataPath = await createData(
        `${luaClass.name}.json`,
        JSON.stringify(luaClass)
      )

      console.log(`Adding path /api/${luaClass.name}`)

      addRoute({
        path: `${baseUrl}api/${luaClass.name}`,
        component: path.resolve(__dirname, "components/LuaClass.js"),
        modules: {
          luaClass: apiDataPath,
          allLuaClassNames,
          options: pluginOptions,
        },
        exact: true,
      })
    }
  },
})

module.exports.validateOptions = ({ options }) => {
  if (!options.code) {
    throw new Error(
      "Moonwave plugin: expected option `code` to point to your source code."
    )
  }

  if (options.sourceUrl && typeof options.sourceUrl !== "string") {
    throw new Error(
      "Moonwave plugin: expected option `sourceUrl` to be a string."
    )
  }

  if (options.projectDir && typeof options.projectDir !== "string") {
    throw new Error(
      "Moonwave plugin: expected option `projectDir` to be a string."
    )
  }

  if (!Array.isArray(options.code)) {
    options.code = [options.code]
  }

  for (const [index, codePath] of options.code.entries()) {
    if (typeof codePath !== "string") {
      throw new Error(
        `Moonwave plugin: code should be an array of strings, found a: ${typeof codePath}`
      )
    }

    const resolvedPath = path.resolve(process.cwd(), codePath)

    if (!fs.existsSync(resolvedPath)) {
      throw new Error(
        `Moonwave plugin: code path ${resolvedPath} does not actually exist.`
      )
    }

    options.code[index] = resolvedPath
  }

  return options
}
