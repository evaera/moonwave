import Link from "@docusaurus/Link"
import React from "react"

function isValidUrl(urlString) {
  const urlPattern = /https?:\/\//
  return urlPattern.test(urlString)
}

export default function GenericLink({ to, style, children }) {
  if (isValidUrl(to)) {
    return (
      <a style={style} href={to}>
        {children}
      </a>
    )
  }

  return (
    <Link style={style} to={to}>
      {children}
    </Link>
  )
}
