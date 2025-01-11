import useDocusaurusContext from "@docusaurus/useDocusaurusContext"
import rehypePrism from "@mapbox/rehype-prism"
import { defaultSchema } from "hast-util-sanitize"
import "prism-material-themes/themes/material-default.css"
import React, { useContext } from "react"
import format from "rehype-format"
import sanitize from "rehype-sanitize"
import html from "rehype-stringify"
import directives from "remark-directive"
import remarkGfm from "remark-gfm"
import remarkRehypeAdmonitions from "../remark/remarkRehypeAdmonitions.js"
import remarkExtendedLinkReferences from "../remark/remarkExtendedLinkReferences.js"
import parse from "remark-parse"
import remark2rehype from "remark-rehype"
import { unified } from "unified"
import { TypeLinksContext } from "./LuaClass.js"

const schema = {
  ...defaultSchema,
  tagNames: [...defaultSchema.tagNames, "svg", "path"],
  attributes: {
    ...defaultSchema.attributes,
    svg: ["xmlns", "width", "height", "viewBox"],
    path: ["fill-rule", "d", "fill"],
    "*": [...defaultSchema.attributes["*"], "className"],
  },
}

const linkTransformer = (baseUrl) => (node) => {
  if (node.children) {
    node.children.forEach(linkTransformer(baseUrl))
  }

  if (node.tagName === "a") {
    const url = node.properties.href

    if (url.startsWith("http")) {
      node.properties.target = "_blank"
    } else if (url.startsWith("/")) {
      node.properties.href = baseUrl + url.slice(1)
    }
  }
}

const autoLinkReferences = (typeLinks, baseUrl) => (node) => {
  const replaceLinkRefs = (node) => {
    if (node.type === "linkReference") {
      const label = node.label.replace(/(:|\.)/, "#")
      const name = label.replace(/#.*$/, "")
      const hashMatch = label.match(/#(.+)$/)

      if (name in typeLinks) {
        let link = typeLinks[name]

        if (link.startsWith(baseUrl)) {
          link = link.slice(baseUrl.length - 1)
        }

        node.type = "link"
        node.url = link + (hashMatch ? `#${hashMatch[1]}` : "")
        delete node.referenceType
      }
    }

    if (node.children) {
      node.children = node.children.map(replaceLinkRefs)
    }

    return node
  }

  node.children = node.children.map(replaceLinkRefs)
}

// Backwards compatibility for Docusaurus V2 Admonitions
function convertAdmonitions(content) {
  const blocksToConvert =
    /:::(\w+)(?:[ \t]+([^\[\]{}\n]+))?\n((?:[ \t]*\n?(?:(?!:::).)*\n?)+):::/gm

  return content.replace(blocksToConvert, (_, name, label, innerContent) => {
    label = label ? `[${label}]` : ""

    return `:::${name}${label}\n${innerContent}\n:::`
  })
}

export default function Markdown({ content, inline }) {
  const { siteConfig } = useDocusaurusContext()
  const typeLinks = useContext(TypeLinksContext)

  content = convertAdmonitions(content)

  const markdownHtml = unified()
    .use(parse)
    .use(remarkExtendedLinkReferences)
    .use(remarkGfm)
    .use(directives)
    .use(() => autoLinkReferences(typeLinks, siteConfig.baseUrl))
    .use(remark2rehype, {
      handlers: { ...remarkRehypeAdmonitions },
    })
    .use(() => linkTransformer(siteConfig.baseUrl))
    .use(rehypePrism)
    .use(format)
    .use(html)
    .use(sanitize, schema)
    .processSync(content)

  const Tag = inline ? "span" : "div"

  return <Tag dangerouslySetInnerHTML={{ __html: markdownHtml }} />
}
