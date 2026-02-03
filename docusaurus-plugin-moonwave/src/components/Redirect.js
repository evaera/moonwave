import { Redirect as RouterRedirect } from "@docusaurus/router"
import React from "react"

// Find the first valid link in a potentially nested sidebar structure
function findFirstValidLink(items) {
  for (let index = 0; index < items.length; index++) {
    const element = items[index]

    // Skip empty categories
    if (element.type === "category" && element.items.length === 0) {
      continue
    }

    // If it's a link, return the label (class name)
    if (element.type === "link") {
      return element.label.replace(/[\u200B]/g, "") // Strip out any zero-width spaces
    }

    // If it's a category, recursively look for the first link in its items
    if (element.type === "category") {
      const nestedLink = findFirstValidLink(element.items)
      if (nestedLink) {
        return nestedLink
      }
    }
  }

  return null // No valid link found
}

export default function Redirect({ sidebarClassNames, pluginOptions }) {
  const firstLuaClassName = findFirstValidLink(sidebarClassNames)

  if (firstLuaClassName) {
    return (
      <RouterRedirect to={`${pluginOptions.baseUrl}api/${firstLuaClassName}`} />
    )
  }

  // No valid link found, redirect to 404
  return <RouterRedirect to={`${pluginOptions.baseUrl}api/404`} />
}
