import { codes } from "micromark-util-symbol"

function fromSyntax() {
  function afterTokenize(_, ok, notOk) {
    return (code) => {
      if (
        code === codes.colon ||
        code === codes.leftParenthesis ||
        code === codes.leftCurlyBrace ||
        code === codes.leftSquareBracket
      ) {
        return notOk(code)
      }

      return ok(code)
    }
  }

  function tokenize(effects, ok, notOk) {
    function start(code) {
      effects.enter("extendedLinkReference")
      effects.enter("extendedLinkReferenceMarker")
      effects.consume(code)
      effects.exit("extendedLinkReferenceMarker")
      effects.enter("extendedLinkReferenceLabel")
      return begin
    }

    function begin(code) {
      return code === codes.rightSquareBracket ? notOk(code) : inside(code)
    }

    function inside(code) {
      if (code === -5 || code === -4 || code === -3 || code === null) {
        return notOk(code)
      }

      if (code == codes.rightSquareBracket) {
        effects.exit("extendedLinkReferenceLabel")
        effects.enter("extendedLinkReferenceMarker")
        effects.consume(code)
        effects.exit("extendedLinkReferenceMarker")
        effects.exit("extendedLinkReference")

        return effects.check({ tokenize: afterTokenize }, ok, notOk)
      }

      effects.consume(code)

      return inside
    }

    return start
  }

  return {
    text: {
      [codes.leftSquareBracket]: {
        name: "extendedLinkReference",
        tokenize: tokenize,
      },
    },
  }
}

function fromMarkdown() {
  function enter(token) {
    this.enter(
      {
        type: "linkReference",
        referenceType: "shortcut",
      },
      token
    )
  }

  function exit(token) {
    this.exit(token)
  }

  function exitLabel(token) {
    const label = this.sliceSerialize(token)
    const node = this.stack[this.stack.length - 1]
    node.identifier = label
    node.label = label
    node.children = [{ type: "text", value: label }]
  }

  return {
    enter: {
      extendedLinkReference: enter,
    },
    exit: {
      extendedLinkReference: exit,
      extendedLinkReferenceLabel: exitLabel,
    },
  }
}

export default function remarkExtendedLinkReferences() {
  const data = this.data()

  const micromarkExtensions =
    data.micromarkExtensions || (data.micromarkExtensions = [])

  const fromMarkdownExtensions =
    data.fromMarkdownExtensions || (data.fromMarkdownExtensions = [])

  micromarkExtensions.push(fromSyntax())
  fromMarkdownExtensions.push(fromMarkdown())
}
