import React from "react"
import styles from "./styles.module.css"
import { Op } from "./Syntax"

const isWhitespace = (char) => !!char.match(/\s/)
const isAtom = (char) => char !== ")" && !isWhitespace(char)

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
    }

    if (isGroup && peek() === ",") {
      next()
      tokens.push({
        separator: true,
      })
    }

    const atom = read((char) =>
      isGroup ? char !== "," && isAtom(char) : isAtom(char)
    )

    if (atom) {
      if (atom === "->") {
        tokens.push({ arrow: true })
      } else if (atom.endsWith(":")) {
        tokens.push({ identifier: atom.slice(0, -1) })
      } else {
        tokens.push({
          luaType: atom,
        })
      }
    }
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
  switch (Object.keys(token)[0]) {
    case "root":
      return <Tokens tokens={token.root} depth={0} />
    case "tuple":
      return <Tuple tuple={token.tuple} depth={depth + 1} />
    case "identifier":
      return (
        <>
          <code>{token.identifier}</code>
          <Op>:</Op>&nbsp;
        </>
      )
    case "arrow":
      return <Op depth={depth + 1}>&nbsp;→&nbsp;</Op>
    case "luaType":
      return <code className={styles.blue}>{token.luaType}</code>
    default:
      return <span>unknown token {Object.keys(token)[0]}</span>
  }
}

export default function LuaType({ code }) {
  if (code.includes("(")) {
    const tokens = tokenize(code)

    return <Token token={{ root: tokens }} />
  }

  return <code className={styles.blue}>{code}</code>
}
