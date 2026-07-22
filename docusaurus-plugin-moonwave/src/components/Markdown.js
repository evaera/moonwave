import useDocusaurusContext from "@docusaurus/useDocusaurusContext"
import Mermaid from "@theme/Mermaid"
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
    /:::(\w+)(?:[ \t]+([^\[\]{}\n]+))?\n((?:(?!:::).\n?)*):::/gm

  return content.replace(blocksToConvert, (_, name, label, innerContent) => {
    label = label ? `[${label}]` : ""

    return `:::${name}${label}\n${innerContent}\n:::`
  })
}

// Splits content into alternating markdown and mermaid segments so that
// ```mermaid code fences can be rendered as diagrams instead of code blocks.
function splitMermaidBlocks(content) {
  const fence = /^[ \t]{0,3}```mermaid[ \t]*\n([\s\S]*?)^[ \t]{0,3}```[ \t]*$/gm

  const segments = []
  let lastIndex = 0
  let match

  while ((match = fence.exec(content))) {
    segments.push({
      type: "markdown",
      value: content.slice(lastIndex, match.index),
    })
    segments.push({ type: "mermaid", value: match[1] })
    lastIndex = match.index + match[0].length
  }

  segments.push({ type: "markdown", value: content.slice(lastIndex) })

  return segments.filter(
    (segment) => segment.type === "mermaid" || segment.value.trim().length > 0
  )
}

export default function Markdown({ content, inline }) {
  const { siteConfig } = useDocusaurusContext()
  const typeLinks = useContext(TypeLinksContext)

  content = convertAdmonitions(content)

  const renderMarkdown = (markdown) =>
    unified()
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
      .processSync(markdown)

  const Tag = inline ? "span" : "div"

  const segments = splitMermaidBlocks(content)

  if (!segments.some((segment) => segment.type === "mermaid")) {
    return <Tag dangerouslySetInnerHTML={{ __html: renderMarkdown(content) }} />
  }

  const mappedSegments = segments.map((segment, index) =>
    segment.type === "mermaid" ? (
      <Mermaid key={index} value={segment.value} />
    ) : (
      <Tag
        key={index}
        dangerouslySetInnerHTML={{ __html: renderMarkdown(segment.value) }}
      />
    )
  )

  return (<Tag>{mappedSegments}</Tag>)
}
