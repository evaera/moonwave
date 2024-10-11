import { visit, EXIT } from "unist-util-visit"
import { fromHtml } from "hast-util-from-html"
import { renderToString } from "react-dom/server"

import Admonition from "../components/Admonition"

/// remark-rehype handlers
const handlers = {
  containerDirective: directivesToHast,
  leafDirective: directivesToHast,
  inlineDirective: directivesToHast,
}

// Docusaurus style admonitions from remark directives
function directivesToHast(state, directive) {
  let children = directive.children || []
  let title = children[0]?.data?.directiveLabel
    ? children.shift()
    : undefined

  const admonition = renderToString(
    <Admonition
      variation={directive.name}
    />
  )

  const html = fromHtml(admonition)

  const transformContent = (content, parentClassName) => {
    visit(html, (node) => {
      if (node.properties?.className?.[0] === parentClassName) {
        node.children = state.all(content)
        return EXIT
      }
    })
  }

  if (title) { transformContent(title, "admonition-title") }
  transformContent(directive, "admonition-content")

  state.patch(directive, html)
  return state.applyData(directive, html)
}

export default handlers
