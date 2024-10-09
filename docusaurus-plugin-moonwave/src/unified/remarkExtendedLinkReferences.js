import syntax from "./extended-link-references/syntax"
import fromMarkdown from "./extended-link-references/fromMarkdown"

export default function remarkExtendedLinkReferences() {
  const data = this.data()

  const micromarkExtensions =
    data.micromarkExtensions || (data.micromarkExtensions = [])

  const fromMarkdownExtensions =
    data.fromMarkdownExtensions || (data.fromMarkdownExtensions = [])

  micromarkExtensions.push(syntax())
  fromMarkdownExtensions.push(fromMarkdown())
}
