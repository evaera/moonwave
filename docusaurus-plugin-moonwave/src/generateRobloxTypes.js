/*****
 * Original function built by evaera (https://github.com/evaera). Used in Moonwave with explicit permission from original author
 *
 * Author: evaera (https://github.com/evaera)
 * Project: Cmdr (https://github.com/evaera/Cmdr)
 * Copyright © 2018 Eryn L. K.
 *****/

const fetch = require("node-fetch")

const dataTypes = [
  "Axes",
  "BrickColor",
  "CFrame",
  "Color3",
  "ColorSequence",
  "ColorSequenceKeypoint",
  "DockWidgetPluginGuiInfo",
  "Enum",
  "EnumItem",
  "Enums",
  "Faces",
  "Instance",
  "NumberRange",
  "NumberSequence",
  "NumberSequenceKeypoint",
  "PathWaypoint",
  "PhysicalProperties",
  "Random",
  "Ray",
  "RBXScriptConnection",
  "RBXScriptSignal",
  "Rect",
  "Region3",
  "Region3int16",
  "TweenInfo",
  "UDim",
  "UDim2",
  "Vector2",
  "Vector2int16",
  "Vector3",
  "Vector3int16",
]

module.exports.generateRobloxTypes = async function generateRobloxTypes() {
  const req = await fetch(
    "https://raw.githubusercontent.com/CloneTrooper1019/Roblox-Client-Watch/roblox/API-Dump.json"
  )

  const api = await req.json()

  const types = api.Classes.map((c) => ({
    name: c.Name,
    link: `https://developer.roblox.com/en-us/api-reference/class/${c.Name}`,
  }))
    .concat(
      api.Enums.map((e) => ({
        name: `${e.Name}`,
        link: `https://developer.roblox.com/en-us/api-reference/enum/${e.Name}`,
      }))
    )
    .concat(
      dataTypes.map((t) => ({
        link: `https://developer.roblox.com/en-us/api-reference/datatype/${t}`,
        name: t,
      }))
    )
    .reduce((a, v) => {
      a[v.name] = v.link
      return a
    }, {})

  return types
}
