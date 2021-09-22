import Heading from "@theme/Heading"
import clsx from "clsx"
import React from "react"
import InlineDescription from "./InlineDescription"
import LuaType from "./LuaType"
import Markdown from "./Markdown"
import styles from "./styles.module.css"
import { PrOp } from "./Syntax"

const H3 = Heading("h3")

const Param = ({ name, lua_type }) => (
  <>
    <code>{name}:&nbsp;</code>
    <LuaType code={lua_type} />
  </>
)

export default function LuaFunction({
  luaClassName,
  name,
  params,
  returns,
  desc,
  function_type: functionType,
  errors,
}) {
  return (
    <>
      <div className={styles.memberString}>
        <code>
          {luaClassName}
          {functionType === "static" ? "." : ":"}
        </code>
        <code className={styles.green}>{name}</code>
        <PrOp>(</PrOp>
        {params.length < 2 && (!params[0] || !params[0].desc) ? (
          params[0] && <Param {...params[0]} />
        ) : (
          <>
            <div className={styles.inset}>
              {params.map((param, index) => (
                <div key={index}>
                  <Param {...param} />
                  {index !== params.length - 1 && <code>,</code>}
                  {param.desc && <InlineDescription content={param.desc} />}
                </div>
              ))}
            </div>
          </>
        )}
        <PrOp>) →&nbsp;</PrOp>
        {returns.length !== 1 && <PrOp>(</PrOp>}
        {returns.length === 1 ? (
          <>
            <LuaType code={returns[0].lua_type} />
            {returns[0].desc && <InlineDescription content={returns[0].desc} />}
          </>
        ) : (
          returns.length > 1 && (
            <div className={styles.inset}>
              {returns.map((ret, index) => (
                <div key={index}>
                  <LuaType code={ret.lua_type} />
                  {index !== returns.length - 1 && <code>,</code>}
                  {ret.desc && <InlineDescription content={ret.desc} />}
                </div>
              ))}
            </div>
          )
        )}
        {returns.length !== 1 && <PrOp>)</PrOp>}
      </div>
      <Markdown content={desc} />

      {errors && (
        <>
          <H3>Errors</H3>
          <table className={clsx(styles.errorTable)}>
            <thead>
              <tr>
                <th>Type</th>
                <th>Description</th>
              </tr>
            </thead>
            <tbody>
              {errors.map((error, i) => (
                <tr key={i}>
                  <td>{error.lua_type}</td>
                  <td>{error.desc}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </>
      )}
    </>
  )
}
