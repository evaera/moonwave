import { Redirect as RouterRedirect } from "@docusaurus/router"
import React from "react"

export default function Redirect({ allLuaClassNames, pluginOptions }) {
  return (
    <RouterRedirect to={`${pluginOptions.baseUrl}api/${allLuaClassNames[0]}`} />
  )
}
