import { codes } from "micromark-util-symbol"

const extendedLinkReferenceConstruct = {
  name: "extendedLinkReference",
  tokenize: extendedLinkReferenceTokenize,
}

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

function extendedLinkReferenceTokenize(effects, ok, notOk) {
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

export default function extendedLinkReference() {
  return {
    text: { [codes.leftSquareBracket]: extendedLinkReferenceConstruct },
  }
}
