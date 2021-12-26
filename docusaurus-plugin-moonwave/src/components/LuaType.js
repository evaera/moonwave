import React, { useContext } from "react"
import GenericLink from "./GenericLink"
import { TypeLinksContext } from "./LuaClass"
import styles from "./styles.module.css"
import { Op, PrOp } from "./Syntax"

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

  const readBalanced = (left, right) => {
    let buffer = ""

    let depth = 0
    while (peek()) {
      if (peek() === left) {
        depth++
      } else if (peek() === right) {
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

    if (position >= code.length) {
      break
    }

    if (peek() === "(") {
      next()
      tokens.push({
        type: "tuple",
        unseparatedTokens: tokenize(readBalanced("(", ")"), true),
      })
      next()
      continue
    }

    if (peek() === "[") {
      next()
      tokens.push({
        type: "indexer",
        tokens: tokenize(readBalanced("[", "]")),
      })
      next()
      continue
    }

    if (peek() === "{") {
      next()
      tokens.push({
        type: "table",
        unseparatedTokens: tokenize(readBalanced("{", "}"), true),
      })
      next()
      continue
    }

    if (isGroup && peek() === ",") {
      next()
      tokens.push({
        type: "separator",
      })
      continue
    }

    if (isPunc(peek())) {
      const punc = next()

      if (punc === "-" && peek() === ">") {
        tokens.push({
          type: "arrow",
        })
        next()
        continue
      }

      if (punc === "|") {
        tokens.push({ type: "union" })
        continue
      }

      tokens.push({
        type: "punc",
        punc,
      })
      continue
    }

    const atom = read((char) =>
      isGroup ? char !== "," && isAtom(char) : isAtom(char)
    )

    if (atom) {
      if (atom.endsWith(":")) {
        tokens.push({ type: "identifier", identifier: atom.slice(0, -1) })
      } else {
        tokens.push({
          type: "luaType",
          luaType: atom,
        })
      }
      continue
    }

    throw new Error(`Reached bottom of tokenizer with no match: ${peek()}`)
  }

  return tokens.map(separateGroups)
}

function separateGroups(token) {
  if (!token.unseparatedTokens) {
    return token
  }

  const separatedTokens = [[]]

  token.unseparatedTokens.forEach((token) => {
    if (token.type === "separator") {
      separatedTokens.push([])
    } else {
      token = separateGroups(token)

      separatedTokens[separatedTokens.length - 1].push(token)
    }
  })

  return {
    ...token,
    separatedTokens,
  }
}

function Group({ tokenGroups, depth, left, right }) {
  if (tokenGroups.length > 1) {
    return (
      <>
        <Op depth={depth}>{left}</Op>
        {tokenGroups.map((tokens, i) => (
          <div className={styles.inset} key={i}>
            <Tokens tokens={tokens} depth={depth} />
            {i !== tokenGroups.length - 1 && <Op depth={depth}>,</Op>}
          </div>
        ))}
        <Op depth={depth}>{right}</Op>
      </>
    )
  }

  return (
    <>
      <Op depth={depth}>{left}</Op>
      <Tokens tokens={tokenGroups[0]} depth={depth} />
      <Op depth={depth}>{right}</Op>
    </>
  )
}

function Tokens({ tokens, depth }) {
  return tokens.map((token, i) => <Token key={i} token={token} depth={depth} />)
}

function Token({ token, depth }) {
  const typeLinks = useContext(TypeLinksContext)

  switch (token.type) {
    case "root":
      return <Tokens tokens={token.tokens} depth={0} />
    case "tuple":
      return (
        <Group
          tokenGroups={token.separatedTokens}
          depth={depth + 1}
          left="("
          right=")"
        />
      )
    case "table":
      return (
        <Group
          tokenGroups={token.separatedTokens}
          depth={depth + 1}
          left="{"
          right="}"
        />
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
    case "indexer":
      return (
        <span>
          <PrOp>[</PrOp>
          <Tokens tokens={token.tokens} depth={depth + 1} />
          <PrOp>]</PrOp>
        </span>
      )
    case "luaType":
      const sanitizedToken = token.luaType.replace(/\W/g, "")
      if (sanitizedToken in typeLinks) {
        return (
          <code className={styles.blue}>
            <GenericLink
              to={typeLinks[sanitizedToken]}
              style={{ textDecoration: "underline", color: "inherit" }}
            >
              {token.luaType}
            </GenericLink>
          </code>
        )
      }

      return <code className={styles.blue}>{token.luaType}</code>
    default:
      return <span>unknown token {token.type}</span>
  }
}

export default function LuaType({ code }) {
  const tokens = tokenize(code)

  return <Token token={{ type: "root", tokens }} />
}
