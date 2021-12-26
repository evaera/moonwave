import { Redirect as RouterRedirect } from "@docusaurus/router"
import React from "react"

export default function Redirect({ sidebarClassNames, pluginOptions }) {
  if (sidebarClassNames.length > 0) {
    const firstLuaClassName = (
      sidebarClassNames[0].type === "link"
        ? sidebarClassNames[0].label
        : sidebarClassNames[0].items[0].label
    ).replace(/[\u200B]/g, "") // Strip out any extraneous 0-width spaces

    return (
      <RouterRedirect to={`${pluginOptions.baseUrl}api/${firstLuaClassName}`} />
    )
  } else {
    return <RouterRedirect to={`${pluginOptions.baseUrl}api/404`} />
  }
}
