import { Redirect as RouterRedirect } from "@docusaurus/router"
import React from "react"

export default function Redirect({ allLuaClassNames }) {
  return <RouterRedirect to={`/api/${allLuaClassNames[0]}`} />
}
