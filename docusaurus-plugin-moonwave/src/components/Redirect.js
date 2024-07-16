import { Redirect as RouterRedirect } from "@docusaurus/router"
import React from "react"

export default function Redirect({ sidebarClassNames, pluginOptions }) {
  for (let index = 0; index < sidebarClassNames.length; index++) {
    const element = sidebarClassNames[index]
    if (element.type == "category" && element.items.length === 0) {
      continue
    }

    const firstLuaClassName = (
      element.type === "link" ? element.label : element.items[0].label
    ).replace(/[\u200B]/g, "") // Strip out any extraneous 0-width spaces

    return (
      <RouterRedirect to={`${pluginOptions.baseUrl}api/${firstLuaClassName}`} />
    )
  }

  return <RouterRedirect to={`${pluginOptions.baseUrl}api/404`} />
}
