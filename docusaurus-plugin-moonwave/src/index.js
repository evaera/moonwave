const path = require("path")
const fs = require("fs")
const { promisify } = require("util")
const exec = promisify(require("child_process").exec)
const { generateRobloxTypes } = require("./generateRobloxTypes")

const capitalize = (text) => text[0].toUpperCase() + text.substring(1)

const breakCapitalWordsZeroWidth = (text) =>
  text.replace(/([A-Z])/g, "\u200B$1") // Adds a zero-width space before each capital letter. This way, the css word-break: break-word; rule can apply correctly

const getFunctionCallOperator = (type) =>
  type === "static" ? "." : type === "method" ? ":" : ""

const mapLinks = (nameSet, items) =>
  items.map((name) => {
    if (!nameSet.has(name)) {
      throw new Error(
        `Moonwave plugin: "${name}" listed in classOrder option does not exist`
      )
    }

    return {
      type: "link",
      href: `/api/${name}`,
      label: breakCapitalWordsZeroWidth(name),
    }
  })

function flattenTOC(toc) {
  const flat = []

  const iterate = (list) => {
    for (const item of list) {
      flat.push({
        ...item,
        children: undefined,
      })

      if (item.children) {
        iterate(item.children)
      }
    }
  }

  iterate(toc)

  return flat
}

function parseSimpleClassOrder(content, classOrder, nameSet) {
  const listedLinks = mapLinks(nameSet, classOrder)

  const unlistedLinks = content
    .map((luaClass) => luaClass.name)
    .filter((name) => !classOrder.includes(name))
    .sort((a, b) => a.localeCompare(b))
    .map((name) => ({
      type: "link",
      href: `/api/${name}`,
      label: breakCapitalWordsZeroWidth(name),
    }))

  return [...listedLinks, ...unlistedLinks]
}

function parseSectionalClassOrder(content, classOrder, nameSet) {
  const listedNames = classOrder.flatMap((section) => section.classes)

  const listedSidebar = []
  classOrder.forEach((element) => {
    if (element.section) {
      listedSidebar.push({
        type: "category",
        label: element.section,
        collapsible: true,
        collapsed: element.collapsed ?? true,
        items: mapLinks(nameSet, element.classes),
      })
    } else {
      listedSidebar.push(...mapLinks(nameSet, element.classes))
    }
  })

  const unlistedSidebar = content
    .map((luaClass) => luaClass.name)
    .filter((name) => !listedNames.includes(name))
    .sort((a, b) => a.localeCompare(b))
    .map((name) => ({
      type: "link",
      href: `/api/${name}`,
      label: breakCapitalWordsZeroWidth(name),
    }))

  return [...listedSidebar, ...unlistedSidebar]
}

function parseClassOrder(content, classOrder, nameSet) {
  if (classOrder.length === 0) {
    return [...nameSet].sort().map((name) => ({
      type: "link",
      href: `/api/${name}`,
      label: breakCapitalWordsZeroWidth(name),
    }))
  }

  if (typeof classOrder[0] === "string") {
    // Handles simple classOrder array assignment
    return parseSimpleClassOrder(content, classOrder, nameSet)
  } else {
    // Handles cases where classOrder is assigned via TOML tables
    return parseSectionalClassOrder(content, classOrder, nameSet)
  }
}

function parseApiCategories(luaClass, apiCategories) {
  const tocData = []

  // Loop through each member type of a LuaClass and check if it has any tagged children. If the tags match any tag provided by the user with the apiCategories config option, add it to it's own subheading in the table of contents
  const SECTIONS = ["types", "properties", "functions"]
  SECTIONS.forEach((section) => {
    const tagSet = new Set(
      luaClass[section]
        .filter((member) => !member.ignore)
        .filter((member) => member.tags)
        .flatMap((member) => member.tags)
    )

    const sectionChildren = []

    for (const category of apiCategories) {
      if (!tagSet.has(category)) {
        continue
      }

      const apiCategoryChild = []

      apiCategoryChild.push({
        value: capitalize(category),
        id: category,
        level: 3,
        children: luaClass[section]
          .filter((member) => !member.ignore)
          .filter((member) => member.tags && member.tags.includes(category))
          .map((member) => {
            return {
              value:
                member.name === "__call"
                  ? luaClass.name + "()"
                  : getFunctionCallOperator(member.function_type) + member.name,
              id: member.name,
              children: [],
              level: 4,
              private: member.private,
            }
          })
          .sort((childA, childB) => childA.value.localeCompare(childB.value)),
      })

      sectionChildren.push(...apiCategoryChild)
    }

    const baseCategories = luaClass[section]
      .filter((member) => !member.ignore)
      .filter(
        (member) =>
          !member.tags ||
          !member.tags.some((tag) => apiCategories.includes(tag))
      )
      .map((member) => ({
        value:
          member.name === "__call"
            ? luaClass.name + "()"
            : getFunctionCallOperator(member.function_type) + member.name,
        id: member.name,
        children: [],
        level: 3,
        private: member.private,
      }))
      .sort((childA, childB) => childA.value.localeCompare(childB.value))

    sectionChildren.push(...baseCategories)

    tocData.push({
      value: capitalize(section),
      id: section,
      children: sectionChildren,
      level: 2,
    })
  })

  return [...tocData]
}

async function generateTypeLinks(nameSet, luaClasses, baseUrl) {
  const classNames = {}

  nameSet.forEach((name) => (classNames[name] = `${baseUrl}api/${name}`))

  const classTypesNames = luaClasses
    .filter((luaClass) => luaClass.types.length > 0)
    .forEach((luaClass) =>
      luaClass.types.forEach(
        (type) =>
          (classNames[
            type.name
          ] = `${baseUrl}api/${luaClass.name}#${type.name}`)
      )
    )

  const robloxTypes = await generateRobloxTypes()

  const typeLinks = {
    ...robloxTypes, // The Roblox types go first, as they can be overwritten if the user has created their own classes and types with identical names
    ...classNames,
    ...classTypesNames,
  }

  return typeLinks
}

module.exports = (context, options) => ({
  name: "docusaurus-plugin-moonwave",

  getThemePath() {
    return path.resolve(__dirname, "./theme")
  },

  getPathsToWatch() {
    return options.code.map((filePath) => `${filePath}/**/*.{lua,luau}`)
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
          )}" --base "${basePath}"`,
          {
            maxBuffer: 10 * 1024 * 1024,
          }
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
    const filteredContent = content.filter((luaClass) => !luaClass.ignore)

    filteredContent.sort((a, b) => {
      if (a.name < b.name) {
        return -1
      } else if (a.name > b.name) {
        return 1
      } else {
        return 0
      }
    })

    const nameSet = new Set()
    filteredContent.forEach((luaClass) => nameSet.add(luaClass.name))

    const classOrder = options.classOrder

    if (options.autoSectionPath) {
      if (
        classOrder.length > 0 &&
        !classOrder.every((item) => typeof item === "object")
      ) {
        throw new Error(
          "When using autoSectionPath, classOrder cannot contain bare string keys." +
            "Use sectional style instead: https://eryn.io/moonwave/docs/Configuration#sections"
        )
      }

      const prefix = options.autoSectionPath

      for (const luaClass of filteredContent) {
        if (luaClass.source.path.startsWith(prefix)) {
          const classPath = luaClass.source.path.slice(prefix.length + 1)

          const nextDirMatch = classPath.match(/^(.+?)\//)

          if (nextDirMatch) {
            const nextDir = nextDirMatch[1]

            // convert kebab-case, camelCase, PascalCase to Title Case
            const title = nextDir
              .replace(/(?<!-)([A-Z])/g, " $1")
              .replace("-", " ")
              .split(/\s+/)
              .filter((str) => str.length > 0)
              .map(capitalize)
              .join(" ")

            const existingSection = classOrder.find(
              (section) => section.section === title
            )

            if (existingSection) {
              existingSection.classes.push(luaClass.name)
            } else {
              classOrder.push({
                section: title,
                classes: [luaClass.name],
              })
            }
          }
        }
      }
    }

    const allLuaClassNamesOrdered = parseClassOrder(
      filteredContent,
      classOrder,
      nameSet
    )

    const sidebarClassNames = await createData(
      "sidebar.json",
      JSON.stringify(allLuaClassNamesOrdered)
    )

    const apiCategories = options.apiCategories
    const baseUrl = context.baseUrl
    const pluginOptions = await createData(
      "options.json",
      JSON.stringify({
        sourceUrl: options.sourceUrl,
        baseUrl: baseUrl,
        classOrder: classOrder,
        apiCategories: apiCategories,
      })
    )

    const typeLinksData = await generateTypeLinks(
      nameSet,
      filteredContent,
      baseUrl
    )
    const typeLinks = await createData(
      "typeLinks.json",
      JSON.stringify(typeLinksData)
    )

    addRoute({
      path: baseUrl + "api/",
      exact: true,
      component: path.resolve(__dirname, "components/Redirect.js"),
      modules: {
        sidebarClassNames,
        pluginOptions,
      },
    })

    for (const luaClass of filteredContent) {
      const apiDataPath = await createData(
        `${luaClass.name}.json`,
        JSON.stringify(luaClass)
      )

      const tocDataOrdered = parseApiCategories(luaClass, apiCategories)

      const tocData = await createData(
        `${luaClass.name}-toc.json`,
        JSON.stringify(flattenTOC(tocDataOrdered))
      )

      console.log(`Adding path /api/${luaClass.name}`)

      addRoute({
        path: `${baseUrl}api/${luaClass.name}`,
        component: path.resolve(__dirname, "components/LuaClass.js"),
        modules: {
          luaClass: apiDataPath,
          sidebarClassNames,
          typeLinks,
          tocData,
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
