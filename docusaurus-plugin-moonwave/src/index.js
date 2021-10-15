const path = require("path")
const fs = require("fs")
const { promisify } = require("util")
const { create } = require("domain")
const exec = promisify(require("child_process").exec)

const SECTIONS = ["types", "properties", "functions"]

const capitalize = (text) => text[0].toUpperCase() + text.substring(1)

const breakCapitalWordsZeroWidth = (text) =>
  text.replace(/([A-Z])/g, "\u200B$1") // Adds a zero-width space before each capital letter. This way, the css word-break: break-word; rule can apply correctly

const addFunctionTypeSymbol = (text, type) =>
  (type === "static" ? "." : type === "method" ? ":" : "") + text

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

function parseSimpleApiCategories(luaClass, apiCategories) {
  const tagSet = new Set(
    luaClass.functions.filter((func) => func.tags).flatMap((func) => func.tags)
  )

  // This LuaClass has no special tags, so it can be skipped
  // if (tagSet.length === 0 || !apiCategories.some((tag) => tagSet.has(tag))) {
  //   return parseBaseApiCategories(luaClass)
  // } else {
  let listedCategories = []
  apiCategories.forEach((category) => {
    if (tagSet.has(category)) {
      listedCategories.push({
        value: capitalize(category),
        id: category,
        children: luaClass.functions
          .filter((func) => func.tags && func.tags.includes(category))
          .map((member) => {
            return {
              value: addFunctionTypeSymbol(member.name, member.function_type),
              id: member.name,
              children: [],
            }
          })
          .sort((childA, childB) => childA.id.localeCompare(childB.id)),
      })
    }
  })

  const baseCategories = SECTIONS.map((section) => ({
    value: capitalize(section),
    id: section,
    children: luaClass[section]
      .filter(
        (member) =>
          !member.tags ||
          member.tags.some((tag) => !apiCategories.includes(tag))
      )
      .map((member) => ({
        value: addFunctionTypeSymbol(member.name, member.function_type),
        id: member.name,
        children: [],
      }))
      .sort((childA, childB) => childA.id.localeCompare(childB.id)),
  }))

  return [...listedCategories, ...baseCategories]
  // }
}

function parseSectionalApiCategories(luaClass, apiCategories) {
  const functionSet = new Set(luaClass.functions.flatMap((func) => func.name))

  const flatApiCategories = apiCategories
    .filter((category) => category.class === luaClass.name)
    .flatMap((category) => category.members)

  flatApiCategories.forEach((member) => {
    if (!functionSet.has(member)) {
      throw new Error(
        `Moonwave plugin: "${member}" listed in apiCategories "${luaClass.name}" option does not exist`
      )
    }
  })

  const mappedCategories = apiCategories
    .filter((category) => category.class === luaClass.name)
    .map((section) => {
      if (section.class === luaClass.name) {
        return {
          category: section.category,
          members: section.members,
        }
      }
    })

  const listedCategories = mappedCategories.map((section) => ({
    value: capitalize(section.category),
    id: section.category,
    children: section.members
      .map((member) => ({
        value: addFunctionTypeSymbol(
          member,
          luaClass["functions"].find((element) => element.name === member)
            .function_type
        ),
        id: member,
        children: [],
      }))
      .sort((childA, childB) => childA.id.localeCompare(childB.id)),
  }))

  const baseCategories = SECTIONS.map((section) => ({
    value: capitalize(section),
    id: section,
    children: luaClass[section]
      .filter((member) => !flatApiCategories.includes(member.name))
      .map((member) => ({
        value: addFunctionTypeSymbol(member.name, member.function_type),
        id: member.name,
        children: [],
      }))
      .sort((childA, childB) => childA.id.localeCompare(childB.id)),
  }))

  return [...listedCategories, ...baseCategories]
}

function parseBaseApiCategories(luaClass) {
  const baseCategories = SECTIONS.map((section) => ({
    value: capitalize(section),
    id: section,
    children: luaClass[section]
      .map((member) => ({
        value: addFunctionTypeSymbol(member.name, member.function_type),
        id: member.name,
        children: [],
      }))
      .sort((childA, childB) => childA.id.localeCompare(childB.id)),
  }))

  return baseCategories
}

function parseApiCategories(luaClass, apiCategories) {
  if (typeof apiCategories[0] === "string") {
    // Handles simple apiCategories array assignment
    return parseSimpleApiCategories(luaClass, apiCategories)
  } else if (
    // Handles cases where classOrder is assigned via TOML tables
    apiCategories.some((category) => category.class === luaClass.name)
  ) {
    return parseSectionalApiCategories(luaClass, apiCategories)
  } else {
    // Handles where no apiCategory config is provided
    return parseBaseApiCategories(luaClass)
  }
}

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

    const nameSet = new Set()
    content.forEach((luaClass) => nameSet.add(luaClass.name))

    const classOrder = options.classOrder
    const apiCategories = options.apiCategories

    const allLuaClassNamesOrdered = parseClassOrder(
      content,
      classOrder,
      nameSet
    )

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
        apiCategories: apiCategories,
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

      const tocDataOrdered = parseApiCategories(luaClass, apiCategories)

      const tocData = await createData(
        `${luaClass.name}-toc.json`,
        JSON.stringify(tocDataOrdered)
      )

      console.log(`Adding path /api/${luaClass.name}`)

      addRoute({
        path: `${baseUrl}api/${luaClass.name}`,
        component: path.resolve(__dirname, "components/LuaClass.js"),
        modules: {
          luaClass: apiDataPath,
          allLuaClassNames,
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
