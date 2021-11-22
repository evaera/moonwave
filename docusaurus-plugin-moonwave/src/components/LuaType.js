import React, { useContext } from "react"
import GenericLink from "./GenericLink"
import { TypeLinksContext } from "./LuaClass"
import styles from "./styles.module.css"
import { Op } from "./Syntax"

const isPunc = (char) => !!char.match(/[\{\}<>\-\|]/)
const isWhitespace = (char) => !!char.match(/\s/)
const isAtom = (char) => !isWhitespace(char) && !isPunc(char)

function tokenize(code, isGroup) {
  let position = 0

  const next = () => code[position++]
  const peek = () => code[position]

  const read = (condition) => {
    let buffer = ""

    while (peek() && condition(peek())) {
      buffer += next()
    }

    return buffer
  }

  const readBalanced = () => {
    let buffer = ""

    let depth = 0
    while (peek()) {
      if (peek() === "(") {
        depth++
      } else if (peek() === ")") {
        if (depth === 0) {
          break
        } else {
          depth--
        }
      }

      buffer += next()
    }

    return buffer
  }

  const tokens = []

  while (position < code.length) {
    read(isWhitespace)

    if (peek() === "(") {
      next()
      tokens.push({
        tuple: tokenize(readBalanced(), true),
      })
      next()
      continue
    }

    if (isGroup && peek() === ",") {
      next()
      tokens.push({
        separator: true,
      })
      continue
    }

    if (isPunc(peek())) {
      const punc = next()

      if (punc === "-" && peek() === ">") {
        tokens.push({
          arrow: true,
        })
        next()
        continue
      }

      if (punc === "|") {
        tokens.push({ union: true })
        continue
      }

      tokens.push({
        punc,
      })
      continue
    }

    const atom = read((char) =>
      isGroup ? char !== "," && isAtom(char) : isAtom(char)
    )

    if (atom) {
      if (atom.endsWith(":")) {
        tokens.push({ identifier: atom.slice(0, -1) })
      } else {
        tokens.push({
          luaType: atom,
        })
      }
      continue
    }

    throw new Error(`Reached bottom of tokenizer with no match: ${peek()}`)
  }

  return groupTuples(tokens)
}

function groupTuples(tokens) {
  return tokens.map((token) => {
    if (!token.tuple) {
      return token
    }

    let subTokens = [[]]

    token.tuple.forEach((token) => {
      if (token.separator) {
        subTokens.push([])
      } else {
        if (token.tuple) {
          token = { tuple: groupTuples(token.tuple) }
        }
        subTokens[subTokens.length - 1].push(token)
      }
    })

    return {
      tuple: subTokens,
    }
  })
}

function Tuple({ tuple, depth }) {
  if (tuple.length > 1) {
    return (
      <>
        <Op depth={depth}>(</Op>
        {tuple.map((tokens, i) => (
          <div className={styles.inset} key={i}>
            <Tokens tokens={tokens} depth={depth} />
            {i !== tuple.length - 1 && <Op depth={depth}>,</Op>}
          </div>
        ))}
        <Op depth={depth}>)</Op>
      </>
    )
  }

  return (
    <>
      <Op depth={depth}>(</Op>
      <Tokens tokens={tuple[0]} depth={depth} />
      <Op depth={depth}>)</Op>
    </>
  )
}

function Tokens({ tokens, depth }) {
  return tokens.map((token, i) => <Token key={i} token={token} depth={depth} />)
}

function Token({ token, depth }) {
  const typeLinks = useContext(TypeLinksContext)

  switch (Object.keys(token)[0]) {
    case "root":
      return <Tokens tokens={token.root} depth={0} typeLinks={typeLinks} />
    case "tuple":
      return (
        <Tuple tuple={token.tuple} depth={depth + 1} typeLinks={typeLinks} />
      )
    case "identifier":
      return (
        <>
          <code>{token.identifier}:&nbsp;</code>
        </>
      )
    case "arrow":
      return <Op depth={depth + 1}>&nbsp;→&nbsp;</Op>
    case "punc":
      return <Op>{token.punc}</Op>
    case "union":
      return <Op>&nbsp;|&nbsp;</Op>
    case "luaType":
      const sanitizedToken = token.luaType.replace(/\W/g, "")
      if (typeLinks.has(sanitizedToken)) {
        return (
          <code className={styles.blue}>
            <GenericLink
              to={typeLinks.get(sanitizedToken)}
              style={{ textDecoration: "underline", color: "inherit" }}
            >
              {token.luaType}
            </GenericLink>
          </code>
        )
      }

      return <code className={styles.blue}>{token.luaType}</code>
    default:
      return <span>unknown token {Object.keys(token)[0]}</span>
  }
}

export default function LuaType({ code }) {
  const tokens = tokenize(code)

  return <Token token={{ root: tokens }} />
}
