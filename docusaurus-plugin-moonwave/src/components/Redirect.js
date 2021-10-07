import { Redirect as RouterRedirect } from "@docusaurus/router"
import React from "react"

export default function Redirect({ allLuaClassNames, pluginOptions }) {
  const firstLuaClassName =
    allLuaClassNames[0].type === "link"
      ? allLuaClassNames[0].label
      : allLuaClassNames[0].items[0].label

  return (
    <RouterRedirect to={`${pluginOptions.baseUrl}api/${firstLuaClassName}`} />
  )
}
